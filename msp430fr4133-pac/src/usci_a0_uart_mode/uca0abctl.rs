#[doc = "Register `UCA0ABCTL` reader"]
pub type R = crate::R<Uca0abctlSpec>;
#[doc = "Register `UCA0ABCTL` writer"]
pub type W = crate::W<Uca0abctlSpec>;
#[doc = "Field `UCABDEN` reader - Auto Baud Rate detect enable"]
pub type UcabdenR = crate::BitReader;
#[doc = "Field `UCABDEN` writer - Auto Baud Rate detect enable"]
pub type UcabdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBTOE` reader - Break Timeout error"]
pub type UcbtoeR = crate::BitReader;
#[doc = "Field `UCBTOE` writer - Break Timeout error"]
pub type UcbtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTOE` reader - Sync-Field Timeout error"]
pub type UcstoeR = crate::BitReader;
#[doc = "Field `UCSTOE` writer - Sync-Field Timeout error"]
pub type UcstoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDELIM0` reader - Break Sync Delimiter 0"]
pub type Ucdelim0R = crate::BitReader;
#[doc = "Field `UCDELIM0` writer - Break Sync Delimiter 0"]
pub type Ucdelim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDELIM1` reader - Break Sync Delimiter 1"]
pub type Ucdelim1R = crate::BitReader;
#[doc = "Field `UCDELIM1` writer - Break Sync Delimiter 1"]
pub type Ucdelim1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UcabdenR {
        UcabdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UcbtoeR {
        UcbtoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UcstoeR {
        UcstoeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim0(&self) -> Ucdelim0R {
        Ucdelim0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Break Sync Delimiter 1"]
    #[inline(always)]
    pub fn ucdelim1(&self) -> Ucdelim1R {
        Ucdelim1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UcabdenW<'_, Uca0abctlSpec> {
        UcabdenW::new(self, 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UcbtoeW<'_, Uca0abctlSpec> {
        UcbtoeW::new(self, 2)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UcstoeW<'_, Uca0abctlSpec> {
        UcstoeW::new(self, 3)
    }
    #[doc = "Bit 4 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim0(&mut self) -> Ucdelim0W<'_, Uca0abctlSpec> {
        Ucdelim0W::new(self, 4)
    }
    #[doc = "Bit 5 - Break Sync Delimiter 1"]
    #[inline(always)]
    pub fn ucdelim1(&mut self) -> Ucdelim1W<'_, Uca0abctlSpec> {
        Ucdelim1W::new(self, 5)
    }
}
#[doc = "USCI A0 LIN Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0abctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0abctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0abctlSpec;
impl crate::RegisterSpec for Uca0abctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca0abctl::R`](R) reader structure"]
impl crate::Readable for Uca0abctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0abctl::W`](W) writer structure"]
impl crate::Writable for Uca0abctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0ABCTL to value 0"]
impl crate::Resettable for Uca0abctlSpec {}
