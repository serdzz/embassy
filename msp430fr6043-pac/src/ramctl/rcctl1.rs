#[doc = "Register `RCCTL1` reader"]
pub type R = crate::R<Rcctl1Spec>;
#[doc = "Register `RCCTL1` writer"]
pub type W = crate::W<Rcctl1Spec>;
#[doc = "DACCESS Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daccessifg {
    #[doc = "0: DACCESS Interrupt is not pending"]
    Daccessifg0 = 0,
    #[doc = "1: DACCESS Interrupt is pending."]
    Daccessifg1 = 1,
}
impl From<Daccessifg> for bool {
    #[inline(always)]
    fn from(variant: Daccessifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCESSIFG` reader - DACCESS Interrupt Flag"]
pub type DaccessifgR = crate::BitReader<Daccessifg>;
impl DaccessifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daccessifg {
        match self.bits {
            false => Daccessifg::Daccessifg0,
            true => Daccessifg::Daccessifg1,
        }
    }
    #[doc = "DACCESS Interrupt is not pending"]
    #[inline(always)]
    pub fn is_daccessifg_0(&self) -> bool {
        *self == Daccessifg::Daccessifg0
    }
    #[doc = "DACCESS Interrupt is pending."]
    #[inline(always)]
    pub fn is_daccessifg_1(&self) -> bool {
        *self == Daccessifg::Daccessifg1
    }
}
#[doc = "Field `DACCESSIFG` writer - DACCESS Interrupt Flag"]
pub type DaccessifgW<'a, REG> = crate::BitWriter<'a, REG, Daccessifg>;
impl<'a, REG> DaccessifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DACCESS Interrupt is not pending"]
    #[inline(always)]
    pub fn daccessifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Daccessifg::Daccessifg0)
    }
    #[doc = "DACCESS Interrupt is pending."]
    #[inline(always)]
    pub fn daccessifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Daccessifg::Daccessifg1)
    }
}
#[doc = "DACCESS Bus Error NMI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daccessie {
    #[doc = "0: Disable NMI for DACCESS Interrupt"]
    Daccessie0 = 0,
    #[doc = "1: Enable NMI for DACCESS Interrupt"]
    Daccessie1 = 1,
}
impl From<Daccessie> for bool {
    #[inline(always)]
    fn from(variant: Daccessie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCESSIE` reader - DACCESS Bus Error NMI enable"]
pub type DaccessieR = crate::BitReader<Daccessie>;
impl DaccessieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daccessie {
        match self.bits {
            false => Daccessie::Daccessie0,
            true => Daccessie::Daccessie1,
        }
    }
    #[doc = "Disable NMI for DACCESS Interrupt"]
    #[inline(always)]
    pub fn is_daccessie_0(&self) -> bool {
        *self == Daccessie::Daccessie0
    }
    #[doc = "Enable NMI for DACCESS Interrupt"]
    #[inline(always)]
    pub fn is_daccessie_1(&self) -> bool {
        *self == Daccessie::Daccessie1
    }
}
#[doc = "Field `DACCESSIE` writer - DACCESS Bus Error NMI enable"]
pub type DaccessieW<'a, REG> = crate::BitWriter<'a, REG, Daccessie>;
impl<'a, REG> DaccessieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable NMI for DACCESS Interrupt"]
    #[inline(always)]
    pub fn daccessie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Daccessie::Daccessie0)
    }
    #[doc = "Enable NMI for DACCESS Interrupt"]
    #[inline(always)]
    pub fn daccessie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Daccessie::Daccessie1)
    }
}
impl R {
    #[doc = "Bit 0 - DACCESS Interrupt Flag"]
    #[inline(always)]
    pub fn daccessifg(&self) -> DaccessifgR {
        DaccessifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - DACCESS Bus Error NMI enable"]
    #[inline(always)]
    pub fn daccessie(&self) -> DaccessieR {
        DaccessieR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DACCESS Interrupt Flag"]
    #[inline(always)]
    pub fn daccessifg(&mut self) -> DaccessifgW<'_, Rcctl1Spec> {
        DaccessifgW::new(self, 0)
    }
    #[doc = "Bit 8 - DACCESS Bus Error NMI enable"]
    #[inline(always)]
    pub fn daccessie(&mut self) -> DaccessieW<'_, Rcctl1Spec> {
        DaccessieW::new(self, 8)
    }
}
#[doc = "RAM Controller Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcctl1Spec;
impl crate::RegisterSpec for Rcctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcctl1::R`](R) reader structure"]
impl crate::Readable for Rcctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcctl1::W`](W) writer structure"]
impl crate::Writable for Rcctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCCTL1 to value 0"]
impl crate::Resettable for Rcctl1Spec {}
