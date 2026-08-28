#[doc = "Register `SVSCTL` reader"]
pub type R = crate::R<SvsctlSpec>;
#[doc = "Register `SVSCTL` writer"]
pub type W = crate::W<SvsctlSpec>;
#[doc = "Field `SVSFG` reader - SVS Flag"]
pub type SvsfgR = crate::BitReader;
#[doc = "Field `SVSFG` writer - SVS Flag"]
pub type SvsfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSOP` reader - SVS output (read only)"]
pub type SvsopR = crate::BitReader;
#[doc = "Field `SVSOP` writer - SVS output (read only)"]
pub type SvsopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSON` reader - Switches the SVS on/off"]
pub type SvsonR = crate::BitReader;
#[doc = "Field `SVSON` writer - Switches the SVS on/off"]
pub type SvsonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORON` reader - Enable POR Generation if Low Voltage"]
pub type PoronR = crate::BitReader;
#[doc = "Field `PORON` writer - Enable POR Generation if Low Voltage"]
pub type PoronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLD0` reader - VLD0"]
pub type Vld0R = crate::BitReader;
#[doc = "Field `VLD0` writer - VLD0"]
pub type Vld0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLD1` reader - VLD1"]
pub type Vld1R = crate::BitReader;
#[doc = "Field `VLD1` writer - VLD1"]
pub type Vld1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLD2` reader - VLD2"]
pub type Vld2R = crate::BitReader;
#[doc = "Field `VLD2` writer - VLD2"]
pub type Vld2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLD3` reader - VLD3"]
pub type Vld3R = crate::BitReader;
#[doc = "Field `VLD3` writer - VLD3"]
pub type Vld3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SVS Flag"]
    #[inline(always)]
    pub fn svsfg(&self) -> SvsfgR {
        SvsfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SVS output (read only)"]
    #[inline(always)]
    pub fn svsop(&self) -> SvsopR {
        SvsopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Switches the SVS on/off"]
    #[inline(always)]
    pub fn svson(&self) -> SvsonR {
        SvsonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable POR Generation if Low Voltage"]
    #[inline(always)]
    pub fn poron(&self) -> PoronR {
        PoronR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VLD0"]
    #[inline(always)]
    pub fn vld0(&self) -> Vld0R {
        Vld0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VLD1"]
    #[inline(always)]
    pub fn vld1(&self) -> Vld1R {
        Vld1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VLD2"]
    #[inline(always)]
    pub fn vld2(&self) -> Vld2R {
        Vld2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VLD3"]
    #[inline(always)]
    pub fn vld3(&self) -> Vld3R {
        Vld3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SVS Flag"]
    #[inline(always)]
    pub fn svsfg(&mut self) -> SvsfgW<'_, SvsctlSpec> {
        SvsfgW::new(self, 0)
    }
    #[doc = "Bit 1 - SVS output (read only)"]
    #[inline(always)]
    pub fn svsop(&mut self) -> SvsopW<'_, SvsctlSpec> {
        SvsopW::new(self, 1)
    }
    #[doc = "Bit 2 - Switches the SVS on/off"]
    #[inline(always)]
    pub fn svson(&mut self) -> SvsonW<'_, SvsctlSpec> {
        SvsonW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable POR Generation if Low Voltage"]
    #[inline(always)]
    pub fn poron(&mut self) -> PoronW<'_, SvsctlSpec> {
        PoronW::new(self, 3)
    }
    #[doc = "Bit 4 - VLD0"]
    #[inline(always)]
    pub fn vld0(&mut self) -> Vld0W<'_, SvsctlSpec> {
        Vld0W::new(self, 4)
    }
    #[doc = "Bit 5 - VLD1"]
    #[inline(always)]
    pub fn vld1(&mut self) -> Vld1W<'_, SvsctlSpec> {
        Vld1W::new(self, 5)
    }
    #[doc = "Bit 6 - VLD2"]
    #[inline(always)]
    pub fn vld2(&mut self) -> Vld2W<'_, SvsctlSpec> {
        Vld2W::new(self, 6)
    }
    #[doc = "Bit 7 - VLD3"]
    #[inline(always)]
    pub fn vld3(&mut self) -> Vld3W<'_, SvsctlSpec> {
        Vld3W::new(self, 7)
    }
}
#[doc = "SVS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`svsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SvsctlSpec;
impl crate::RegisterSpec for SvsctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`svsctl::R`](R) reader structure"]
impl crate::Readable for SvsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`svsctl::W`](W) writer structure"]
impl crate::Writable for SvsctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SVSCTL to value 0"]
impl crate::Resettable for SvsctlSpec {}
