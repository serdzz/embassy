#[doc = "Register `PMMCTL2` reader"]
pub type R = crate::R<Pmmctl2Spec>;
#[doc = "Register `PMMCTL2` writer"]
pub type W = crate::W<Pmmctl2Spec>;
#[doc = "Field `INTREFEN` reader - Internal Reference Enable"]
pub type IntrefenR = crate::BitReader;
#[doc = "Field `INTREFEN` writer - Internal Reference Enable"]
pub type IntrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTREFEN` reader - External Reference output Enable"]
pub type ExtrefenR = crate::BitReader;
#[doc = "Field `EXTREFEN` writer - External Reference output Enable"]
pub type ExtrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENSOREN` reader - Temperature Sensor Enable"]
pub type TsensorenR = crate::BitReader;
#[doc = "Field `TSENSOREN` writer - Temperature Sensor Enable"]
pub type TsensorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENACT` reader - REF Reference generator active"]
pub type RefgenactR = crate::BitReader;
#[doc = "Field `REFGENACT` writer - REF Reference generator active"]
pub type RefgenactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGACT` reader - REF Reference bandgap active"]
pub type RefbgactR = crate::BitReader;
#[doc = "Field `REFBGACT` writer - REF Reference bandgap active"]
pub type RefbgactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGMODE` reader - REF Bandgap mode"]
pub type BgmodeR = crate::BitReader;
#[doc = "Field `BGMODE` writer - REF Bandgap mode"]
pub type BgmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENRDY` reader - REF Reference generator ready"]
pub type RefgenrdyR = crate::BitReader;
#[doc = "Field `REFGENRDY` writer - REF Reference generator ready"]
pub type RefgenrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGRDY` reader - REF Reference bandgap ready"]
pub type RefbgrdyR = crate::BitReader;
#[doc = "Field `REFBGRDY` writer - REF Reference bandgap ready"]
pub type RefbgrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> IntrefenR {
        IntrefenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> ExtrefenR {
        ExtrefenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TsensorenR {
        TsensorenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> RefgenactR {
        RefgenactR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> RefbgactR {
        RefbgactR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BgmodeR {
        BgmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> RefgenrdyR {
        RefgenrdyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> RefbgrdyR {
        RefbgrdyR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&mut self) -> IntrefenW<'_, Pmmctl2Spec> {
        IntrefenW::new(self, 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&mut self) -> ExtrefenW<'_, Pmmctl2Spec> {
        ExtrefenW::new(self, 1)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&mut self) -> TsensorenW<'_, Pmmctl2Spec> {
        TsensorenW::new(self, 3)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&mut self) -> RefgenactW<'_, Pmmctl2Spec> {
        RefgenactW::new(self, 8)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&mut self) -> RefbgactW<'_, Pmmctl2Spec> {
        RefbgactW::new(self, 9)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BgmodeW<'_, Pmmctl2Spec> {
        BgmodeW::new(self, 11)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> RefgenrdyW<'_, Pmmctl2Spec> {
        RefgenrdyW::new(self, 12)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> RefbgrdyW<'_, Pmmctl2Spec> {
        RefbgrdyW::new(self, 13)
    }
}
#[doc = "PMM Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl2Spec;
impl crate::RegisterSpec for Pmmctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl2::R`](R) reader structure"]
impl crate::Readable for Pmmctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl2::W`](W) writer structure"]
impl crate::Writable for Pmmctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL2 to value 0"]
impl crate::Resettable for Pmmctl2Spec {}
