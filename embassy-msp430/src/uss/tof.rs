//! Turning two captured waveforms into a time-of-flight difference.
//!
//! This is the arithmetic, not the driver. [`super::Uss`] gives you what the receiving transducer
//! heard in each direction; what follows works out how much later one arrived than the other, which
//! is the quantity a flow meter is actually measuring.
//!
//! # How much to trust it
//!
//! The interesting number is tiny. Water flowing at a metre a second down a fifty-millimetre path
//! shifts the flight time by something like a nanosecond, against a flight of thirty-odd
//! microseconds — five parts in a hundred thousand. Sampling at four megasamples a second, one
//! sample is 250 nanoseconds, so the whole signal of interest is a fraction of a sample and every
//! bit of it comes from interpolating between them.
//!
//! That is why this is honest about being an estimator. It finds the correlation peak between the
//! two captures and fits a parabola through the three points around it, which is a standard and
//! reasonable thing to do — and it is not what TI's library does, does not track the envelope
//! across temperature, and has no answer at all for zero-flow drift, which is the error that
//! decides whether a meter is billable. Expect it to show you flow. Do not expect it to be right
//! in the third digit.
//!
//! # What it costs
//!
//! A correlation over a few hundred samples and a few dozen lags is tens of thousands of
//! multiply-accumulates. This device has an `LEA` accelerator built precisely for that and no
//! driver for it here, so this runs on the CPU — and if the linker was given the software multiply
//! routines, every one of those is a subroutine call. Budget tens of milliseconds, and measure
//! before putting it anywhere that has to be quick.

use super::Config;

/// What one pair of captures came to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Measurement {
    /// How much later the second waveform arrived than the first, in picoseconds. Signed: negative
    /// means it arrived earlier.
    pub delta_t_ps: i32,
    /// Where the burst was judged to start in the first capture, as a sample index.
    pub start_first: usize,
    /// The same for the second capture.
    pub start_second: usize,
    /// The correlation peak's height, as a rough measure of how alike the two waveforms were.
    ///
    /// A collapsing value across a run of measurements usually means the signal is being lost —
    /// air in the pipe, a fouled transducer — long before the answer starts looking wrong.
    pub quality: i32,
}

/// The first sample whose magnitude passes `threshold`.
///
/// Deliberately crude. It is used to line the two captures up to within a sample or two before the
/// correlation runs, so that the correlation only has to search a small range of lags; the accuracy
/// comes from the correlation, not from here.
pub fn burst_start(samples: &[i16], threshold: i16) -> Option<usize> {
    samples
        .iter()
        .position(|&s| s.unsigned_abs() >= threshold.unsigned_abs())
}

/// Correlation of `a` against `b` shifted by `lag` samples.
///
/// Only the overlapping region is summed, and the result is scaled down so that a few hundred
/// products of two 14-bit samples cannot overflow the accumulator.
fn correlate(a: &[i16], b: &[i16], lag: i16) -> i32 {
    let (a_start, b_start) = if lag >= 0 {
        (lag as usize, 0usize)
    } else {
        (0usize, (-lag) as usize)
    };
    if a_start >= a.len() || b_start >= b.len() {
        return 0;
    }

    let n = (a.len() - a_start).min(b.len() - b_start);
    let mut sum: i32 = 0;
    for i in 0..n {
        // >> 6 keeps a 14-bit product inside i32 for the several hundred terms this sums, without
        // losing so much that the parabola through the peak goes flat.
        sum += ((a[a_start + i] as i32) * (b[b_start + i] as i32)) >> 6;
    }
    sum
}

/// Sub-sample offset of the peak, in 1/256ths of a sample.
///
/// A parabola through the peak and its two neighbours; its vertex is where the true peak would be
/// if the waveform were smooth, which for a resonant transducer it very nearly is. Fixed point
/// throughout — this CPU has no floating-point unit, and the fractions involved are small enough
/// that a float would be doing the same arithmetic more slowly.
fn interpolate_peak(left: i32, centre: i32, right: i32) -> i32 {
    let denominator = 2 * centre - left - right;
    if denominator == 0 {
        return 0;
    }
    // (left - right) / (2 * denominator), scaled by 256. The shift happens first so the numerator
    // keeps its resolution.
    (((left - right) as i64 * 128) / denominator as i64) as i32
}

/// Find how far `second` lags `first`, searching `±max_lag` samples.
///
/// Returns the lag in 1/256ths of a sample, and the peak height.
pub fn lag_q8(first: &[i16], second: &[i16], max_lag: i16) -> Option<(i32, i32)> {
    if first.is_empty() || second.is_empty() || max_lag <= 0 {
        return None;
    }

    let mut best_lag = 0i16;
    let mut best = i32::MIN;
    for lag in -max_lag..=max_lag {
        let value = correlate(first, second, lag);
        if value > best {
            best = value;
            best_lag = lag;
        }
    }

    // A peak at the edge of the search means the true peak is outside it, and the parabola through
    // it would be fitted to the wrong three points. Better to say so than to return a number that
    // looks like an answer.
    if best_lag == -max_lag || best_lag == max_lag {
        return None;
    }

    let left = correlate(first, second, best_lag - 1);
    let right = correlate(first, second, best_lag + 1);
    let fraction = interpolate_peak(left, best, right);

    Some((best_lag as i32 * 256 + fraction, best))
}

/// Work out the difference in flight time between two captures.
///
/// `threshold` is the magnitude that counts as the burst having arrived; it wants setting from what
/// the receiving transducer actually produces on the board in question, somewhere well above the
/// noise and well below the peak. `max_lag` bounds the search, and should be a little more than the
/// largest shift the plumbing can produce.
///
/// `window` is how many samples after the burst to correlate, or zero for all of them. It is worth
/// setting: the correlation is the only expensive arithmetic in a measurement, its cost is the
/// window times the number of lags, and the samples worth correlating are the first few cycles of
/// the burst where the signal is strongest. The tail adds work and noise in equal measure.
///
/// Returns `None` when the burst was not found in one of the captures, or when the correlation peak
/// ran to the edge of the search — both of which mean there is no answer here rather than a poor
/// one.
pub fn analyse(
    up: &[i16],
    down: &[i16],
    config: &Config,
    threshold: i16,
    max_lag: i16,
    window: usize,
) -> Option<Measurement> {
    let start_first = burst_start(up, threshold)?;
    let start_second = burst_start(down, threshold)?;

    // Correlate from the burst onwards rather than over the whole capture: the quiet part before it
    // is noise, and including it only dilutes the peak. The window is measured from the burst, not
    // from the start of the capture — a window from the start could easily end before the burst
    // begins.
    let first = &up[start_first..];
    let second = &down[start_second..];
    let span = if window == 0 {
        first.len().min(second.len())
    } else {
        window.min(first.len()).min(second.len())
    };
    let (lag, quality) = lag_q8(&first[..span], &second[..span], max_lag)?;

    // The coarse alignment from the threshold has to go back in — the correlation measured the
    // residual shift after both captures were moved to their own burst.
    let coarse = (start_second as i32 - start_first as i32) * 256;
    let total_q8 = coarse + lag;

    Some(Measurement {
        delta_t_ps: ((total_q8 as i64 * config.sample_period_ps() as i64) / 256) as i32,
        start_first,
        start_second,
        quality,
    })
}

/// Flight time from the start of the capture to the burst, in nanoseconds.
///
/// Only as good as the threshold that found the burst, so it is a coarse number — useful for
/// sanity, for spotting an empty pipe, and as the `t_up · t_down` term in the flow equation, where
/// a per-cent error hardly matters. It is not the precise quantity; that is
/// [`Measurement::delta_t_ps`].
pub fn flight_time_ns(start: usize, config: &Config) -> u32 {
    ((start as u64 * config.sample_period_ps() as u64) / 1000) as u32
}
