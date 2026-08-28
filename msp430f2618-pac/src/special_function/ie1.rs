#[doc = "Register `IE1` reader"]
pub type R = crate::R<Ie1Spec>;
#[doc = "Register `IE1` writer"]
pub type W = crate::W<Ie1Spec>;
#[doc = "Field `WDTIE` reader - Watchdog Interrupt Enable"]
pub type WdtieR = crate::BitReader;
#[doc = "Field `WDTIE` writer - Watchdog Interrupt Enable"]
pub type WdtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIE` reader - Osc. Fault Interrupt Enable"]
pub type OfieR = crate::BitReader;
#[doc = "Field `OFIE` writer - Osc. Fault Interrupt Enable"]
pub type OfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub type NmiieR = crate::BitReader;
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub type NmiieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCVIE` reader - Flash Access Violation Interrupt Enable"]
pub type AccvieR = crate::BitReader;
#[doc = "Field `ACCVIE` writer - Flash Access Violation Interrupt Enable"]
pub type AccvieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WdtieR {
        WdtieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OfieR {
        OfieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NmiieR {
        NmiieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> AccvieR {
        AccvieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WdtieW<'_, Ie1Spec> {
        WdtieW::new(self, 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OfieW<'_, Ie1Spec> {
        OfieW::new(self, 1)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NmiieW<'_, Ie1Spec> {
        NmiieW::new(self, 4)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&mut self) -> AccvieW<'_, Ie1Spec> {
        AccvieW::new(self, 5)
    }
}
#[doc = "Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ie1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ie1Spec;
impl crate::RegisterSpec for Ie1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ie1::R`](R) reader structure"]
impl crate::Readable for Ie1Spec {}
#[doc = "`write(|w| ..)` method takes [`ie1::W`](W) writer structure"]
impl crate::Writable for Ie1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IE1 to value 0"]
impl crate::Resettable for Ie1Spec {}
