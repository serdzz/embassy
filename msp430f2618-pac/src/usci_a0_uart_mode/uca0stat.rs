#[doc = "Register `UCA0STAT` reader"]
pub type R = crate::R<Uca0statSpec>;
#[doc = "Register `UCA0STAT` writer"]
pub type W = crate::W<Uca0statSpec>;
#[doc = "Field `UCBUSY` reader - USCI Busy Flag"]
pub type UcbusyR = crate::BitReader;
#[doc = "Field `UCBUSY` writer - USCI Busy Flag"]
pub type UcbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDR` reader - USCI Address received Flag"]
pub type UcaddrR = crate::BitReader;
#[doc = "Field `UCADDR` writer - USCI Address received Flag"]
pub type UcaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXERR` reader - USCI RX Error Flag"]
pub type UcrxerrR = crate::BitReader;
#[doc = "Field `UCRXERR` writer - USCI RX Error Flag"]
pub type UcrxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBRK` reader - USCI Break received"]
pub type UcbrkR = crate::BitReader;
#[doc = "Field `UCBRK` writer - USCI Break received"]
pub type UcbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPE` reader - USCI Parity Error Flag"]
pub type UcpeR = crate::BitReader;
#[doc = "Field `UCPE` writer - USCI Parity Error Flag"]
pub type UcpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOE` reader - USCI Overrun Error Flag"]
pub type UcoeR = crate::BitReader;
#[doc = "Field `UCOE` writer - USCI Overrun Error Flag"]
pub type UcoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCFE` reader - USCI Frame Error Flag"]
pub type UcfeR = crate::BitReader;
#[doc = "Field `UCFE` writer - USCI Frame Error Flag"]
pub type UcfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub type UclistenR = crate::BitReader;
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub type UclistenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UcbusyR {
        UcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    pub fn ucaddr(&self) -> UcaddrR {
        UcaddrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    pub fn ucrxerr(&self) -> UcrxerrR {
        UcrxerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    pub fn ucbrk(&self) -> UcbrkR {
        UcbrkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    pub fn ucpe(&self) -> UcpeR {
        UcpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UcoeR {
        UcoeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UcfeR {
        UcfeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UclistenR {
        UclistenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Busy Flag"]
    #[inline(always)]
    pub fn ucbusy(&mut self) -> UcbusyW<'_, Uca0statSpec> {
        UcbusyW::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Address received Flag"]
    #[inline(always)]
    pub fn ucaddr(&mut self) -> UcaddrW<'_, Uca0statSpec> {
        UcaddrW::new(self, 1)
    }
    #[doc = "Bit 2 - USCI RX Error Flag"]
    #[inline(always)]
    pub fn ucrxerr(&mut self) -> UcrxerrW<'_, Uca0statSpec> {
        UcrxerrW::new(self, 2)
    }
    #[doc = "Bit 3 - USCI Break received"]
    #[inline(always)]
    pub fn ucbrk(&mut self) -> UcbrkW<'_, Uca0statSpec> {
        UcbrkW::new(self, 3)
    }
    #[doc = "Bit 4 - USCI Parity Error Flag"]
    #[inline(always)]
    pub fn ucpe(&mut self) -> UcpeW<'_, Uca0statSpec> {
        UcpeW::new(self, 4)
    }
    #[doc = "Bit 5 - USCI Overrun Error Flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UcoeW<'_, Uca0statSpec> {
        UcoeW::new(self, 5)
    }
    #[doc = "Bit 6 - USCI Frame Error Flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UcfeW<'_, Uca0statSpec> {
        UcfeW::new(self, 6)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UclistenW<'_, Uca0statSpec> {
        UclistenW::new(self, 7)
    }
}
#[doc = "USCI A0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0statSpec;
impl crate::RegisterSpec for Uca0statSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0stat::R`](R) reader structure"]
impl crate::Readable for Uca0statSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0stat::W`](W) writer structure"]
impl crate::Writable for Uca0statSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets UCA0STAT to value 0"]
impl crate::Resettable for Uca0statSpec {}
