#[doc = "Register `SDHSCTL4` reader"]
pub type R = crate::R<Sdhsctl4Spec>;
#[doc = "Register `SDHSCTL4` writer"]
pub type W = crate::W<Sdhsctl4Spec>;
#[doc = "SDHS Power-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdhson {
    #[doc = "0: Power down the SDHS module"]
    Sdhson0 = 0,
    #[doc = "1: Power on the SDHS module"]
    Sdhson1 = 1,
}
impl From<Sdhson> for bool {
    #[inline(always)]
    fn from(variant: Sdhson) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDHSON` reader - SDHS Power-up"]
pub type SdhsonR = crate::BitReader<Sdhson>;
impl SdhsonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdhson {
        match self.bits {
            false => Sdhson::Sdhson0,
            true => Sdhson::Sdhson1,
        }
    }
    #[doc = "Power down the SDHS module"]
    #[inline(always)]
    pub fn is_sdhson_0(&self) -> bool {
        *self == Sdhson::Sdhson0
    }
    #[doc = "Power on the SDHS module"]
    #[inline(always)]
    pub fn is_sdhson_1(&self) -> bool {
        *self == Sdhson::Sdhson1
    }
}
#[doc = "Field `SDHSON` writer - SDHS Power-up"]
pub type SdhsonW<'a, REG> = crate::BitWriter<'a, REG, Sdhson>;
impl<'a, REG> SdhsonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power down the SDHS module"]
    #[inline(always)]
    pub fn sdhson_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdhson::Sdhson0)
    }
    #[doc = "Power on the SDHS module"]
    #[inline(always)]
    pub fn sdhson_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdhson::Sdhson1)
    }
}
impl R {
    #[doc = "Bit 0 - SDHS Power-up"]
    #[inline(always)]
    pub fn sdhson(&self) -> SdhsonR {
        SdhsonR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDHS Power-up"]
    #[inline(always)]
    pub fn sdhson(&mut self) -> SdhsonW<'_, Sdhsctl4Spec> {
        SdhsonW::new(self, 0)
    }
}
#[doc = "SDHS Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl4Spec;
impl crate::RegisterSpec for Sdhsctl4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl4::R`](R) reader structure"]
impl crate::Readable for Sdhsctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl4::W`](W) writer structure"]
impl crate::Writable for Sdhsctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL4 to value 0"]
impl crate::Resettable for Sdhsctl4Spec {}
