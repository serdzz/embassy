#[doc = "Register `UCB0CTL1_SPI` reader"]
pub type R = crate::R<Ucb0ctl1SpiSpec>;
#[doc = "Register `UCB0CTL1_SPI` writer"]
pub type W = crate::W<Ucb0ctl1SpiSpec>;
#[doc = "Field `UCSWRST` reader - USCI Software Reset"]
pub type UcswrstR = crate::BitReader;
#[doc = "Field `UCSWRST` writer - USCI Software Reset"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USCI 1 Clock Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucssel {
    #[doc = "0: USCI 0 Clock Source: 0"]
    Ucssel0 = 0,
    #[doc = "1: USCI 0 Clock Source: 1"]
    Ucssel1 = 1,
    #[doc = "2: USCI 0 Clock Source: 2"]
    Ucssel2 = 2,
    #[doc = "3: USCI 0 Clock Source: 3"]
    Ucssel3 = 3,
}
impl From<Ucssel> for u8 {
    #[inline(always)]
    fn from(variant: Ucssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucssel {
    type Ux = u8;
}
impl crate::IsEnum for Ucssel {}
#[doc = "Field `UCSSEL` reader - USCI 1 Clock Source Select 1"]
pub type UcsselR = crate::FieldReader<Ucssel>;
impl UcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucssel {
        match self.bits {
            0 => Ucssel::Ucssel0,
            1 => Ucssel::Ucssel1,
            2 => Ucssel::Ucssel2,
            3 => Ucssel::Ucssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == Ucssel::Ucssel0
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        *self == Ucssel::Ucssel1
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        *self == Ucssel::Ucssel2
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == Ucssel::Ucssel3
    }
}
#[doc = "Field `UCSSEL` writer - USCI 1 Clock Source Select 1"]
pub type UcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucssel, crate::Safe>;
impl<'a, REG> UcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel0)
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn ucssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel1)
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn ucssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel2)
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel3)
    }
}
impl R {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:7 - USCI 1 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&self) -> UcsselR {
        UcsselR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<'_, Ucb0ctl1SpiSpec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bits 6:7 - USCI 1 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<'_, Ucb0ctl1SpiSpec> {
        UcsselW::new(self, 6)
    }
}
#[doc = "USCI B0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ctl1_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ctl1_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ctl1SpiSpec;
impl crate::RegisterSpec for Ucb0ctl1SpiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0ctl1_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0ctl1SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctl1_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0ctl1SpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0CTL1_SPI to value 0"]
impl crate::Resettable for Ucb0ctl1SpiSpec {}
