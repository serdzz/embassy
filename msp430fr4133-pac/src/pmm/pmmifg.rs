#[doc = "Register `PMMIFG` reader"]
pub type R = crate::R<PmmifgSpec>;
#[doc = "Register `PMMIFG` writer"]
pub type W = crate::W<PmmifgSpec>;
#[doc = "Field `PMMBORIFG` reader - PMM Software BOR interrupt flag"]
pub type PmmborifgR = crate::BitReader;
#[doc = "Field `PMMBORIFG` writer - PMM Software BOR interrupt flag"]
pub type PmmborifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMRSTIFG` reader - PMM RESET pin interrupt flag"]
pub type PmmrstifgR = crate::BitReader;
#[doc = "Field `PMMRSTIFG` writer - PMM RESET pin interrupt flag"]
pub type PmmrstifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMPORIFG` reader - PMM Software POR interrupt flag"]
pub type PmmporifgR = crate::BitReader;
#[doc = "Field `PMMPORIFG` writer - PMM Software POR interrupt flag"]
pub type PmmporifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSHIFG` reader - SVS low side interrupt flag"]
pub type SvshifgR = crate::BitReader;
#[doc = "Field `SVSHIFG` writer - SVS low side interrupt flag"]
pub type SvshifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMLPM5IFG` reader - LPM5 indication Flag"]
pub type Pmmlpm5ifgR = crate::BitReader;
#[doc = "Field `PMMLPM5IFG` writer - LPM5 indication Flag"]
pub type Pmmlpm5ifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&self) -> PmmborifgR {
        PmmborifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&self) -> PmmrstifgR {
        PmmrstifgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&self) -> PmmporifgR {
        PmmporifgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&self) -> SvshifgR {
        SvshifgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&self) -> Pmmlpm5ifgR {
        Pmmlpm5ifgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&mut self) -> PmmborifgW<'_, PmmifgSpec> {
        PmmborifgW::new(self, 8)
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&mut self) -> PmmrstifgW<'_, PmmifgSpec> {
        PmmrstifgW::new(self, 9)
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&mut self) -> PmmporifgW<'_, PmmifgSpec> {
        PmmporifgW::new(self, 10)
    }
    #[doc = "Bit 13 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&mut self) -> SvshifgW<'_, PmmifgSpec> {
        SvshifgW::new(self, 13)
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&mut self) -> Pmmlpm5ifgW<'_, PmmifgSpec> {
        Pmmlpm5ifgW::new(self, 15)
    }
}
#[doc = "PMM Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmmifgSpec;
impl crate::RegisterSpec for PmmifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmifg::R`](R) reader structure"]
impl crate::Readable for PmmifgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmmifg::W`](W) writer structure"]
impl crate::Writable for PmmifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMIFG to value 0"]
impl crate::Resettable for PmmifgSpec {}
