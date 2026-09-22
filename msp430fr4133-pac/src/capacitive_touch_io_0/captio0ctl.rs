#[doc = "Register `CAPTIO0CTL` reader"]
pub type R = crate::R<Captio0ctlSpec>;
#[doc = "Register `CAPTIO0CTL` writer"]
pub type W = crate::W<Captio0ctlSpec>;
#[doc = "Field `CAPTIOPISEL0` reader - CapTouchIO Pin Select Bit: 0"]
pub type Captiopisel0R = crate::BitReader;
#[doc = "Field `CAPTIOPISEL0` writer - CapTouchIO Pin Select Bit: 0"]
pub type Captiopisel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIOPISEL1` reader - CapTouchIO Pin Select Bit: 1"]
pub type Captiopisel1R = crate::BitReader;
#[doc = "Field `CAPTIOPISEL1` writer - CapTouchIO Pin Select Bit: 1"]
pub type Captiopisel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIOPISEL2` reader - CapTouchIO Pin Select Bit: 2"]
pub type Captiopisel2R = crate::BitReader;
#[doc = "Field `CAPTIOPISEL2` writer - CapTouchIO Pin Select Bit: 2"]
pub type Captiopisel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIOPOSEL0` reader - CapTouchIO Port Select Bit: 0"]
pub type Captioposel0R = crate::BitReader;
#[doc = "Field `CAPTIOPOSEL0` writer - CapTouchIO Port Select Bit: 0"]
pub type Captioposel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIOPOSEL1` reader - CapTouchIO Port Select Bit: 1"]
pub type Captioposel1R = crate::BitReader;
#[doc = "Field `CAPTIOPOSEL1` writer - CapTouchIO Port Select Bit: 1"]
pub type Captioposel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIOPOSEL2` reader - CapTouchIO Port Select Bit: 2"]
pub type Captioposel2R = crate::BitReader;
#[doc = "Field `CAPTIOPOSEL2` writer - CapTouchIO Port Select Bit: 2"]
pub type Captioposel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIOPOSEL3` reader - CapTouchIO Port Select Bit: 3"]
pub type Captioposel3R = crate::BitReader;
#[doc = "Field `CAPTIOPOSEL3` writer - CapTouchIO Port Select Bit: 3"]
pub type Captioposel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIOEN` reader - CapTouchIO Enable"]
pub type CaptioenR = crate::BitReader;
#[doc = "Field `CAPTIOEN` writer - CapTouchIO Enable"]
pub type CaptioenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTIO` reader - CapTouchIO state"]
pub type CaptioR = crate::BitReader;
#[doc = "Field `CAPTIO` writer - CapTouchIO state"]
pub type CaptioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - CapTouchIO Pin Select Bit: 0"]
    #[inline(always)]
    pub fn captiopisel0(&self) -> Captiopisel0R {
        Captiopisel0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CapTouchIO Pin Select Bit: 1"]
    #[inline(always)]
    pub fn captiopisel1(&self) -> Captiopisel1R {
        Captiopisel1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CapTouchIO Pin Select Bit: 2"]
    #[inline(always)]
    pub fn captiopisel2(&self) -> Captiopisel2R {
        Captiopisel2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CapTouchIO Port Select Bit: 0"]
    #[inline(always)]
    pub fn captioposel0(&self) -> Captioposel0R {
        Captioposel0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CapTouchIO Port Select Bit: 1"]
    #[inline(always)]
    pub fn captioposel1(&self) -> Captioposel1R {
        Captioposel1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CapTouchIO Port Select Bit: 2"]
    #[inline(always)]
    pub fn captioposel2(&self) -> Captioposel2R {
        Captioposel2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CapTouchIO Port Select Bit: 3"]
    #[inline(always)]
    pub fn captioposel3(&self) -> Captioposel3R {
        Captioposel3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CapTouchIO Enable"]
    #[inline(always)]
    pub fn captioen(&self) -> CaptioenR {
        CaptioenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CapTouchIO state"]
    #[inline(always)]
    pub fn captio(&self) -> CaptioR {
        CaptioR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CapTouchIO Pin Select Bit: 0"]
    #[inline(always)]
    pub fn captiopisel0(&mut self) -> Captiopisel0W<'_, Captio0ctlSpec> {
        Captiopisel0W::new(self, 1)
    }
    #[doc = "Bit 2 - CapTouchIO Pin Select Bit: 1"]
    #[inline(always)]
    pub fn captiopisel1(&mut self) -> Captiopisel1W<'_, Captio0ctlSpec> {
        Captiopisel1W::new(self, 2)
    }
    #[doc = "Bit 3 - CapTouchIO Pin Select Bit: 2"]
    #[inline(always)]
    pub fn captiopisel2(&mut self) -> Captiopisel2W<'_, Captio0ctlSpec> {
        Captiopisel2W::new(self, 3)
    }
    #[doc = "Bit 4 - CapTouchIO Port Select Bit: 0"]
    #[inline(always)]
    pub fn captioposel0(&mut self) -> Captioposel0W<'_, Captio0ctlSpec> {
        Captioposel0W::new(self, 4)
    }
    #[doc = "Bit 5 - CapTouchIO Port Select Bit: 1"]
    #[inline(always)]
    pub fn captioposel1(&mut self) -> Captioposel1W<'_, Captio0ctlSpec> {
        Captioposel1W::new(self, 5)
    }
    #[doc = "Bit 6 - CapTouchIO Port Select Bit: 2"]
    #[inline(always)]
    pub fn captioposel2(&mut self) -> Captioposel2W<'_, Captio0ctlSpec> {
        Captioposel2W::new(self, 6)
    }
    #[doc = "Bit 7 - CapTouchIO Port Select Bit: 3"]
    #[inline(always)]
    pub fn captioposel3(&mut self) -> Captioposel3W<'_, Captio0ctlSpec> {
        Captioposel3W::new(self, 7)
    }
    #[doc = "Bit 8 - CapTouchIO Enable"]
    #[inline(always)]
    pub fn captioen(&mut self) -> CaptioenW<'_, Captio0ctlSpec> {
        CaptioenW::new(self, 8)
    }
    #[doc = "Bit 9 - CapTouchIO state"]
    #[inline(always)]
    pub fn captio(&mut self) -> CaptioW<'_, Captio0ctlSpec> {
        CaptioW::new(self, 9)
    }
}
#[doc = "Capacitive_Touch_IO 0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`captio0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`captio0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Captio0ctlSpec;
impl crate::RegisterSpec for Captio0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`captio0ctl::R`](R) reader structure"]
impl crate::Readable for Captio0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`captio0ctl::W`](W) writer structure"]
impl crate::Writable for Captio0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAPTIO0CTL to value 0"]
impl crate::Resettable for Captio0ctlSpec {}
