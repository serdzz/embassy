#[doc = "Register `SYSCFG0` reader"]
pub type R = crate::R<Syscfg0Spec>;
#[doc = "Register `SYSCFG0` writer"]
pub type W = crate::W<Syscfg0Spec>;
#[doc = "Field `PFWP` reader - Program FRAM Write Protection"]
pub type PfwpR = crate::BitReader;
#[doc = "Field `PFWP` writer - Program FRAM Write Protection"]
pub type PfwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFWP` reader - Data FRAM Write Protection"]
pub type DfwpR = crate::BitReader;
#[doc = "Field `DFWP` writer - Data FRAM Write Protection"]
pub type DfwpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PfwpR {
        PfwpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DfwpR {
        DfwpR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&mut self) -> PfwpW<'_, Syscfg0Spec> {
        PfwpW::new(self, 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&mut self) -> DfwpW<'_, Syscfg0Spec> {
        DfwpW::new(self, 1)
    }
}
#[doc = "System Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg0Spec;
impl crate::RegisterSpec for Syscfg0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg0::R`](R) reader structure"]
impl crate::Readable for Syscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg0::W`](W) writer structure"]
impl crate::Writable for Syscfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for Syscfg0Spec {}
