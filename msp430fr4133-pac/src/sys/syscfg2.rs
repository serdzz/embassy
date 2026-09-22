#[doc = "Register `SYSCFG2` reader"]
pub type R = crate::R<Syscfg2Spec>;
#[doc = "Register `SYSCFG2` writer"]
pub type W = crate::W<Syscfg2Spec>;
#[doc = "Field `ADCPCTL0` reader - ADC input A0 pin select"]
pub type Adcpctl0R = crate::BitReader;
#[doc = "Field `ADCPCTL0` writer - ADC input A0 pin select"]
pub type Adcpctl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL1` reader - ADC input A1 pin select"]
pub type Adcpctl1R = crate::BitReader;
#[doc = "Field `ADCPCTL1` writer - ADC input A1 pin select"]
pub type Adcpctl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL2` reader - ADC input A2 pin select"]
pub type Adcpctl2R = crate::BitReader;
#[doc = "Field `ADCPCTL2` writer - ADC input A2 pin select"]
pub type Adcpctl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL3` reader - ADC input A3 pin select"]
pub type Adcpctl3R = crate::BitReader;
#[doc = "Field `ADCPCTL3` writer - ADC input A3 pin select"]
pub type Adcpctl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL4` reader - ADC input A4 pin select"]
pub type Adcpctl4R = crate::BitReader;
#[doc = "Field `ADCPCTL4` writer - ADC input A4 pin select"]
pub type Adcpctl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL5` reader - ADC input A5 pin select"]
pub type Adcpctl5R = crate::BitReader;
#[doc = "Field `ADCPCTL5` writer - ADC input A5 pin select"]
pub type Adcpctl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL6` reader - ADC input A6 pin select"]
pub type Adcpctl6R = crate::BitReader;
#[doc = "Field `ADCPCTL6` writer - ADC input A6 pin select"]
pub type Adcpctl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL7` reader - ADC input A7 pin select"]
pub type Adcpctl7R = crate::BitReader;
#[doc = "Field `ADCPCTL7` writer - ADC input A7 pin select"]
pub type Adcpctl7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL8` reader - ADC input A8 pin select"]
pub type Adcpctl8R = crate::BitReader;
#[doc = "Field `ADCPCTL8` writer - ADC input A8 pin select"]
pub type Adcpctl8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL9` reader - ADC input A9 pin select"]
pub type Adcpctl9R = crate::BitReader;
#[doc = "Field `ADCPCTL9` writer - ADC input A9 pin select"]
pub type Adcpctl9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDPCTL` reader - LCD Power Pin"]
pub type LcdpctlR = crate::BitReader;
#[doc = "Field `LCDPCTL` writer - LCD Power Pin"]
pub type LcdpctlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&self) -> Adcpctl0R {
        Adcpctl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&self) -> Adcpctl1R {
        Adcpctl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&self) -> Adcpctl2R {
        Adcpctl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&self) -> Adcpctl3R {
        Adcpctl3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&self) -> Adcpctl4R {
        Adcpctl4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&self) -> Adcpctl5R {
        Adcpctl5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&self) -> Adcpctl6R {
        Adcpctl6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&self) -> Adcpctl7R {
        Adcpctl7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&self) -> Adcpctl8R {
        Adcpctl8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&self) -> Adcpctl9R {
        Adcpctl9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD Power Pin"]
    #[inline(always)]
    pub fn lcdpctl(&self) -> LcdpctlR {
        LcdpctlR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&mut self) -> Adcpctl0W<'_, Syscfg2Spec> {
        Adcpctl0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&mut self) -> Adcpctl1W<'_, Syscfg2Spec> {
        Adcpctl1W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&mut self) -> Adcpctl2W<'_, Syscfg2Spec> {
        Adcpctl2W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&mut self) -> Adcpctl3W<'_, Syscfg2Spec> {
        Adcpctl3W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&mut self) -> Adcpctl4W<'_, Syscfg2Spec> {
        Adcpctl4W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&mut self) -> Adcpctl5W<'_, Syscfg2Spec> {
        Adcpctl5W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&mut self) -> Adcpctl6W<'_, Syscfg2Spec> {
        Adcpctl6W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&mut self) -> Adcpctl7W<'_, Syscfg2Spec> {
        Adcpctl7W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC input A8 pin select"]
    #[inline(always)]
    pub fn adcpctl8(&mut self) -> Adcpctl8W<'_, Syscfg2Spec> {
        Adcpctl8W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC input A9 pin select"]
    #[inline(always)]
    pub fn adcpctl9(&mut self) -> Adcpctl9W<'_, Syscfg2Spec> {
        Adcpctl9W::new(self, 9)
    }
    #[doc = "Bit 12 - LCD Power Pin"]
    #[inline(always)]
    pub fn lcdpctl(&mut self) -> LcdpctlW<'_, Syscfg2Spec> {
        LcdpctlW::new(self, 12)
    }
}
#[doc = "System Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg2Spec;
impl crate::RegisterSpec for Syscfg2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg2::R`](R) reader structure"]
impl crate::Readable for Syscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg2::W`](W) writer structure"]
impl crate::Writable for Syscfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG2 to value 0"]
impl crate::Resettable for Syscfg2Spec {}
