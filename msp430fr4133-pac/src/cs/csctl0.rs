#[doc = "Register `CSCTL0` reader"]
pub type R = crate::R<Csctl0Spec>;
#[doc = "Register `CSCTL0` writer"]
pub type W = crate::W<Csctl0Spec>;
#[doc = "Field `DCO0` reader - DCO TAP Bit : 0"]
pub type Dco0R = crate::BitReader;
#[doc = "Field `DCO0` writer - DCO TAP Bit : 0"]
pub type Dco0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO1` reader - DCO TAP Bit : 1"]
pub type Dco1R = crate::BitReader;
#[doc = "Field `DCO1` writer - DCO TAP Bit : 1"]
pub type Dco1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO2` reader - DCO TAP Bit : 2"]
pub type Dco2R = crate::BitReader;
#[doc = "Field `DCO2` writer - DCO TAP Bit : 2"]
pub type Dco2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO3` reader - DCO TAP Bit : 3"]
pub type Dco3R = crate::BitReader;
#[doc = "Field `DCO3` writer - DCO TAP Bit : 3"]
pub type Dco3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO4` reader - DCO TAP Bit : 4"]
pub type Dco4R = crate::BitReader;
#[doc = "Field `DCO4` writer - DCO TAP Bit : 4"]
pub type Dco4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO5` reader - DCO TAP Bit : 5"]
pub type Dco5R = crate::BitReader;
#[doc = "Field `DCO5` writer - DCO TAP Bit : 5"]
pub type Dco5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO6` reader - DCO TAP Bit : 6"]
pub type Dco6R = crate::BitReader;
#[doc = "Field `DCO6` writer - DCO TAP Bit : 6"]
pub type Dco6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO7` reader - DCO TAP Bit : 7"]
pub type Dco7R = crate::BitReader;
#[doc = "Field `DCO7` writer - DCO TAP Bit : 7"]
pub type Dco7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO8` reader - DCO TAP Bit : 8"]
pub type Dco8R = crate::BitReader;
#[doc = "Field `DCO8` writer - DCO TAP Bit : 8"]
pub type Dco8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD0` reader - Modulation Bit Counter Bit : 0"]
pub type Mod0R = crate::BitReader;
#[doc = "Field `MOD0` writer - Modulation Bit Counter Bit : 0"]
pub type Mod0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD1` reader - Modulation Bit Counter Bit : 1"]
pub type Mod1R = crate::BitReader;
#[doc = "Field `MOD1` writer - Modulation Bit Counter Bit : 1"]
pub type Mod1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD2` reader - Modulation Bit Counter Bit : 2"]
pub type Mod2R = crate::BitReader;
#[doc = "Field `MOD2` writer - Modulation Bit Counter Bit : 2"]
pub type Mod2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD3` reader - Modulation Bit Counter Bit : 3"]
pub type Mod3R = crate::BitReader;
#[doc = "Field `MOD3` writer - Modulation Bit Counter Bit : 3"]
pub type Mod3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD4` reader - Modulation Bit Counter Bit : 4"]
pub type Mod4R = crate::BitReader;
#[doc = "Field `MOD4` writer - Modulation Bit Counter Bit : 4"]
pub type Mod4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&self) -> Dco0R {
        Dco0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&self) -> Dco1R {
        Dco1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&self) -> Dco2R {
        Dco2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&self) -> Dco3R {
        Dco3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&self) -> Dco4R {
        Dco4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCO TAP Bit : 5"]
    #[inline(always)]
    pub fn dco5(&self) -> Dco5R {
        Dco5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO TAP Bit : 6"]
    #[inline(always)]
    pub fn dco6(&self) -> Dco6R {
        Dco6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCO TAP Bit : 7"]
    #[inline(always)]
    pub fn dco7(&self) -> Dco7R {
        Dco7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 8"]
    #[inline(always)]
    pub fn dco8(&self) -> Dco8R {
        Dco8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&self) -> Mod0R {
        Mod0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&self) -> Mod1R {
        Mod1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&self) -> Mod2R {
        Mod2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&self) -> Mod3R {
        Mod3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&self) -> Mod4R {
        Mod4R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&mut self) -> Dco0W<'_, Csctl0Spec> {
        Dco0W::new(self, 0)
    }
    #[doc = "Bit 1 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&mut self) -> Dco1W<'_, Csctl0Spec> {
        Dco1W::new(self, 1)
    }
    #[doc = "Bit 2 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&mut self) -> Dco2W<'_, Csctl0Spec> {
        Dco2W::new(self, 2)
    }
    #[doc = "Bit 3 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&mut self) -> Dco3W<'_, Csctl0Spec> {
        Dco3W::new(self, 3)
    }
    #[doc = "Bit 4 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&mut self) -> Dco4W<'_, Csctl0Spec> {
        Dco4W::new(self, 4)
    }
    #[doc = "Bit 5 - DCO TAP Bit : 5"]
    #[inline(always)]
    pub fn dco5(&mut self) -> Dco5W<'_, Csctl0Spec> {
        Dco5W::new(self, 5)
    }
    #[doc = "Bit 6 - DCO TAP Bit : 6"]
    #[inline(always)]
    pub fn dco6(&mut self) -> Dco6W<'_, Csctl0Spec> {
        Dco6W::new(self, 6)
    }
    #[doc = "Bit 7 - DCO TAP Bit : 7"]
    #[inline(always)]
    pub fn dco7(&mut self) -> Dco7W<'_, Csctl0Spec> {
        Dco7W::new(self, 7)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 8"]
    #[inline(always)]
    pub fn dco8(&mut self) -> Dco8W<'_, Csctl0Spec> {
        Dco8W::new(self, 8)
    }
    #[doc = "Bit 9 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&mut self) -> Mod0W<'_, Csctl0Spec> {
        Mod0W::new(self, 9)
    }
    #[doc = "Bit 10 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&mut self) -> Mod1W<'_, Csctl0Spec> {
        Mod1W::new(self, 10)
    }
    #[doc = "Bit 11 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&mut self) -> Mod2W<'_, Csctl0Spec> {
        Mod2W::new(self, 11)
    }
    #[doc = "Bit 12 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&mut self) -> Mod3W<'_, Csctl0Spec> {
        Mod3W::new(self, 12)
    }
    #[doc = "Bit 13 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&mut self) -> Mod4W<'_, Csctl0Spec> {
        Mod4W::new(self, 13)
    }
}
#[doc = "CS Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl0Spec;
impl crate::RegisterSpec for Csctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl0::R`](R) reader structure"]
impl crate::Readable for Csctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl0::W`](W) writer structure"]
impl crate::Writable for Csctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL0 to value 0"]
impl crate::Resettable for Csctl0Spec {}
