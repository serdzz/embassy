#[doc = "Register `AESASTAT` reader"]
pub type R = crate::R<AesastatSpec>;
#[doc = "Register `AESASTAT` writer"]
pub type W = crate::W<AesastatSpec>;
#[doc = "AES accelerator module busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesbusy {
    #[doc = "0: Not busy"]
    Idle = 0,
    #[doc = "1: Busy"]
    Busy = 1,
}
impl From<Aesbusy> for bool {
    #[inline(always)]
    fn from(variant: Aesbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESBUSY` reader - AES accelerator module busy"]
pub type AesbusyR = crate::BitReader<Aesbusy>;
impl AesbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesbusy {
        match self.bits {
            false => Aesbusy::Idle,
            true => Aesbusy::Busy,
        }
    }
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Aesbusy::Idle
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Aesbusy::Busy
    }
}
#[doc = "Field `AESBUSY` writer - AES accelerator module busy"]
pub type AesbusyW<'a, REG> = crate::BitWriter<'a, REG, Aesbusy>;
impl<'a, REG> AesbusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Aesbusy::Idle)
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Aesbusy::Busy)
    }
}
#[doc = "All 16 bytes written to AESAKEY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aeskeywr {
    #[doc = "0: Not all bytes written"]
    Aeskeywr0 = 0,
    #[doc = "1: All bytes written"]
    Aeskeywr1 = 1,
}
impl From<Aeskeywr> for bool {
    #[inline(always)]
    fn from(variant: Aeskeywr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESKEYWR` reader - All 16 bytes written to AESAKEY"]
pub type AeskeywrR = crate::BitReader<Aeskeywr>;
impl AeskeywrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aeskeywr {
        match self.bits {
            false => Aeskeywr::Aeskeywr0,
            true => Aeskeywr::Aeskeywr1,
        }
    }
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn is_aeskeywr_0(&self) -> bool {
        *self == Aeskeywr::Aeskeywr0
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn is_aeskeywr_1(&self) -> bool {
        *self == Aeskeywr::Aeskeywr1
    }
}
#[doc = "Field `AESKEYWR` writer - All 16 bytes written to AESAKEY"]
pub type AeskeywrW<'a, REG> = crate::BitWriter<'a, REG, Aeskeywr>;
impl<'a, REG> AeskeywrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aeskeywr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskeywr::Aeskeywr0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aeskeywr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskeywr::Aeskeywr1)
    }
}
#[doc = "All 16 bytes written to AESADIN, AESAXDIN or AESAXIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesdinwr {
    #[doc = "0: Not all bytes written"]
    Aesdinwr0 = 0,
    #[doc = "1: All bytes written"]
    Aesdinwr1 = 1,
}
impl From<Aesdinwr> for bool {
    #[inline(always)]
    fn from(variant: Aesdinwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESDINWR` reader - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
pub type AesdinwrR = crate::BitReader<Aesdinwr>;
impl AesdinwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesdinwr {
        match self.bits {
            false => Aesdinwr::Aesdinwr0,
            true => Aesdinwr::Aesdinwr1,
        }
    }
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn is_aesdinwr_0(&self) -> bool {
        *self == Aesdinwr::Aesdinwr0
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn is_aesdinwr_1(&self) -> bool {
        *self == Aesdinwr::Aesdinwr1
    }
}
#[doc = "Field `AESDINWR` writer - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
pub type AesdinwrW<'a, REG> = crate::BitWriter<'a, REG, Aesdinwr>;
impl<'a, REG> AesdinwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aesdinwr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesdinwr::Aesdinwr0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aesdinwr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesdinwr::Aesdinwr1)
    }
}
#[doc = "All 16 bytes read from AESADOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesdoutrd {
    #[doc = "0: Not all bytes read"]
    Aesdoutrd0 = 0,
    #[doc = "1: All bytes read"]
    Aesdoutrd1 = 1,
}
impl From<Aesdoutrd> for bool {
    #[inline(always)]
    fn from(variant: Aesdoutrd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESDOUTRD` reader - All 16 bytes read from AESADOUT"]
pub type AesdoutrdR = crate::BitReader<Aesdoutrd>;
impl AesdoutrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesdoutrd {
        match self.bits {
            false => Aesdoutrd::Aesdoutrd0,
            true => Aesdoutrd::Aesdoutrd1,
        }
    }
    #[doc = "Not all bytes read"]
    #[inline(always)]
    pub fn is_aesdoutrd_0(&self) -> bool {
        *self == Aesdoutrd::Aesdoutrd0
    }
    #[doc = "All bytes read"]
    #[inline(always)]
    pub fn is_aesdoutrd_1(&self) -> bool {
        *self == Aesdoutrd::Aesdoutrd1
    }
}
#[doc = "Field `AESKEYCNT` reader - Bytes written via AESAKEY for AESKL=00, half-words written via AESAKEY"]
pub type AeskeycntR = crate::FieldReader;
#[doc = "Field `AESDINCNT` reader - Bytes written via AESADIN, AESAXDIN or AESAXIN"]
pub type AesdincntR = crate::FieldReader;
#[doc = "Field `AESDOUTCNT` reader - Bytes read via AESADOUT"]
pub type AesdoutcntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&self) -> AesbusyR {
        AesbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&self) -> AeskeywrR {
        AeskeywrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&self) -> AesdinwrR {
        AesdinwrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - All 16 bytes read from AESADOUT"]
    #[inline(always)]
    pub fn aesdoutrd(&self) -> AesdoutrdR {
        AesdoutrdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bytes written via AESAKEY for AESKL=00, half-words written via AESAKEY"]
    #[inline(always)]
    pub fn aeskeycnt(&self) -> AeskeycntR {
        AeskeycntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Bytes written via AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdincnt(&self) -> AesdincntR {
        AesdincntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Bytes read via AESADOUT"]
    #[inline(always)]
    pub fn aesdoutcnt(&self) -> AesdoutcntR {
        AesdoutcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&mut self) -> AesbusyW<'_, AesastatSpec> {
        AesbusyW::new(self, 0)
    }
    #[doc = "Bit 1 - All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&mut self) -> AeskeywrW<'_, AesastatSpec> {
        AeskeywrW::new(self, 1)
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&mut self) -> AesdinwrW<'_, AesastatSpec> {
        AesdinwrW::new(self, 2)
    }
}
#[doc = "AES Accelerator Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesastat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesastat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesastatSpec;
impl crate::RegisterSpec for AesastatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesastat::R`](R) reader structure"]
impl crate::Readable for AesastatSpec {}
#[doc = "`write(|w| ..)` method takes [`aesastat::W`](W) writer structure"]
impl crate::Writable for AesastatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESASTAT to value 0"]
impl crate::Resettable for AesastatSpec {}
