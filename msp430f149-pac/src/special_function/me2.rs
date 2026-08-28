#[doc = "Register `ME2` reader"]
pub type R = crate::R<Me2Spec>;
#[doc = "Register `ME2` writer"]
pub type W = crate::W<Me2Spec>;
#[doc = "Field `URXE1` reader - URXE1"]
pub type Urxe1R = crate::BitReader;
#[doc = "Field `URXE1` writer - URXE1"]
pub type Urxe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTXE1` reader - UTXE1"]
pub type Utxe1R = crate::BitReader;
#[doc = "Field `UTXE1` writer - UTXE1"]
pub type Utxe1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - URXE1"]
    #[inline(always)]
    pub fn urxe1(&self) -> Urxe1R {
        Urxe1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UTXE1"]
    #[inline(always)]
    pub fn utxe1(&self) -> Utxe1R {
        Utxe1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - URXE1"]
    #[inline(always)]
    pub fn urxe1(&mut self) -> Urxe1W<'_, Me2Spec> {
        Urxe1W::new(self, 4)
    }
    #[doc = "Bit 5 - UTXE1"]
    #[inline(always)]
    pub fn utxe1(&mut self) -> Utxe1W<'_, Me2Spec> {
        Utxe1W::new(self, 5)
    }
}
#[doc = "Module Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`me2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`me2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Me2Spec;
impl crate::RegisterSpec for Me2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`me2::R`](R) reader structure"]
impl crate::Readable for Me2Spec {}
#[doc = "`write(|w| ..)` method takes [`me2::W`](W) writer structure"]
impl crate::Writable for Me2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ME2 to value 0"]
impl crate::Resettable for Me2Spec {}
