#[doc = "Register `UCA1IRTCTL` reader"]
pub type R = crate::R<Uca1irtctlSpec>;
#[doc = "Register `UCA1IRTCTL` writer"]
pub type W = crate::W<Uca1irtctlSpec>;
#[doc = "Field `UCIREN` reader - IRDA Encoder/Decoder enable"]
pub type UcirenR = crate::BitReader;
#[doc = "Field `UCIREN` writer - IRDA Encoder/Decoder enable"]
pub type UcirenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXCLK` reader - IRDA Transmit Pulse Clock Select"]
pub type UcirtxclkR = crate::BitReader;
#[doc = "Field `UCIRTXCLK` writer - IRDA Transmit Pulse Clock Select"]
pub type UcirtxclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL0` reader - IRDA Transmit Pulse Length 0"]
pub type Ucirtxpl0R = crate::BitReader;
#[doc = "Field `UCIRTXPL0` writer - IRDA Transmit Pulse Length 0"]
pub type Ucirtxpl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL1` reader - IRDA Transmit Pulse Length 1"]
pub type Ucirtxpl1R = crate::BitReader;
#[doc = "Field `UCIRTXPL1` writer - IRDA Transmit Pulse Length 1"]
pub type Ucirtxpl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL2` reader - IRDA Transmit Pulse Length 2"]
pub type Ucirtxpl2R = crate::BitReader;
#[doc = "Field `UCIRTXPL2` writer - IRDA Transmit Pulse Length 2"]
pub type Ucirtxpl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL3` reader - IRDA Transmit Pulse Length 3"]
pub type Ucirtxpl3R = crate::BitReader;
#[doc = "Field `UCIRTXPL3` writer - IRDA Transmit Pulse Length 3"]
pub type Ucirtxpl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL4` reader - IRDA Transmit Pulse Length 4"]
pub type Ucirtxpl4R = crate::BitReader;
#[doc = "Field `UCIRTXPL4` writer - IRDA Transmit Pulse Length 4"]
pub type Ucirtxpl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRTXPL5` reader - IRDA Transmit Pulse Length 5"]
pub type Ucirtxpl5R = crate::BitReader;
#[doc = "Field `UCIRTXPL5` writer - IRDA Transmit Pulse Length 5"]
pub type Ucirtxpl5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UcirenR {
        UcirenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UcirtxclkR {
        UcirtxclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl0(&self) -> Ucirtxpl0R {
        Ucirtxpl0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    pub fn ucirtxpl1(&self) -> Ucirtxpl1R {
        Ucirtxpl1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    pub fn ucirtxpl2(&self) -> Ucirtxpl2R {
        Ucirtxpl2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    pub fn ucirtxpl3(&self) -> Ucirtxpl3R {
        Ucirtxpl3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    pub fn ucirtxpl4(&self) -> Ucirtxpl4R {
        Ucirtxpl4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    pub fn ucirtxpl5(&self) -> Ucirtxpl5R {
        Ucirtxpl5R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UcirenW<'_, Uca1irtctlSpec> {
        UcirenW::new(self, 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UcirtxclkW<'_, Uca1irtctlSpec> {
        UcirtxclkW::new(self, 1)
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl0(&mut self) -> Ucirtxpl0W<'_, Uca1irtctlSpec> {
        Ucirtxpl0W::new(self, 2)
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    pub fn ucirtxpl1(&mut self) -> Ucirtxpl1W<'_, Uca1irtctlSpec> {
        Ucirtxpl1W::new(self, 3)
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    pub fn ucirtxpl2(&mut self) -> Ucirtxpl2W<'_, Uca1irtctlSpec> {
        Ucirtxpl2W::new(self, 4)
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    pub fn ucirtxpl3(&mut self) -> Ucirtxpl3W<'_, Uca1irtctlSpec> {
        Ucirtxpl3W::new(self, 5)
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    pub fn ucirtxpl4(&mut self) -> Ucirtxpl4W<'_, Uca1irtctlSpec> {
        Ucirtxpl4W::new(self, 6)
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    pub fn ucirtxpl5(&mut self) -> Ucirtxpl5W<'_, Uca1irtctlSpec> {
        Ucirtxpl5W::new(self, 7)
    }
}
#[doc = "USCI A1 IrDA Transmit Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1irtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1irtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1irtctlSpec;
impl crate::RegisterSpec for Uca1irtctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1irtctl::R`](R) reader structure"]
impl crate::Readable for Uca1irtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1irtctl::W`](W) writer structure"]
impl crate::Writable for Uca1irtctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets UCA1IRTCTL to value 0"]
impl crate::Resettable for Uca1irtctlSpec {}
