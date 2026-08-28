#[doc = "Register `CAPD` reader"]
pub type R = crate::R<CapdSpec>;
#[doc = "Register `CAPD` writer"]
pub type W = crate::W<CapdSpec>;
#[doc = "Field `CAPD0` reader - Comp. A Disable Input Buffer of Port Register .0"]
pub type Capd0R = crate::BitReader;
#[doc = "Field `CAPD0` writer - Comp. A Disable Input Buffer of Port Register .0"]
pub type Capd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD1` reader - Comp. A Disable Input Buffer of Port Register .1"]
pub type Capd1R = crate::BitReader;
#[doc = "Field `CAPD1` writer - Comp. A Disable Input Buffer of Port Register .1"]
pub type Capd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD2` reader - Comp. A Disable Input Buffer of Port Register .2"]
pub type Capd2R = crate::BitReader;
#[doc = "Field `CAPD2` writer - Comp. A Disable Input Buffer of Port Register .2"]
pub type Capd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD3` reader - Comp. A Disable Input Buffer of Port Register .3"]
pub type Capd3R = crate::BitReader;
#[doc = "Field `CAPD3` writer - Comp. A Disable Input Buffer of Port Register .3"]
pub type Capd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD4` reader - Comp. A Disable Input Buffer of Port Register .4"]
pub type Capd4R = crate::BitReader;
#[doc = "Field `CAPD4` writer - Comp. A Disable Input Buffer of Port Register .4"]
pub type Capd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD5` reader - Comp. A Disable Input Buffer of Port Register .5"]
pub type Capd5R = crate::BitReader;
#[doc = "Field `CAPD5` writer - Comp. A Disable Input Buffer of Port Register .5"]
pub type Capd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD6` reader - Comp. A Disable Input Buffer of Port Register .6"]
pub type Capd6R = crate::BitReader;
#[doc = "Field `CAPD6` writer - Comp. A Disable Input Buffer of Port Register .6"]
pub type Capd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPD7` reader - Comp. A Disable Input Buffer of Port Register .7"]
pub type Capd7R = crate::BitReader;
#[doc = "Field `CAPD7` writer - Comp. A Disable Input Buffer of Port Register .7"]
pub type Capd7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&self) -> Capd0R {
        Capd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&self) -> Capd1R {
        Capd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&self) -> Capd2R {
        Capd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&self) -> Capd3R {
        Capd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&self) -> Capd4R {
        Capd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&self) -> Capd5R {
        Capd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&self) -> Capd6R {
        Capd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&self) -> Capd7R {
        Capd7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&mut self) -> Capd0W<'_, CapdSpec> {
        Capd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&mut self) -> Capd1W<'_, CapdSpec> {
        Capd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&mut self) -> Capd2W<'_, CapdSpec> {
        Capd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&mut self) -> Capd3W<'_, CapdSpec> {
        Capd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&mut self) -> Capd4W<'_, CapdSpec> {
        Capd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&mut self) -> Capd5W<'_, CapdSpec> {
        Capd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&mut self) -> Capd6W<'_, CapdSpec> {
        Capd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&mut self) -> Capd7W<'_, CapdSpec> {
        Capd7W::new(self, 7)
    }
}
#[doc = "Comparator A Port Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`capd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapdSpec;
impl crate::RegisterSpec for CapdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`capd::R`](R) reader structure"]
impl crate::Readable for CapdSpec {}
#[doc = "`write(|w| ..)` method takes [`capd::W`](W) writer structure"]
impl crate::Writable for CapdSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CAPD to value 0"]
impl crate::Resettable for CapdSpec {}
