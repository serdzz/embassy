#[doc = "Register `MTIFPGKVAL` reader"]
pub type R = crate::R<MtifpgkvalSpec>;
#[doc = "Register `MTIFPGKVAL` writer"]
pub type W = crate::W<MtifpgkvalSpec>;
#[doc = "Field `KVAL` reader - Pulse Count Number. This register value determines how many pulses are generated withing 256 periods of the pulse grid frequency(with password protection as in PGCNF). PGEN has to be one to perform a change."]
pub type KvalR = crate::FieldReader;
#[doc = "Field `KVAL` writer - Pulse Count Number. This register value determines how many pulses are generated withing 256 periods of the pulse grid frequency(with password protection as in PGCNF). PGEN has to be one to perform a change."]
pub type KvalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PGPW` reader - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
pub type PgpwR = crate::FieldReader;
#[doc = "Field `PGPW` writer - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
pub type PgpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - Pulse Count Number. This register value determines how many pulses are generated withing 256 periods of the pulse grid frequency(with password protection as in PGCNF). PGEN has to be one to perform a change."]
    #[inline(always)]
    pub fn kval(&self) -> KvalR {
        KvalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
    #[inline(always)]
    pub fn pgpw(&self) -> PgpwR {
        PgpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Pulse Count Number. This register value determines how many pulses are generated withing 256 periods of the pulse grid frequency(with password protection as in PGCNF). PGEN has to be one to perform a change."]
    #[inline(always)]
    pub fn kval(&mut self) -> KvalW<'_, MtifpgkvalSpec> {
        KvalW::new(self, 0)
    }
    #[doc = "Bits 8:15 - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
    #[inline(always)]
    pub fn pgpw(&mut self) -> PgpwW<'_, MtifpgkvalSpec> {
        PgpwW::new(self, 8)
    }
}
#[doc = "Pulse Generator Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgkval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgkval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpgkvalSpec;
impl crate::RegisterSpec for MtifpgkvalSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpgkval::R`](R) reader structure"]
impl crate::Readable for MtifpgkvalSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpgkval::W`](W) writer structure"]
impl crate::Writable for MtifpgkvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPGKVAL to value 0"]
impl crate::Resettable for MtifpgkvalSpec {}
