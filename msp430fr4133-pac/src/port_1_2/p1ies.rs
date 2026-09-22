#[doc = "Register `P1IES` reader"]
pub type R = crate::R<P1iesSpec>;
#[doc = "Register `P1IES` writer"]
pub type W = crate::W<P1iesSpec>;
#[doc = "Field `P1IES0` reader - P1IES0"]
pub type P1ies0R = crate::BitReader;
#[doc = "Field `P1IES0` writer - P1IES0"]
pub type P1ies0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES1` reader - P1IES1"]
pub type P1ies1R = crate::BitReader;
#[doc = "Field `P1IES1` writer - P1IES1"]
pub type P1ies1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES2` reader - P1IES2"]
pub type P1ies2R = crate::BitReader;
#[doc = "Field `P1IES2` writer - P1IES2"]
pub type P1ies2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES3` reader - P1IES3"]
pub type P1ies3R = crate::BitReader;
#[doc = "Field `P1IES3` writer - P1IES3"]
pub type P1ies3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES4` reader - P1IES4"]
pub type P1ies4R = crate::BitReader;
#[doc = "Field `P1IES4` writer - P1IES4"]
pub type P1ies4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES5` reader - P1IES5"]
pub type P1ies5R = crate::BitReader;
#[doc = "Field `P1IES5` writer - P1IES5"]
pub type P1ies5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES6` reader - P1IES6"]
pub type P1ies6R = crate::BitReader;
#[doc = "Field `P1IES6` writer - P1IES6"]
pub type P1ies6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1IES7` reader - P1IES7"]
pub type P1ies7R = crate::BitReader;
#[doc = "Field `P1IES7` writer - P1IES7"]
pub type P1ies7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    pub fn p1ies0(&self) -> P1ies0R {
        P1ies0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    pub fn p1ies1(&self) -> P1ies1R {
        P1ies1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    pub fn p1ies2(&self) -> P1ies2R {
        P1ies2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    pub fn p1ies3(&self) -> P1ies3R {
        P1ies3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    pub fn p1ies4(&self) -> P1ies4R {
        P1ies4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    pub fn p1ies5(&self) -> P1ies5R {
        P1ies5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    pub fn p1ies6(&self) -> P1ies6R {
        P1ies6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    pub fn p1ies7(&self) -> P1ies7R {
        P1ies7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    pub fn p1ies0(&mut self) -> P1ies0W<'_, P1iesSpec> {
        P1ies0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    pub fn p1ies1(&mut self) -> P1ies1W<'_, P1iesSpec> {
        P1ies1W::new(self, 1)
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    pub fn p1ies2(&mut self) -> P1ies2W<'_, P1iesSpec> {
        P1ies2W::new(self, 2)
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    pub fn p1ies3(&mut self) -> P1ies3W<'_, P1iesSpec> {
        P1ies3W::new(self, 3)
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    pub fn p1ies4(&mut self) -> P1ies4W<'_, P1iesSpec> {
        P1ies4W::new(self, 4)
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    pub fn p1ies5(&mut self) -> P1ies5W<'_, P1iesSpec> {
        P1ies5W::new(self, 5)
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    pub fn p1ies6(&mut self) -> P1ies6W<'_, P1iesSpec> {
        P1ies6W::new(self, 6)
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    pub fn p1ies7(&mut self) -> P1ies7W<'_, P1iesSpec> {
        P1ies7W::new(self, 7)
    }
}
#[doc = "Port 1 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1iesSpec;
impl crate::RegisterSpec for P1iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ies::R`](R) reader structure"]
impl crate::Readable for P1iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ies::W`](W) writer structure"]
impl crate::Writable for P1iesSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets P1IES to value 0"]
impl crate::Resettable for P1iesSpec {}
