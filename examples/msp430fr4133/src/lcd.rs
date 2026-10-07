//! Driver for the FH-1138P segmented LCD on the MSP-EXP430FR4133 LaunchPad.
//!
//! Ported from TI's BSD-licensed `LCD_Launchpad.cpp` (Energia), which already carries the
//! segment memory map worked out against the FR4133 User's Guide (SLAU595, Table 3 and 4): the
//! `LCDMEM` position for each of the six alphanumeric digits, and the on/off bit pattern for
//! digits 0-9 and uppercase A-Z on a 4-COM, 2-byte-per-character layout.
//!
//! This only drives the six alphanumeric characters — the extra top-row symbols (battery, antenna,
//! heart, degree, RX/TX, …) are a separate `showSymbol`-style API this port does not need yet.
#![allow(dead_code)]

use embassy_msp430::pac::LcdE;

/// Number of alphanumeric character positions on the display.
pub const NUM_CHARS: usize = 6;

/// `LCDMEM` byte offset where each character's two-byte segment pattern starts.
/// From `LCD_Launchpad.cpp`'s FR4133 (`__MSP430_HAS_LCD_E__`) table: position 1 begins at S8,
/// and so on by two LCDMEM bytes (one display mux) per segment line.
const DIGIT_LOC: [usize; NUM_CHARS] = [4, 6, 8, 10, 2, 18];

/// Segment pattern (low byte, high byte) for digits 0-9, indexed by digit value.
const DIGIT: [[u8; 2]; 10] = [
    [0xFC, 0x28], // 0
    [0x60, 0x20], // 1
    [0xDB, 0x00], // 2
    [0xF3, 0x00], // 3
    [0x67, 0x00], // 4
    [0xB7, 0x00], // 5
    [0xBF, 0x00], // 6
    [0xE4, 0x00], // 7
    [0xFF, 0x00], // 8
    [0xF7, 0x00], // 9
];

/// Segment pattern (low byte, high byte) for uppercase A-Z, indexed by `c - b'A'`.
const ALPHABET: [[u8; 2]; 26] = [
    [0xEF, 0x00], // A
    [0xF1, 0x50], // B
    [0x9C, 0x00], // C
    [0xF0, 0x50], // D
    [0x9F, 0x00], // E
    [0x8F, 0x00], // F
    [0xBD, 0x00], // G
    [0x6F, 0x00], // H
    [0x90, 0x50], // I
    [0x78, 0x00], // J
    [0x0E, 0x22], // K
    [0x1C, 0x00], // L
    [0x6C, 0xA0], // M
    [0x6C, 0x82], // N
    [0xFC, 0x00], // O
    [0xCF, 0x00], // P
    [0xFC, 0x02], // Q
    [0xCF, 0x02], // R
    [0xB7, 0x00], // S
    [0x80, 0x50], // T
    [0x7C, 0x00], // U
    [0x0C, 0x28], // V
    [0x6C, 0x0A], // W
    [0x00, 0xAA], // X
    [0x00, 0xB0], // Y
    [0x90, 0x28], // Z
];

/// Mask of bits in the *second* LCDMEM byte of a character that the symbol row shares with it —
/// writing a digit/letter must not clobber these.
const SYMBOL_MASK: u8 = 0b0000_0101;

/// The on-board 6-character alphanumeric LCD.
pub struct Lcd {
    regs: LcdE,
}

impl Lcd {
    /// Bring the LCD_E controller up: mux rate, bias generator/charge pump, and the four L0..L3
    /// pins as COM0..COM3 (the layout used by every FR4133 LaunchPad, per SLAU595 Table 4).
    ///
    /// # Safety
    /// Must be called at most once; takes exclusive ownership of `LCD_E` via `steal()`.
    pub fn new() -> Self {
        let regs = unsafe { LcdE::steal() };

        regs.lcdctl0().write(|w| w.lcdon().clear_bit());

        // Every L pin on the FR4133 can be a SEG or a COM; the LaunchPad wires all of them to
        // the display, so enable every segment pin (LCDS0..LCDS39 across the three PCTL regs).
        regs.lcdpctl0().write(|w| unsafe { w.bits(0xFFFF) });
        regs.lcdpctl1().write(|w| unsafe { w.bits(0x07FF) });
        regs.lcdpctl2().write(|w| unsafe { w.bits(0x00F0) });

        // L0-L3 run off ACLK (LCDSSEL = 0 on each), the rest default to segment-only.
        regs.lcdcssel0().write(|w| unsafe { w.bits(0x000F) });

        // 4-mux (LCDMX1:0), ACLK source, low-power waveform, segments on, divider /3.
        regs.lcdctl0().write(|w| {
            w.lcdmx0()
                .set_bit()
                .lcdmx1()
                .set_bit()
                .lcdssel()
                .lcdssel_0()
                .lcdlp()
                .set_bit()
                .lcdson()
                .set_bit()
                .lcddiv()
                .lcddiv_2()
        });

        // Internal 3.02 V reference, charge pump enabled, 256 Hz charge-pump clock.
        regs.lcdvctl().write(|w| {
            w.lcdrefen()
                .set_bit()
                .lcdcpen()
                .set_bit()
                .vlcd()
                .vlcd_6()
                .lcdcpfsel0()
                .set_bit()
                .lcdcpfsel1()
                .set_bit()
                .lcdcpfsel2()
                .set_bit()
                .lcdcpfsel3()
                .set_bit()
        });

        // Clear main and blinking memory, then select main memory for display.
        regs.lcdmemctl().write(|w| w.lcdclrm().set_bit().lcdclrbm().set_bit());
        regs.lcdm0w().write(|w| unsafe { w.bits(0x8421) });
        regs.lcdbm0w().write(|w| unsafe { w.bits(0x8421) });
        regs.lcdmemctl().write(|w| w.lcddisp().clear_bit());

        // No blinking.
        regs.lcdblkctl()
            .write(|w| w.lcdblkpre().lcdblkpre_2().lcdblkmod().lcdblkmod_0());

        regs.lcdctl0().write(|w| {
            w.lcdmx0()
                .set_bit()
                .lcdmx1()
                .set_bit()
                .lcdssel()
                .lcdssel_0()
                .lcdlp()
                .set_bit()
                .lcdson()
                .set_bit()
                .lcddiv()
                .lcddiv_2()
                .lcdon()
                .set_bit()
        });

        Self { regs }
    }

    /// Clear every character and the blinking memory.
    pub fn clear(&mut self) {
        self.regs.lcdmemctl().write(|w| w.lcdclrm().set_bit().lcdclrbm().set_bit());
    }

    /// Show one character (space, `0`-`9`, or `A`-`Z`/`a`-`z`, folded to uppercase) at `position`
    /// (0-5, left to right). Anything else lights every segment, same as the Energia driver, so a
    /// mistake is obvious on the display rather than silently blank.
    pub fn show_char(&mut self, c: char, position: usize) {
        let base = DIGIT_LOC[position];
        let (low, high) = match c {
            ' ' => (0u8, 0u8),
            '0'..='9' => {
                let d = DIGIT[(c as u8 - b'0') as usize];
                (d[0], d[1])
            }
            'A'..='Z' => {
                let d = ALPHABET[(c as u8 - b'A') as usize];
                (d[0], d[1])
            }
            'a'..='z' => {
                let d = ALPHABET[(c as u8 - b'a') as usize];
                (d[0], d[1])
            }
            _ => (0xFF, 0xFF & !SYMBOL_MASK),
        };
        self.write_mem(base, low);
        self.write_mem(base + 1, high | (self.read_mem(base + 1) & SYMBOL_MASK));
    }

    /// Show a string left to right, space-padding or truncating to [`NUM_CHARS`].
    pub fn show_text(&mut self, s: &str) {
        let mut chars = s.chars();
        for position in 0..NUM_CHARS {
            self.show_char(chars.next().unwrap_or(' '), position);
        }
    }

    /// Show a `NUM_CHARS`-wide window of `text` starting at `offset` (characters, not bytes;
    /// `text` should be ASCII). Positions past the end of `text` show as spaces — this is the
    /// building block for scrolling: the caller only needs to walk `offset` up or down over time.
    pub fn show_window(&mut self, text: &str, offset: usize) {
        for position in 0..NUM_CHARS {
            let c = text.chars().nth(offset + position).unwrap_or(' ');
            self.show_char(c, position);
        }
    }

    /// `LCDMEM` is byte-addressed but the PAC only exposes it as 16-bit `LCDMxW` word registers
    /// (`LCDM0W` covers bytes 0/1, `LCDM2W` covers 2/3, ...). Reconstruct byte access on top: read
    /// the containing word, and keep the other byte unchanged on write.
    fn write_mem(&mut self, byte_offset: usize, value: u8) {
        let word = self.read_word(byte_offset / 2);
        let new_word = if byte_offset % 2 == 0 {
            (word & 0xFF00) | value as u16
        } else {
            (word & 0x00FF) | ((value as u16) << 8)
        };
        self.write_word(byte_offset / 2, new_word);
    }

    fn read_mem(&self, byte_offset: usize) -> u8 {
        let word = self.read_word(byte_offset / 2);
        if byte_offset % 2 == 0 {
            (word & 0xFF) as u8
        } else {
            (word >> 8) as u8
        }
    }

    fn read_word(&self, index: usize) -> u16 {
        match index {
            0 => self.regs.lcdm0w().read().bits(),
            1 => self.regs.lcdm2w().read().bits(),
            2 => self.regs.lcdm4w().read().bits(),
            3 => self.regs.lcdm6w().read().bits(),
            4 => self.regs.lcdm8w().read().bits(),
            5 => self.regs.lcdm10w().read().bits(),
            6 => self.regs.lcdm12w().read().bits(),
            7 => self.regs.lcdm14w().read().bits(),
            8 => self.regs.lcdm16w().read().bits(),
            9 => self.regs.lcdm18w().read().bits(),
            _ => unreachable!("only LCDMEM bytes 0..19 are used by the six characters"),
        }
    }

    fn write_word(&mut self, index: usize, value: u16) {
        match index {
            0 => {
                self.regs.lcdm0w().write(|w| unsafe { w.bits(value) });
            }
            1 => {
                self.regs.lcdm2w().write(|w| unsafe { w.bits(value) });
            }
            2 => {
                self.regs.lcdm4w().write(|w| unsafe { w.bits(value) });
            }
            3 => {
                self.regs.lcdm6w().write(|w| unsafe { w.bits(value) });
            }
            4 => {
                self.regs.lcdm8w().write(|w| unsafe { w.bits(value) });
            }
            5 => {
                self.regs.lcdm10w().write(|w| unsafe { w.bits(value) });
            }
            6 => {
                self.regs.lcdm12w().write(|w| unsafe { w.bits(value) });
            }
            7 => {
                self.regs.lcdm14w().write(|w| unsafe { w.bits(value) });
            }
            8 => {
                self.regs.lcdm16w().write(|w| unsafe { w.bits(value) });
            }
            9 => {
                self.regs.lcdm18w().write(|w| unsafe { w.bits(value) });
            }
            _ => unreachable!("only LCDMEM bytes 0..19 are used by the six characters"),
        }
    }
}
