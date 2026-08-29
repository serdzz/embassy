#[doc = "Register `MTIFPGCTL` reader"]
pub type R = crate::R<MtifpgctlSpec>;
#[doc = "Register `MTIFPGCTL` writer"]
pub type W = crate::W<MtifpgctlSpec>;
#[doc = "Field `PKUR` reader - Pulse K-Count Update Request (with password protection as in PGCNF). The update of KVAL occurs during the frequency grid slot 0xff (e.g. in the last 4ms of a second with a pulse grid frequency of 256Hz)"]
pub type PkurR = crate::BitReader;
#[doc = "Field `PKUR` writer - Pulse K-Count Update Request (with password protection as in PGCNF). The update of KVAL occurs during the frequency grid slot 0xff (e.g. in the last 4ms of a second with a pulse grid frequency of 256Hz)"]
pub type PkurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGUR` reader - Pulse Grid Frequency Update Request (with password protection as in PGCNF). The update of PGFS occurs during the frequency grid slot 0xff (e.g. in the last 4ms of an second with an pulse grid frequency of 256Hz)"]
pub type PgurR = crate::BitReader;
#[doc = "Field `PGUR` writer - Pulse Grid Frequency Update Request (with password protection as in PGCNF). The update of PGFS occurs during the frequency grid slot 0xff (e.g. in the last 4ms of an second with an pulse grid frequency of 256Hz)"]
pub type PgurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGPW` reader - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
pub type PgpwR = crate::FieldReader;
#[doc = "Field `PGPW` writer - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
pub type PgpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Pulse K-Count Update Request (with password protection as in PGCNF). The update of KVAL occurs during the frequency grid slot 0xff (e.g. in the last 4ms of a second with a pulse grid frequency of 256Hz)"]
    #[inline(always)]
    pub fn pkur(&self) -> PkurR {
        PkurR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Grid Frequency Update Request (with password protection as in PGCNF). The update of PGFS occurs during the frequency grid slot 0xff (e.g. in the last 4ms of an second with an pulse grid frequency of 256Hz)"]
    #[inline(always)]
    pub fn pgur(&self) -> PgurR {
        PgurR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
    #[inline(always)]
    pub fn pgpw(&self) -> PgpwR {
        PgpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse K-Count Update Request (with password protection as in PGCNF). The update of KVAL occurs during the frequency grid slot 0xff (e.g. in the last 4ms of a second with a pulse grid frequency of 256Hz)"]
    #[inline(always)]
    pub fn pkur(&mut self) -> PkurW<'_, MtifpgctlSpec> {
        PkurW::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse Grid Frequency Update Request (with password protection as in PGCNF). The update of PGFS occurs during the frequency grid slot 0xff (e.g. in the last 4ms of an second with an pulse grid frequency of 256Hz)"]
    #[inline(always)]
    pub fn pgur(&mut self) -> PgurW<'_, MtifpgctlSpec> {
        PgurW::new(self, 1)
    }
    #[doc = "Bits 8:15 - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
    #[inline(always)]
    pub fn pgpw(&mut self) -> PgpwW<'_, MtifpgctlSpec> {
        PgpwW::new(self, 8)
    }
}
#[doc = "Pulse Generator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpgctlSpec;
impl crate::RegisterSpec for MtifpgctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpgctl::R`](R) reader structure"]
impl crate::Readable for MtifpgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpgctl::W`](W) writer structure"]
impl crate::Writable for MtifpgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPGCTL to value 0"]
impl crate::Resettable for MtifpgctlSpec {}
