#[doc = "Register `IE2` reader"]
pub type R = crate::R<Ie2Spec>;
#[doc = "Register `IE2` writer"]
pub type W = crate::W<Ie2Spec>;
#[doc = "Field `UCA0RXIE` reader - UCA0RXIE"]
pub type Uca0rxieR = crate::BitReader;
#[doc = "Field `UCA0RXIE` writer - UCA0RXIE"]
pub type Uca0rxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA0TXIE` reader - UCA0TXIE"]
pub type Uca0txieR = crate::BitReader;
#[doc = "Field `UCA0TXIE` writer - UCA0TXIE"]
pub type Uca0txieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0RXIE` reader - UCB0RXIE"]
pub type Ucb0rxieR = crate::BitReader;
#[doc = "Field `UCB0RXIE` writer - UCB0RXIE"]
pub type Ucb0rxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB0TXIE` reader - UCB0TXIE"]
pub type Ucb0txieR = crate::BitReader;
#[doc = "Field `UCB0TXIE` writer - UCB0TXIE"]
pub type Ucb0txieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&self) -> Uca0rxieR {
        Uca0rxieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&self) -> Uca0txieR {
        Uca0txieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&self) -> Ucb0rxieR {
        Ucb0rxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&self) -> Ucb0txieR {
        Ucb0txieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&mut self) -> Uca0rxieW<'_, Ie2Spec> {
        Uca0rxieW::new(self, 0)
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&mut self) -> Uca0txieW<'_, Ie2Spec> {
        Uca0txieW::new(self, 1)
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&mut self) -> Ucb0rxieW<'_, Ie2Spec> {
        Ucb0rxieW::new(self, 2)
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&mut self) -> Ucb0txieW<'_, Ie2Spec> {
        Ucb0txieW::new(self, 3)
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
