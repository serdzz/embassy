#[doc = "Register `SDHSCTL1` reader"]
pub type R = crate::R<Sdhsctl1Spec>;
#[doc = "Register `SDHSCTL1` writer"]
pub type W = crate::W<Sdhsctl1Spec>;
#[doc = "Over Sampling Rate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osr {
    #[doc = "0: 10"]
    Osr0 = 0,
    #[doc = "1: 20"]
    Osr1 = 1,
    #[doc = "2: 40"]
    Osr2 = 2,
    #[doc = "3: 80"]
    Osr3 = 3,
    #[doc = "4: 160"]
    Osr4 = 4,
}
impl From<Osr> for u8 {
    #[inline(always)]
    fn from(variant: Osr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osr {
    type Ux = u8;
}
impl crate::IsEnum for Osr {}
#[doc = "Field `OSR` reader - Over Sampling Rate."]
pub type OsrR = crate::FieldReader<Osr>;
impl OsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osr> {
        match self.bits {
            0 => Some(Osr::Osr0),
            1 => Some(Osr::Osr1),
            2 => Some(Osr::Osr2),
            3 => Some(Osr::Osr3),
            4 => Some(Osr::Osr4),
            _ => None,
        }
    }
    #[doc = "10"]
    #[inline(always)]
    pub fn is_osr_0(&self) -> bool {
        *self == Osr::Osr0
    }
    #[doc = "20"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == Osr::Osr1
    }
    #[doc = "40"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == Osr::Osr2
    }
    #[doc = "80"]
    #[inline(always)]
    pub fn is_osr_3(&self) -> bool {
        *self == Osr::Osr3
    }
    #[doc = "160"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == Osr::Osr4
    }
}
#[doc = "Field `OSR` writer - Over Sampling Rate."]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Osr>;
impl<'a, REG> OsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "10"]
    #[inline(always)]
    pub fn osr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr0)
    }
    #[doc = "20"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr1)
    }
    #[doc = "40"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr2)
    }
    #[doc = "80"]
    #[inline(always)]
    pub fn osr_3(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr3)
    }
    #[doc = "160"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut crate::W<REG> {
        self.variant(Osr::Osr4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Over Sampling Rate."]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Over Sampling Rate."]
    #[inline(always)]
    pub fn osr(&mut self) -> OsrW<'_, Sdhsctl1Spec> {
        OsrW::new(self, 0)
    }
}
#[doc = "SDHS Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdhsctl1Spec;
impl crate::RegisterSpec for Sdhsctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsctl1::R`](R) reader structure"]
impl crate::Readable for Sdhsctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhsctl1::W`](W) writer structure"]
impl crate::Writable for Sdhsctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSCTL1 to value 0"]
impl crate::Resettable for Sdhsctl1Spec {}
