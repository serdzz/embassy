#[doc = "Register `LCDCCPCTL` reader"]
pub type R = crate::R<LcdccpctlSpec>;
#[doc = "Register `LCDCCPCTL` writer"]
pub type W = crate::W<LcdccpctlSpec>;
#[doc = "Field `LCDCPDIS` reader - LCD charge pump disable"]
pub type LcdcpdisR = crate::FieldReader;
#[doc = "Field `LCDCPDIS` writer - LCD charge pump disable"]
pub type LcdcpdisW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "LCD charge pump clock synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcpclksync {
    #[doc = "0: Synchronization disabled"]
    Lcdcpclksync0 = 0,
    #[doc = "1: Synchronization enabled"]
    Lcdcpclksync1 = 1,
}
impl From<Lcdcpclksync> for bool {
    #[inline(always)]
    fn from(variant: Lcdcpclksync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCPCLKSYNC` reader - LCD charge pump clock synchronization"]
pub type LcdcpclksyncR = crate::BitReader<Lcdcpclksync>;
impl LcdcpclksyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcpclksync {
        match self.bits {
            false => Lcdcpclksync::Lcdcpclksync0,
            true => Lcdcpclksync::Lcdcpclksync1,
        }
    }
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn is_lcdcpclksync_0(&self) -> bool {
        *self == Lcdcpclksync::Lcdcpclksync0
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn is_lcdcpclksync_1(&self) -> bool {
        *self == Lcdcpclksync::Lcdcpclksync1
    }
}
#[doc = "Field `LCDCPCLKSYNC` writer - LCD charge pump clock synchronization"]
pub type LcdcpclksyncW<'a, REG> = crate::BitWriter<'a, REG, Lcdcpclksync>;
impl<'a, REG> LcdcpclksyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn lcdcpclksync_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpclksync::Lcdcpclksync0)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn lcdcpclksync_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpclksync::Lcdcpclksync1)
    }
}
impl R {
    #[doc = "Bits 0:7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis(&self) -> LcdcpdisR {
        LcdcpdisR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&self) -> LcdcpclksyncR {
        LcdcpclksyncR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis(&mut self) -> LcdcpdisW<'_, LcdccpctlSpec> {
        LcdcpdisW::new(self, 0)
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&mut self) -> LcdcpclksyncW<'_, LcdccpctlSpec> {
        LcdcpclksyncW::new(self, 15)
    }
}
#[doc = "LCD_C charge pump control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdccpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdccpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdccpctlSpec;
impl crate::RegisterSpec for LcdccpctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdccpctl::R`](R) reader structure"]
impl crate::Readable for LcdccpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdccpctl::W`](W) writer structure"]
impl crate::Writable for LcdccpctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCCPCTL to value 0"]
impl crate::Resettable for LcdccpctlSpec {}
