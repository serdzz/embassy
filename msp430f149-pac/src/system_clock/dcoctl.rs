#[doc = "Register `DCOCTL` reader"]
pub type R = crate::R<DcoctlSpec>;
#[doc = "Register `DCOCTL` writer"]
pub type W = crate::W<DcoctlSpec>;
#[doc = "Field `MOD0` reader - Modulation Bit 0"]
pub type Mod0R = crate::BitReader;
#[doc = "Field `MOD0` writer - Modulation Bit 0"]
pub type Mod0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD1` reader - Modulation Bit 1"]
pub type Mod1R = crate::BitReader;
#[doc = "Field `MOD1` writer - Modulation Bit 1"]
pub type Mod1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD2` reader - Modulation Bit 2"]
pub type Mod2R = crate::BitReader;
#[doc = "Field `MOD2` writer - Modulation Bit 2"]
pub type Mod2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD3` reader - Modulation Bit 3"]
pub type Mod3R = crate::BitReader;
#[doc = "Field `MOD3` writer - Modulation Bit 3"]
pub type Mod3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD4` reader - Modulation Bit 4"]
pub type Mod4R = crate::BitReader;
#[doc = "Field `MOD4` writer - Modulation Bit 4"]
pub type Mod4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO0` reader - DCO Select Bit 0"]
pub type Dco0R = crate::BitReader;
#[doc = "Field `DCO0` writer - DCO Select Bit 0"]
pub type Dco0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO1` reader - DCO Select Bit 1"]
pub type Dco1R = crate::BitReader;
#[doc = "Field `DCO1` writer - DCO Select Bit 1"]
pub type Dco1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCO2` reader - DCO Select Bit 2"]
pub type Dco2R = crate::BitReader;
#[doc = "Field `DCO2` writer - DCO Select Bit 2"]
pub type Dco2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod0(&self) -> Mod0R {
        Mod0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Modulation Bit 1"]
    #[inline(always)]
    pub fn mod1(&self) -> Mod1R {
        Mod1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Modulation Bit 2"]
    #[inline(always)]
    pub fn mod2(&self) -> Mod2R {
        Mod2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Modulation Bit 3"]
    #[inline(always)]
    pub fn mod3(&self) -> Mod3R {
        Mod3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Modulation Bit 4"]
    #[inline(always)]
    pub fn mod4(&self) -> Mod4R {
        Mod4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco0(&self) -> Dco0R {
        Dco0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO Select Bit 1"]
    #[inline(always)]
    pub fn dco1(&self) -> Dco1R {
        Dco1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCO Select Bit 2"]
    #[inline(always)]
    pub fn dco2(&self) -> Dco2R {
        Dco2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod0(&mut self) -> Mod0W<'_, DcoctlSpec> {
        Mod0W::new(self, 0)
    }
    #[doc = "Bit 1 - Modulation Bit 1"]
    #[inline(always)]
    pub fn mod1(&mut self) -> Mod1W<'_, DcoctlSpec> {
        Mod1W::new(self, 1)
    }
    #[doc = "Bit 2 - Modulation Bit 2"]
    #[inline(always)]
    pub fn mod2(&mut self) -> Mod2W<'_, DcoctlSpec> {
        Mod2W::new(self, 2)
    }
    #[doc = "Bit 3 - Modulation Bit 3"]
    #[inline(always)]
    pub fn mod3(&mut self) -> Mod3W<'_, DcoctlSpec> {
        Mod3W::new(self, 3)
    }
    #[doc = "Bit 4 - Modulation Bit 4"]
    #[inline(always)]
    pub fn mod4(&mut self) -> Mod4W<'_, DcoctlSpec> {
        Mod4W::new(self, 4)
    }
    #[doc = "Bit 5 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco0(&mut self) -> Dco0W<'_, DcoctlSpec> {
        Dco0W::new(self, 5)
    }
    #[doc = "Bit 6 - DCO Select Bit 1"]
    #[inline(always)]
    pub fn dco1(&mut self) -> Dco1W<'_, DcoctlSpec> {
        Dco1W::new(self, 6)
    }
    #[doc = "Bit 7 - DCO Select Bit 2"]
    #[inline(always)]
    pub fn dco2(&mut self) -> Dco2W<'_, DcoctlSpec> {
        Dco2W::new(self, 7)
    }
}
#[doc = "DCO Clock Frequency Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoctlSpec;
impl crate::RegisterSpec for DcoctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dcoctl::R`](R) reader structure"]
impl crate::Readable for DcoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcoctl::W`](W) writer structure"]
impl crate::Writable for DcoctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DCOCTL to value 0"]
impl crate::Resettable for DcoctlSpec {}
