#[doc = "Register `SFRIE1` reader"]
pub type R = crate::R<Sfrie1Spec>;
#[doc = "Register `SFRIE1` writer"]
pub type W = crate::W<Sfrie1Spec>;
#[doc = "Field `WDTIE` reader - WDT Interrupt Enable"]
pub type WdtieR = crate::BitReader;
#[doc = "Field `WDTIE` writer - WDT Interrupt Enable"]
pub type WdtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIE` reader - Osc Fault Enable"]
pub type OfieR = crate::BitReader;
#[doc = "Field `OFIE` writer - Osc Fault Enable"]
pub type OfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMAIE` reader - Vacant Memory Interrupt Enable"]
pub type VmaieR = crate::BitReader;
#[doc = "Field `VMAIE` writer - Vacant Memory Interrupt Enable"]
pub type VmaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub type NmiieR = crate::BitReader;
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub type NmiieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBINIE` reader - JTAG Mail Box input Interrupt Enable"]
pub type JmbinieR = crate::BitReader;
#[doc = "Field `JMBINIE` writer - JTAG Mail Box input Interrupt Enable"]
pub type JmbinieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUTIE` reader - JTAG Mail Box output Interrupt Enable"]
pub type JmboutieR = crate::BitReader;
#[doc = "Field `JMBOUTIE` writer - JTAG Mail Box output Interrupt Enable"]
pub type JmboutieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WdtieR {
        WdtieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OfieR {
        OfieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VmaieR {
        VmaieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NmiieR {
        NmiieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JmbinieR {
        JmbinieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JmboutieR {
        JmboutieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WdtieW<'_, Sfrie1Spec> {
        WdtieW::new(self, 0)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OfieW<'_, Sfrie1Spec> {
        OfieW::new(self, 1)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&mut self) -> VmaieW<'_, Sfrie1Spec> {
        VmaieW::new(self, 3)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NmiieW<'_, Sfrie1Spec> {
        NmiieW::new(self, 4)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&mut self) -> JmbinieW<'_, Sfrie1Spec> {
        JmbinieW::new(self, 6)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&mut self) -> JmboutieW<'_, Sfrie1Spec> {
        JmboutieW::new(self, 7)
    }
}
#[doc = "Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrie1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrie1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sfrie1Spec;
impl crate::RegisterSpec for Sfrie1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrie1::R`](R) reader structure"]
impl crate::Readable for Sfrie1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfrie1::W`](W) writer structure"]
impl crate::Writable for Sfrie1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRIE1 to value 0"]
impl crate::Resettable for Sfrie1Spec {}
