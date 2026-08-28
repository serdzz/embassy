#[doc = "Register `UC1IFG` reader"]
pub type R = crate::R<Uc1ifgSpec>;
#[doc = "Register `UC1IFG` writer"]
pub type W = crate::W<Uc1ifgSpec>;
#[doc = "Field `UCA1RXIFG` reader - UCA1RXIFG"]
pub type Uca1rxifgR = crate::BitReader;
#[doc = "Field `UCA1RXIFG` writer - UCA1RXIFG"]
pub type Uca1rxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCA1TXIFG` reader - UCA1TXIFG"]
pub type Uca1txifgR = crate::BitReader;
#[doc = "Field `UCA1TXIFG` writer - UCA1TXIFG"]
pub type Uca1txifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB1RXIFG` reader - UCB1RXIFG"]
pub type Ucb1rxifgR = crate::BitReader;
#[doc = "Field `UCB1RXIFG` writer - UCB1RXIFG"]
pub type Ucb1rxifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCB1TXIFG` reader - UCB1TXIFG"]
pub type Ucb1txifgR = crate::BitReader;
#[doc = "Field `UCB1TXIFG` writer - UCB1TXIFG"]
pub type Ucb1txifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UCA1RXIFG"]
    #[inline(always)]
    pub fn uca1rxifg(&self) -> Uca1rxifgR {
        Uca1rxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA1TXIFG"]
    #[inline(always)]
    pub fn uca1txifg(&self) -> Uca1txifgR {
        Uca1txifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB1RXIFG"]
    #[inline(always)]
    pub fn ucb1rxifg(&self) -> Ucb1rxifgR {
        Ucb1rxifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB1TXIFG"]
    #[inline(always)]
    pub fn ucb1txifg(&self) -> Ucb1txifgR {
        Ucb1txifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA1RXIFG"]
    #[inline(always)]
    pub fn uca1rxifg(&mut self) -> Uca1rxifgW<'_, Uc1ifgSpec> {
        Uca1rxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - UCA1TXIFG"]
    #[inline(always)]
    pub fn uca1txifg(&mut self) -> Uca1txifgW<'_, Uc1ifgSpec> {
        Uca1txifgW::new(self, 1)
    }
    #[doc = "Bit 2 - UCB1RXIFG"]
    #[inline(always)]
    pub fn ucb1rxifg(&mut self) -> Ucb1rxifgW<'_, Uc1ifgSpec> {
        Ucb1rxifgW::new(self, 2)
    }
    #[doc = "Bit 3 - UCB1TXIFG"]
    #[inline(always)]
    pub fn ucb1txifg(&mut self) -> Ucb1txifgW<'_, Uc1ifgSpec> {
        Ucb1txifgW::new(self, 3)
    }
}
#[doc = "ISCI 1 Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`uc1ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uc1ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uc1ifgSpec;
impl crate::RegisterSpec for Uc1ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uc1ifg::R`](R) reader structure"]
impl crate::Readable for Uc1ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`uc1ifg::W`](W) writer structure"]
impl crate::Writable for Uc1ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UC1IFG to value 0"]
impl crate::Resettable for Uc1ifgSpec {}
