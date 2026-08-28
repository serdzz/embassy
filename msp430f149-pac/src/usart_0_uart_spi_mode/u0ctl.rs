#[doc = "Register `U0CTL` reader"]
pub type R = crate::R<U0ctlSpec>;
#[doc = "Register `U0CTL` writer"]
pub type W = crate::W<U0ctlSpec>;
#[doc = "Field `SWRST` reader - USART Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - USART Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MM` reader - Master Mode off/on"]
pub type MmR = crate::BitReader;
#[doc = "Field `MM` writer - Master Mode off/on"]
pub type MmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC` reader - UART / SPI mode"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - UART / SPI mode"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LISTEN` reader - Listen mode"]
pub type ListenR = crate::BitReader;
#[doc = "Field `LISTEN` writer - Listen mode"]
pub type ListenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAR` reader - Data 0:7-bits / 1:8-bits"]
pub type CharR = crate::BitReader;
#[doc = "Field `CHAR` writer - Data 0:7-bits / 1:8-bits"]
pub type CharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPB` reader - Stop Bits 0:one / 1: two"]
pub type SpbR = crate::BitReader;
#[doc = "Field `SPB` writer - Stop Bits 0:one / 1: two"]
pub type SpbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEV` reader - Parity 0:odd / 1:even"]
pub type PevR = crate::BitReader;
#[doc = "Field `PEV` writer - Parity 0:odd / 1:even"]
pub type PevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENA` reader - Parity enable"]
pub type PenaR = crate::BitReader;
#[doc = "Field `PENA` writer - Parity enable"]
pub type PenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Mode off/on"]
    #[inline(always)]
    pub fn mm(&self) -> MmR {
        MmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART / SPI mode"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Listen mode"]
    #[inline(always)]
    pub fn listen(&self) -> ListenR {
        ListenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data 0:7-bits / 1:8-bits"]
    #[inline(always)]
    pub fn char(&self) -> CharR {
        CharR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn spb(&self) -> SpbR {
        SpbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn pev(&self) -> PevR {
        PevR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity enable"]
    #[inline(always)]
    pub fn pena(&self) -> PenaR {
        PenaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, U0ctlSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Mode off/on"]
    #[inline(always)]
    pub fn mm(&mut self) -> MmW<'_, U0ctlSpec> {
        MmW::new(self, 1)
    }
    #[doc = "Bit 2 - UART / SPI mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, U0ctlSpec> {
        SyncW::new(self, 2)
    }
    #[doc = "Bit 3 - Listen mode"]
    #[inline(always)]
    pub fn listen(&mut self) -> ListenW<'_, U0ctlSpec> {
        ListenW::new(self, 3)
    }
    #[doc = "Bit 4 - Data 0:7-bits / 1:8-bits"]
    #[inline(always)]
    pub fn char(&mut self) -> CharW<'_, U0ctlSpec> {
        CharW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn spb(&mut self) -> SpbW<'_, U0ctlSpec> {
        SpbW::new(self, 5)
    }
    #[doc = "Bit 6 - Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn pev(&mut self) -> PevW<'_, U0ctlSpec> {
        PevW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity enable"]
    #[inline(always)]
    pub fn pena(&mut self) -> PenaW<'_, U0ctlSpec> {
        PenaW::new(self, 7)
    }
}
#[doc = "USART 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`u0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0ctlSpec;
impl crate::RegisterSpec for U0ctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`u0ctl::R`](R) reader structure"]
impl crate::Readable for U0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`u0ctl::W`](W) writer structure"]
impl crate::Writable for U0ctlSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets U0CTL to value 0"]
impl crate::Resettable for U0ctlSpec {}
