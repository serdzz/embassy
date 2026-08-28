#[doc = "Register `UC1IE` reader"]
pub type R = crate::R<Uc1ieSpec>;
#[doc = "Register `UC1IE` writer"]
pub type W = crate::W<Uc1ieSpec>;
#[doc = "Field `UCA1RXIE` reader - UCA1RXIE"]
pub type Uca1rxieR = crate::BitReader;
#[doc = "Field `UCA1RXIE` writer - UCA1RXIE"]
pub type Uca1rxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA1TXIE` reader - UCA1TXIE"]
pub type Uca1txieR = crate::BitReader;
#[doc = "Field `UCA1TXIE` writer - UCA1TXIE"]
pub type Uca1txieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB1RXIE` reader - UCB1RXIE"]
pub type Ucb1rxieR = crate::BitReader;
#[doc = "Field `UCB1RXIE` writer - UCB1RXIE"]
pub type Ucb1rxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB1TXIE` reader - UCB1TXIE"]
pub type Ucb1txieR = crate::BitReader;
#[doc = "Field `UCB1TXIE` writer - UCB1TXIE"]
pub type Ucb1txieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UCA1RXIE"]
    #[inline(always)]
    pub fn uca1rxie(&self) -> Uca1rxieR {
        Uca1rxieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA1TXIE"]
    #[inline(always)]
    pub fn uca1txie(&self) -> Uca1txieR {
        Uca1txieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB1RXIE"]
    #[inline(always)]
    pub fn ucb1rxie(&self) -> Ucb1rxieR {
        Ucb1rxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB1TXIE"]
    #[inline(always)]
    pub fn ucb1txie(&self) -> Ucb1txieR {
        Ucb1txieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA1RXIE"]
    #[inline(always)]
    pub fn uca1rxie(&mut self) -> Uca1rxieW<'_, Uc1ieSpec> {
        Uca1rxieW::new(self, 0)
    }
    #[doc = "Bit 1 - UCA1TXIE"]
    #[inline(always)]
    pub fn uca1txie(&mut self) -> Uca1txieW<'_, Uc1ieSpec> {
        Uca1txieW::new(self, 1)
    }
    #[doc = "Bit 2 - UCB1RXIE"]
    #[inline(always)]
    pub fn ucb1rxie(&mut self) -> Ucb1rxieW<'_, Uc1ieSpec> {
        Ucb1rxieW::new(self, 2)
    }
    #[doc = "Bit 3 - UCB1TXIE"]
    #[inline(always)]
    pub fn ucb1txie(&mut self) -> Ucb1txieW<'_, Uc1ieSpec> {
        Ucb1txieW::new(self, 3)
    }
}
#[doc = "USCI 1 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uc1ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uc1ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uc1ieSpec;
impl crate::RegisterSpec for Uc1ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uc1ie::R`](R) reader structure"]
impl crate::Readable for Uc1ieSpec {}
#[doc = "`write(|w| ..)` method takes [`uc1ie::W`](W) writer structure"]
impl crate::Writable for Uc1ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UC1IE to value 0"]
impl crate::Resettable for Uc1ieSpec {}
