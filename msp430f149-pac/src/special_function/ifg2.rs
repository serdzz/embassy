#[doc = "Register `IFG2` reader"]
pub type R = crate::R<Ifg2Spec>;
#[doc = "Register `IFG2` writer"]
pub type W = crate::W<Ifg2Spec>;
#[doc = "Field `URXIFG1` reader - URXIFG1"]
pub type Urxifg1R = crate::BitReader;
#[doc = "Field `URXIFG1` writer - URXIFG1"]
pub type Urxifg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTXIFG1` reader - UTXIFG1"]
pub type Utxifg1R = crate::BitReader;
#[doc = "Field `UTXIFG1` writer - UTXIFG1"]
pub type Utxifg1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - URXIFG1"]
    #[inline(always)]
    pub fn urxifg1(&self) -> Urxifg1R {
        Urxifg1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UTXIFG1"]
    #[inline(always)]
    pub fn utxifg1(&self) -> Utxifg1R {
        Utxifg1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - URXIFG1"]
    #[inline(always)]
    pub fn urxifg1(&mut self) -> Urxifg1W<'_, Ifg2Spec> {
        Urxifg1W::new(self, 4)
    }
    #[doc = "Bit 5 - UTXIFG1"]
    #[inline(always)]
    pub fn utxifg1(&mut self) -> Utxifg1W<'_, Ifg2Spec> {
        Utxifg1W::new(self, 5)
    }
}
#[doc = "Interrupt Flag 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifg2Spec;
impl crate::RegisterSpec for Ifg2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifg2::R`](R) reader structure"]
impl crate::Readable for Ifg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ifg2::W`](W) writer structure"]
impl crate::Writable for Ifg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFG2 to value 0"]
impl crate::Resettable for Ifg2Spec {}
