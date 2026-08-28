#[doc = "Register `UCA1IRRCTL` reader"]
pub type R = crate::R<Uca1irrctlSpec>;
#[doc = "Register `UCA1IRRCTL` writer"]
pub type W = crate::W<Uca1irrctlSpec>;
#[doc = "Field `UCIRRXFE` reader - IRDA Receive Filter enable"]
pub type UcirrxfeR = crate::BitReader;
#[doc = "Field `UCIRRXFE` writer - IRDA Receive Filter enable"]
pub type UcirrxfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXPL` reader - IRDA Receive Input Polarity"]
pub type UcirrxplR = crate::BitReader;
#[doc = "Field `UCIRRXPL` writer - IRDA Receive Input Polarity"]
pub type UcirrxplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL0` reader - IRDA Receive Filter Length 0"]
pub type Ucirrxfl0R = crate::BitReader;
#[doc = "Field `UCIRRXFL0` writer - IRDA Receive Filter Length 0"]
pub type Ucirrxfl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL1` reader - IRDA Receive Filter Length 1"]
pub type Ucirrxfl1R = crate::BitReader;
#[doc = "Field `UCIRRXFL1` writer - IRDA Receive Filter Length 1"]
pub type Ucirrxfl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL2` reader - IRDA Receive Filter Length 2"]
pub type Ucirrxfl2R = crate::BitReader;
#[doc = "Field `UCIRRXFL2` writer - IRDA Receive Filter Length 2"]
pub type Ucirrxfl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL3` reader - IRDA Receive Filter Length 3"]
pub type Ucirrxfl3R = crate::BitReader;
#[doc = "Field `UCIRRXFL3` writer - IRDA Receive Filter Length 3"]
pub type Ucirrxfl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL4` reader - IRDA Receive Filter Length 4"]
pub type Ucirrxfl4R = crate::BitReader;
#[doc = "Field `UCIRRXFL4` writer - IRDA Receive Filter Length 4"]
pub type Ucirrxfl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCIRRXFL5` reader - IRDA Receive Filter Length 5"]
pub type Ucirrxfl5R = crate::BitReader;
#[doc = "Field `UCIRRXFL5` writer - IRDA Receive Filter Length 5"]
pub type Ucirrxfl5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UcirrxfeR {
        UcirrxfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UcirrxplR {
        UcirrxplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&self) -> Ucirrxfl0R {
        Ucirrxfl0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&self) -> Ucirrxfl1R {
        Ucirrxfl1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&self) -> Ucirrxfl2R {
        Ucirrxfl2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&self) -> Ucirrxfl3R {
        Ucirrxfl3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&self) -> Ucirrxfl4R {
        Ucirrxfl4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&self) -> Ucirrxfl5R {
        Ucirrxfl5R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UcirrxfeW<'_, Uca1irrctlSpec> {
        UcirrxfeW::new(self, 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UcirrxplW<'_, Uca1irrctlSpec> {
        UcirrxplW::new(self, 1)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&mut self) -> Ucirrxfl0W<'_, Uca1irrctlSpec> {
        Ucirrxfl0W::new(self, 2)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&mut self) -> Ucirrxfl1W<'_, Uca1irrctlSpec> {
        Ucirrxfl1W::new(self, 3)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&mut self) -> Ucirrxfl2W<'_, Uca1irrctlSpec> {
        Ucirrxfl2W::new(self, 4)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&mut self) -> Ucirrxfl3W<'_, Uca1irrctlSpec> {
        Ucirrxfl3W::new(self, 5)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&mut self) -> Ucirrxfl4W<'_, Uca1irrctlSpec> {
        Ucirrxfl4W::new(self, 6)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&mut self) -> Ucirrxfl5W<'_, Uca1irrctlSpec> {
        Ucirrxfl5W::new(self, 7)
    }
}
#[doc = "USCI A1 IrDA Receive Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1irrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1irrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1irrctlSpec;
impl crate::RegisterSpec for Uca1irrctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1irrctl::R`](R) reader structure"]
impl crate::Readable for Uca1irrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca1irrctl::W`](W) writer structure"]
impl crate::Writable for Uca1irrctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets UCA1IRRCTL to value 0"]
impl crate::Resettable for Uca1irrctlSpec {}
