#[doc = "Register `GCCTL0` reader"]
pub type R = crate::R<Gcctl0Spec>;
#[doc = "Register `GCCTL0` writer"]
pub type W = crate::W<Gcctl0Spec>;
#[doc = "Field `FRLPMPWR` reader - FRAM Enable FRAM auto power up after LPM"]
pub type FrlpmpwrR = crate::BitReader;
#[doc = "Field `FRLPMPWR` writer - FRAM Enable FRAM auto power up after LPM"]
pub type FrlpmpwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRPWR` reader - FRAM Power Control"]
pub type FrpwrR = crate::BitReader;
#[doc = "Field `FRPWR` writer - FRAM Power Control"]
pub type FrpwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCTEIE` reader - RESERVED"]
pub type AccteieR = crate::BitReader;
#[doc = "Field `ACCTEIE` writer - RESERVED"]
pub type AccteieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBDIE` reader - Enable NMI event if correctable bit error detected"]
pub type CbdieR = crate::BitReader;
#[doc = "Field `CBDIE` writer - Enable NMI event if correctable bit error detected"]
pub type CbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UBDIE` reader - Enable NMI event if uncorrectable bit error detected"]
pub type UbdieR = crate::BitReader;
#[doc = "Field `UBDIE` writer - Enable NMI event if uncorrectable bit error detected"]
pub type UbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UBDRSTEN` reader - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub type UbdrstenR = crate::BitReader;
#[doc = "Field `UBDRSTEN` writer - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub type UbdrstenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FRAM Enable FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&self) -> FrlpmpwrR {
        FrlpmpwrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM Power Control"]
    #[inline(always)]
    pub fn frpwr(&self) -> FrpwrR {
        FrpwrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RESERVED"]
    #[inline(always)]
    pub fn accteie(&self) -> AccteieR {
        AccteieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable NMI event if correctable bit error detected"]
    #[inline(always)]
    pub fn cbdie(&self) -> CbdieR {
        CbdieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable NMI event if uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdie(&self) -> UbdieR {
        UbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdrsten(&self) -> UbdrstenR {
        UbdrstenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM Enable FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&mut self) -> FrlpmpwrW<'_, Gcctl0Spec> {
        FrlpmpwrW::new(self, 1)
    }
    #[doc = "Bit 2 - FRAM Power Control"]
    #[inline(always)]
    pub fn frpwr(&mut self) -> FrpwrW<'_, Gcctl0Spec> {
        FrpwrW::new(self, 2)
    }
    #[doc = "Bit 3 - RESERVED"]
    #[inline(always)]
    pub fn accteie(&mut self) -> AccteieW<'_, Gcctl0Spec> {
        AccteieW::new(self, 3)
    }
    #[doc = "Bit 5 - Enable NMI event if correctable bit error detected"]
    #[inline(always)]
    pub fn cbdie(&mut self) -> CbdieW<'_, Gcctl0Spec> {
        CbdieW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable NMI event if uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdie(&mut self) -> UbdieW<'_, Gcctl0Spec> {
        UbdieW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdrsten(&mut self) -> UbdrstenW<'_, Gcctl0Spec> {
        UbdrstenW::new(self, 7)
    }
}
#[doc = "General Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcctl0Spec;
impl crate::RegisterSpec for Gcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gcctl0::R`](R) reader structure"]
impl crate::Readable for Gcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`gcctl0::W`](W) writer structure"]
impl crate::Writable for Gcctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCCTL0 to value 0"]
impl crate::Resettable for Gcctl0Spec {}
