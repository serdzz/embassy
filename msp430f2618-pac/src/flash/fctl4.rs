#[doc = "Register `FCTL4` reader"]
pub type R = crate::R<Fctl4Spec>;
#[doc = "Register `FCTL4` writer"]
pub type W = crate::W<Fctl4Spec>;
#[doc = "Field `MGR0` reader - Marginal read 0 mode."]
pub type Mgr0R = crate::BitReader;
#[doc = "Field `MGR0` writer - Marginal read 0 mode."]
pub type Mgr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGR1` reader - Marginal read 1 mode."]
pub type Mgr1R = crate::BitReader;
#[doc = "Field `MGR1` writer - Marginal read 1 mode."]
pub type Mgr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    pub fn mgr0(&self) -> Mgr0R {
        Mgr0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    pub fn mgr1(&self) -> Mgr1R {
        Mgr1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Marginal read 0 mode."]
    #[inline(always)]
    pub fn mgr0(&mut self) -> Mgr0W<'_, Fctl4Spec> {
        Mgr0W::new(self, 4)
    }
    #[doc = "Bit 5 - Marginal read 1 mode."]
    #[inline(always)]
    pub fn mgr1(&mut self) -> Mgr1W<'_, Fctl4Spec> {
        Mgr1W::new(self, 5)
    }
}
#[doc = "FLASH Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctl4Spec;
impl crate::RegisterSpec for Fctl4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl4::R`](R) reader structure"]
impl crate::Readable for Fctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`fctl4::W`](W) writer structure"]
impl crate::Writable for Fctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTL4 to value 0"]
impl crate::Resettable for Fctl4Spec {}
