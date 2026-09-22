#[doc = "Register `PM5CTL0` reader"]
pub type R = crate::R<Pm5ctl0Spec>;
#[doc = "Register `PM5CTL0` writer"]
pub type W = crate::W<Pm5ctl0Spec>;
#[doc = "Field `LOCKLPM5` reader - Lock I/O pin configuration upon entry/exit to/from LPM5"]
pub type Locklpm5R = crate::BitReader;
#[doc = "Field `LOCKLPM5` writer - Lock I/O pin configuration upon entry/exit to/from LPM5"]
pub type Locklpm5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM5SW` reader - LPMx.5 switch dis/connected"]
pub type Lpm5swR = crate::BitReader;
#[doc = "Field `LPM5SW` writer - LPMx.5 switch dis/connected"]
pub type Lpm5swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM5SM` reader - Manual mode for LPM3.5 switch"]
pub type Lpm5smR = crate::BitReader;
#[doc = "Field `LPM5SM` writer - Manual mode for LPM3.5 switch"]
pub type Lpm5smW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    pub fn locklpm5(&self) -> Locklpm5R {
        Locklpm5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - LPMx.5 switch dis/connected"]
    #[inline(always)]
    pub fn lpm5sw(&self) -> Lpm5swR {
        Lpm5swR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Manual mode for LPM3.5 switch"]
    #[inline(always)]
    pub fn lpm5sm(&self) -> Lpm5smR {
        Lpm5smR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> Locklpm5W<'_, Pm5ctl0Spec> {
        Locklpm5W::new(self, 0)
    }
    #[doc = "Bit 4 - LPMx.5 switch dis/connected"]
    #[inline(always)]
    pub fn lpm5sw(&mut self) -> Lpm5swW<'_, Pm5ctl0Spec> {
        Lpm5swW::new(self, 4)
    }
    #[doc = "Bit 5 - Manual mode for LPM3.5 switch"]
    #[inline(always)]
    pub fn lpm5sm(&mut self) -> Lpm5smW<'_, Pm5ctl0Spec> {
        Lpm5smW::new(self, 5)
    }
}
#[doc = "PMM Power Mode 5 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pm5ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm5ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pm5ctl0Spec;
impl crate::RegisterSpec for Pm5ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pm5ctl0::R`](R) reader structure"]
impl crate::Readable for Pm5ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pm5ctl0::W`](W) writer structure"]
impl crate::Writable for Pm5ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PM5CTL0 to value 0"]
impl crate::Resettable for Pm5ctl0Spec {}
