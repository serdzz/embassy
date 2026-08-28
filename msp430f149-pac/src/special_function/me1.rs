#[doc = "Register `ME1` reader"]
pub type R = crate::R<Me1Spec>;
#[doc = "Register `ME1` writer"]
pub type W = crate::W<Me1Spec>;
#[doc = "Field `URXE0` reader - URXE0"]
pub type Urxe0R = crate::BitReader;
#[doc = "Field `URXE0` writer - URXE0"]
pub type Urxe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTXE0` reader - UTXE0"]
pub type Utxe0R = crate::BitReader;
#[doc = "Field `UTXE0` writer - UTXE0"]
pub type Utxe0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - URXE0"]
    #[inline(always)]
    pub fn urxe0(&self) -> Urxe0R {
        Urxe0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UTXE0"]
    #[inline(always)]
    pub fn utxe0(&self) -> Utxe0R {
        Utxe0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - URXE0"]
    #[inline(always)]
    pub fn urxe0(&mut self) -> Urxe0W<'_, Me1Spec> {
        Urxe0W::new(self, 6)
    }
    #[doc = "Bit 7 - UTXE0"]
    #[inline(always)]
    pub fn utxe0(&mut self) -> Utxe0W<'_, Me1Spec> {
        Utxe0W::new(self, 7)
    }
}
#[doc = "Module Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`me1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`me1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Me1Spec;
impl crate::RegisterSpec for Me1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`me1::R`](R) reader structure"]
impl crate::Readable for Me1Spec {}
#[doc = "`write(|w| ..)` method takes [`me1::W`](W) writer structure"]
impl crate::Writable for Me1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ME1 to value 0"]
impl crate::Resettable for Me1Spec {}
