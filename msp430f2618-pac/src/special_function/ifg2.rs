#[doc = "Register `IFG2` reader"]
pub type R = crate::R<Ifg2Spec>;
#[doc = "Register `IFG2` writer"]
pub type W = crate::W<Ifg2Spec>;
#[doc = "Field `UCA0RXIFG` reader - UCA0RXIFG"]
pub type Uca0rxifgR = crate::BitReader;
#[doc = "Field `UCA0RXIFG` writer - UCA0RXIFG"]
pub type Uca0rxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA0TXIFG` reader - UCA0TXIFG"]
pub type Uca0txifgR = crate::BitReader;
#[doc = "Field `UCA0TXIFG` writer - UCA0TXIFG"]
pub type Uca0txifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0RXIFG` reader - UCB0RXIFG"]
pub type Ucb0rxifgR = crate::BitReader;
#[doc = "Field `UCB0RXIFG` writer - UCB0RXIFG"]
pub type Ucb0rxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0TXIFG` reader - UCB0TXIFG"]
pub type Ucb0txifgR = crate::BitReader;
#[doc = "Field `UCB0TXIFG` writer - UCB0TXIFG"]
pub type Ucb0txifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&self) -> Uca0rxifgR {
        Uca0rxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&self) -> Uca0txifgR {
        Uca0txifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&self) -> Ucb0rxifgR {
        Ucb0rxifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&self) -> Ucb0txifgR {
        Ucb0txifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&mut self) -> Uca0rxifgW<'_, Ifg2Spec> {
        Uca0rxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&mut self) -> Uca0txifgW<'_, Ifg2Spec> {
        Uca0txifgW::new(self, 1)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&mut self) -> Ucb0rxifgW<'_, Ifg2Spec> {
        Ucb0rxifgW::new(self, 2)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&mut self) -> Ucb0txifgW<'_, Ifg2Spec> {
        Ucb0txifgW::new(self, 3)
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
