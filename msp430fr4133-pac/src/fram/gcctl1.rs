#[doc = "Register `GCCTL1` reader"]
pub type R = crate::R<Gcctl1Spec>;
#[doc = "Register `GCCTL1` writer"]
pub type W = crate::W<Gcctl1Spec>;
#[doc = "Field `CBDIFG` reader - FRAM correctable bit error flag"]
pub type CbdifgR = crate::BitReader;
#[doc = "Field `CBDIFG` writer - FRAM correctable bit error flag"]
pub type CbdifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UBDIFG` reader - FRAM uncorrectable bit error flag"]
pub type UbdifgR = crate::BitReader;
#[doc = "Field `UBDIFG` writer - FRAM uncorrectable bit error flag"]
pub type UbdifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCTEIFG` reader - Access time error flag"]
pub type AccteifgR = crate::BitReader;
#[doc = "Field `ACCTEIFG` writer - Access time error flag"]
pub type AccteifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CbdifgR {
        CbdifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UbdifgR {
        UbdifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> AccteifgR {
        AccteifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&mut self) -> CbdifgW<'_, Gcctl1Spec> {
        CbdifgW::new(self, 1)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&mut self) -> UbdifgW<'_, Gcctl1Spec> {
        UbdifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&mut self) -> AccteifgW<'_, Gcctl1Spec> {
        AccteifgW::new(self, 3)
    }
}
#[doc = "General Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcctl1Spec;
impl crate::RegisterSpec for Gcctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gcctl1::R`](R) reader structure"]
impl crate::Readable for Gcctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`gcctl1::W`](W) writer structure"]
impl crate::Writable for Gcctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCCTL1 to value 0"]
impl crate::Resettable for Gcctl1Spec {}
