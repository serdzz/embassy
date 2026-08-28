#[doc = "Register `IFG1` reader"]
pub type R = crate::R<Ifg1Spec>;
#[doc = "Register `IFG1` writer"]
pub type W = crate::W<Ifg1Spec>;
#[doc = "Field `WDTIFG` reader - WDTIFG"]
pub type WdtifgR = crate::BitReader;
#[doc = "Field `WDTIFG` writer - WDTIFG"]
pub type WdtifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIFG` reader - OFIFG"]
pub type OfifgR = crate::BitReader;
#[doc = "Field `OFIFG` writer - OFIFG"]
pub type OfifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIFG` reader - NMIIFG"]
pub type NmiifgR = crate::BitReader;
#[doc = "Field `NMIIFG` writer - NMIIFG"]
pub type NmiifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URXIFG0` reader - URXIFG0"]
pub type Urxifg0R = crate::BitReader;
#[doc = "Field `URXIFG0` writer - URXIFG0"]
pub type Urxifg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTXIFG0` reader - UTXIFG0"]
pub type Utxifg0R = crate::BitReader;
#[doc = "Field `UTXIFG0` writer - UTXIFG0"]
pub type Utxifg0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDTIFG"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WdtifgR {
        WdtifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OFIFG"]
    #[inline(always)]
    pub fn ofifg(&self) -> OfifgR {
        OfifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NMIIFG"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NmiifgR {
        NmiifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - URXIFG0"]
    #[inline(always)]
    pub fn urxifg0(&self) -> Urxifg0R {
        Urxifg0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UTXIFG0"]
    #[inline(always)]
    pub fn utxifg0(&self) -> Utxifg0R {
        Utxifg0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTIFG"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WdtifgW<'_, Ifg1Spec> {
        WdtifgW::new(self, 0)
    }
    #[doc = "Bit 1 - OFIFG"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OfifgW<'_, Ifg1Spec> {
        OfifgW::new(self, 1)
    }
    #[doc = "Bit 4 - NMIIFG"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NmiifgW<'_, Ifg1Spec> {
        NmiifgW::new(self, 4)
    }
    #[doc = "Bit 6 - URXIFG0"]
    #[inline(always)]
    pub fn urxifg0(&mut self) -> Urxifg0W<'_, Ifg1Spec> {
        Urxifg0W::new(self, 6)
    }
    #[doc = "Bit 7 - UTXIFG0"]
    #[inline(always)]
    pub fn utxifg0(&mut self) -> Utxifg0W<'_, Ifg1Spec> {
        Utxifg0W::new(self, 7)
    }
}
#[doc = "Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ifg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifg1Spec;
impl crate::RegisterSpec for Ifg1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifg1::R`](R) reader structure"]
impl crate::Readable for Ifg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ifg1::W`](W) writer structure"]
impl crate::Writable for Ifg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFG1 to value 0"]
impl crate::Resettable for Ifg1Spec {}
