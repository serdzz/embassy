#[doc = "Register `IFG1` reader"]
pub type R = crate::R<Ifg1Spec>;
#[doc = "Register `IFG1` writer"]
pub type W = crate::W<Ifg1Spec>;
#[doc = "Field `WDTIFG` reader - Watchdog Interrupt Flag"]
pub type WdtifgR = crate::BitReader;
#[doc = "Field `WDTIFG` writer - Watchdog Interrupt Flag"]
pub type WdtifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIFG` reader - Osc. Fault Interrupt Flag"]
pub type OfifgR = crate::BitReader;
#[doc = "Field `OFIFG` writer - Osc. Fault Interrupt Flag"]
pub type OfifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORIFG` reader - Power On Interrupt Flag"]
pub type PorifgR = crate::BitReader;
#[doc = "Field `PORIFG` writer - Power On Interrupt Flag"]
pub type PorifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIFG` reader - Reset Interrupt Flag"]
pub type RstifgR = crate::BitReader;
#[doc = "Field `RSTIFG` writer - Reset Interrupt Flag"]
pub type RstifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub type NmiifgR = crate::BitReader;
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub type NmiifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WdtifgR {
        WdtifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OfifgR {
        OfifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&self) -> PorifgR {
        PorifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&self) -> RstifgR {
        RstifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NmiifgR {
        NmiifgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WdtifgW<'_, Ifg1Spec> {
        WdtifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OfifgW<'_, Ifg1Spec> {
        OfifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&mut self) -> PorifgW<'_, Ifg1Spec> {
        PorifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&mut self) -> RstifgW<'_, Ifg1Spec> {
        RstifgW::new(self, 3)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NmiifgW<'_, Ifg1Spec> {
        NmiifgW::new(self, 4)
    }
}
#[doc = "Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifg1Spec;
impl crate::RegisterSpec for Ifg1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifg1::R`](R) reader structure"]
impl crate::Readable for Ifg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ifg1::W`](W) writer structure"]
impl crate::Writable for Ifg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFG1 to value 0"]
impl crate::Resettable for Ifg1Spec {}
