#[doc = "Register `IE2` reader"]
pub type R = crate::R<Ie2Spec>;
#[doc = "Register `IE2` writer"]
pub type W = crate::W<Ie2Spec>;
#[doc = "Field `URXIE1` reader - URXIE1"]
pub type Urxie1R = crate::BitReader;
#[doc = "Field `URXIE1` writer - URXIE1"]
pub type Urxie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTXIE1` reader - UTXIE1"]
pub type Utxie1R = crate::BitReader;
#[doc = "Field `UTXIE1` writer - UTXIE1"]
pub type Utxie1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - URXIE1"]
    #[inline(always)]
    pub fn urxie1(&self) -> Urxie1R {
        Urxie1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UTXIE1"]
    #[inline(always)]
    pub fn utxie1(&self) -> Utxie1R {
        Utxie1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - URXIE1"]
    #[inline(always)]
    pub fn urxie1(&mut self) -> Urxie1W<'_, Ie2Spec> {
        Urxie1W::new(self, 4)
    }
    #[doc = "Bit 5 - UTXIE1"]
    #[inline(always)]
    pub fn utxie1(&mut self) -> Utxie1W<'_, Ie2Spec> {
        Utxie1W::new(self, 5)
    }
}
#[doc = "Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ie2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ie2Spec;
impl crate::RegisterSpec for Ie2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ie2::R`](R) reader structure"]
impl crate::Readable for Ie2Spec {}
#[doc = "`write(|w| ..)` method takes [`ie2::W`](W) writer structure"]
impl crate::Writable for Ie2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IE2 to value 0"]
impl crate::Resettable for Ie2Spec {}
