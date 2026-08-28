#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WdtctlSpec>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WdtctlSpec>;
#[doc = "Field `WDTIS0` reader - WDTIS0"]
pub type Wdtis0R = crate::BitReader;
#[doc = "Field `WDTIS0` writer - WDTIS0"]
pub type Wdtis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTIS1` reader - WDTIS1"]
pub type Wdtis1R = crate::BitReader;
#[doc = "Field `WDTIS1` writer - WDTIS1"]
pub type Wdtis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTSSEL` reader - WDTSSEL"]
pub type WdtsselR = crate::BitReader;
#[doc = "Field `WDTSSEL` writer - WDTSSEL"]
pub type WdtsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTCNTCL` reader - WDTCNTCL"]
pub type WdtcntclR = crate::BitReader;
#[doc = "Field `WDTCNTCL` writer - WDTCNTCL"]
pub type WdtcntclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTTMSEL` reader - WDTTMSEL"]
pub type WdttmselR = crate::BitReader;
#[doc = "Field `WDTTMSEL` writer - WDTTMSEL"]
pub type WdttmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTNMI` reader - WDTNMI"]
pub type WdtnmiR = crate::BitReader;
#[doc = "Field `WDTNMI` writer - WDTNMI"]
pub type WdtnmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTNMIES` reader - WDTNMIES"]
pub type WdtnmiesR = crate::BitReader;
#[doc = "Field `WDTNMIES` writer - WDTNMIES"]
pub type WdtnmiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTHOLD` reader - WDTHOLD"]
pub type WdtholdR = crate::BitReader;
#[doc = "Field `WDTHOLD` writer - WDTHOLD"]
pub type WdtholdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis0(&self) -> Wdtis0R {
        Wdtis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDTIS1"]
    #[inline(always)]
    pub fn wdtis1(&self) -> Wdtis1R {
        Wdtis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WdtsselR {
        WdtsselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WdtcntclR {
        WdtcntclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WdttmselR {
        WdttmselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&self) -> WdtnmiR {
        WdtnmiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&self) -> WdtnmiesR {
        WdtnmiesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&self) -> WdtholdR {
        WdtholdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis0(&mut self) -> Wdtis0W<'_, WdtctlSpec> {
        Wdtis0W::new(self, 0)
    }
    #[doc = "Bit 1 - WDTIS1"]
    #[inline(always)]
    pub fn wdtis1(&mut self) -> Wdtis1W<'_, WdtctlSpec> {
        Wdtis1W::new(self, 1)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WdtsselW<'_, WdtctlSpec> {
        WdtsselW::new(self, 2)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WdtcntclW<'_, WdtctlSpec> {
        WdtcntclW::new(self, 3)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WdttmselW<'_, WdtctlSpec> {
        WdttmselW::new(self, 4)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&mut self) -> WdtnmiW<'_, WdtctlSpec> {
        WdtnmiW::new(self, 5)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&mut self) -> WdtnmiesW<'_, WdtctlSpec> {
        WdtnmiesW::new(self, 6)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WdtholdW<'_, WdtctlSpec> {
        WdtholdW::new(self, 7)
    }
}
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtctlSpec;
impl crate::RegisterSpec for WdtctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtctl::R`](R) reader structure"]
impl crate::Readable for WdtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtctl::W`](W) writer structure"]
impl crate::Writable for WdtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WdtctlSpec {}
