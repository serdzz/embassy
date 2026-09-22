#[doc = "Register `SYSCFG1` reader"]
pub type R = crate::R<Syscfg1Spec>;
#[doc = "Register `SYSCFG1` writer"]
pub type W = crate::W<Syscfg1Spec>;
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRPSEL` reader - Infrared polarity select"]
pub type IrpselR = crate::BitReader;
#[doc = "Field `IRPSEL` writer - Infrared polarity select"]
pub type IrpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRMSEL` reader - Infrared mode select"]
pub type IrmselR = crate::BitReader;
#[doc = "Field `IRMSEL` writer - Infrared mode select"]
pub type IrmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDSSEL` reader - Infrared data source select"]
pub type IrdsselR = crate::BitReader;
#[doc = "Field `IRDSSEL` writer - Infrared data source select"]
pub type IrdsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDATA` reader - Infrared enable"]
pub type IrdataR = crate::BitReader;
#[doc = "Field `IRDATA` writer - Infrared enable"]
pub type IrdataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&self) -> IrpselR {
        IrpselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&self) -> IrmselR {
        IrmselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&self) -> IrdsselR {
        IrdsselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Infrared enable"]
    #[inline(always)]
    pub fn irdata(&self) -> IrdataR {
        IrdataR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<'_, Syscfg1Spec> {
        IrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&mut self) -> IrpselW<'_, Syscfg1Spec> {
        IrpselW::new(self, 1)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&mut self) -> IrmselW<'_, Syscfg1Spec> {
        IrmselW::new(self, 2)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&mut self) -> IrdsselW<'_, Syscfg1Spec> {
        IrdsselW::new(self, 3)
    }
    #[doc = "Bit 4 - Infrared enable"]
    #[inline(always)]
    pub fn irdata(&mut self) -> IrdataW<'_, Syscfg1Spec> {
        IrdataW::new(self, 4)
    }
}
#[doc = "System Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg1Spec;
impl crate::RegisterSpec for Syscfg1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg1::R`](R) reader structure"]
impl crate::Readable for Syscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg1::W`](W) writer structure"]
impl crate::Writable for Syscfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG1 to value 0"]
impl crate::Resettable for Syscfg1Spec {}
