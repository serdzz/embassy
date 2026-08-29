#[doc = "Register `MTIFPGCNF` reader"]
pub type R = crate::R<MtifpgcnfSpec>;
#[doc = "Register `MTIFPGCNF` writer"]
pub type W = crate::W<MtifpgcnfSpec>;
#[doc = "Field `PGEN` reader - PG sub module enable. This bit enables the PG sub module when set to one"]
pub type PgenR = crate::BitReader;
#[doc = "Field `PGEN` writer - PG sub module enable. This bit enables the PG sub module when set to one"]
pub type PgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGCLR` reader - PG pulse counter clear. This bit allows to clear the pulse generator (PGEN has to be set to one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. PCEN =0 and LFXTOFF=1 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
pub type PgclrR = crate::BitReader;
#[doc = "Field `PGCLR` writer - PG pulse counter clear. This bit allows to clear the pulse generator (PGEN has to be set to one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. PCEN =0 and LFXTOFF=1 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
pub type PgclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PG pulse grid frequency select. This value determines at which time grid pulses are generated. The pulse generator frame frequency is an 1/256th of this (PGEN has to be one to perform a change).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pgfs {
    #[doc = "0: Pulse grid frequency is set to 8 Hz (nominal)"]
    Pgfs0 = 0,
    #[doc = "1: Pulse grid frequency is set to 16 Hz (nominal)"]
    Pgfs1 = 1,
    #[doc = "2: Pulse grid frequency is set to 32 Hz (nominal)"]
    Pgfs2 = 2,
    #[doc = "3: Pulse grid frequency is set to 64 Hz (nominal)"]
    Pgfs3 = 3,
    #[doc = "4: Pulse grid frequency is set to 128 Hz (nominal)"]
    Pgfs4 = 4,
    #[doc = "5: Pulse grid frequency is set to 256 Hz (nominal)"]
    Pgfs5 = 5,
    #[doc = "6: Pulse grid frequency is set to 512 Hz (nominal)"]
    Pgfs6 = 6,
    #[doc = "7: Pulse grid frequency is set to 1024 Hz (nominal) default"]
    Pgfs7 = 7,
}
impl From<Pgfs> for u8 {
    #[inline(always)]
    fn from(variant: Pgfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pgfs {
    type Ux = u8;
}
impl crate::IsEnum for Pgfs {}
#[doc = "Field `PGFS` reader - PG pulse grid frequency select. This value determines at which time grid pulses are generated. The pulse generator frame frequency is an 1/256th of this (PGEN has to be one to perform a change)."]
pub type PgfsR = crate::FieldReader<Pgfs>;
impl PgfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgfs {
        match self.bits {
            0 => Pgfs::Pgfs0,
            1 => Pgfs::Pgfs1,
            2 => Pgfs::Pgfs2,
            3 => Pgfs::Pgfs3,
            4 => Pgfs::Pgfs4,
            5 => Pgfs::Pgfs5,
            6 => Pgfs::Pgfs6,
            7 => Pgfs::Pgfs7,
            _ => unreachable!(),
        }
    }
    #[doc = "Pulse grid frequency is set to 8 Hz (nominal)"]
    #[inline(always)]
    pub fn is_pgfs_0(&self) -> bool {
        *self == Pgfs::Pgfs0
    }
    #[doc = "Pulse grid frequency is set to 16 Hz (nominal)"]
    #[inline(always)]
    pub fn is_pgfs_1(&self) -> bool {
        *self == Pgfs::Pgfs1
    }
    #[doc = "Pulse grid frequency is set to 32 Hz (nominal)"]
    #[inline(always)]
    pub fn is_pgfs_2(&self) -> bool {
        *self == Pgfs::Pgfs2
    }
    #[doc = "Pulse grid frequency is set to 64 Hz (nominal)"]
    #[inline(always)]
    pub fn is_pgfs_3(&self) -> bool {
        *self == Pgfs::Pgfs3
    }
    #[doc = "Pulse grid frequency is set to 128 Hz (nominal)"]
    #[inline(always)]
    pub fn is_pgfs_4(&self) -> bool {
        *self == Pgfs::Pgfs4
    }
    #[doc = "Pulse grid frequency is set to 256 Hz (nominal)"]
    #[inline(always)]
    pub fn is_pgfs_5(&self) -> bool {
        *self == Pgfs::Pgfs5
    }
    #[doc = "Pulse grid frequency is set to 512 Hz (nominal)"]
    #[inline(always)]
    pub fn is_pgfs_6(&self) -> bool {
        *self == Pgfs::Pgfs6
    }
    #[doc = "Pulse grid frequency is set to 1024 Hz (nominal) default"]
    #[inline(always)]
    pub fn is_pgfs_7(&self) -> bool {
        *self == Pgfs::Pgfs7
    }
}
#[doc = "Field `PGFS` writer - PG pulse grid frequency select. This value determines at which time grid pulses are generated. The pulse generator frame frequency is an 1/256th of this (PGEN has to be one to perform a change)."]
pub type PgfsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pgfs, crate::Safe>;
impl<'a, REG> PgfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pulse grid frequency is set to 8 Hz (nominal)"]
    #[inline(always)]
    pub fn pgfs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs0)
    }
    #[doc = "Pulse grid frequency is set to 16 Hz (nominal)"]
    #[inline(always)]
    pub fn pgfs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs1)
    }
    #[doc = "Pulse grid frequency is set to 32 Hz (nominal)"]
    #[inline(always)]
    pub fn pgfs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs2)
    }
    #[doc = "Pulse grid frequency is set to 64 Hz (nominal)"]
    #[inline(always)]
    pub fn pgfs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs3)
    }
    #[doc = "Pulse grid frequency is set to 128 Hz (nominal)"]
    #[inline(always)]
    pub fn pgfs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs4)
    }
    #[doc = "Pulse grid frequency is set to 256 Hz (nominal)"]
    #[inline(always)]
    pub fn pgfs_5(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs5)
    }
    #[doc = "Pulse grid frequency is set to 512 Hz (nominal)"]
    #[inline(always)]
    pub fn pgfs_6(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs6)
    }
    #[doc = "Pulse grid frequency is set to 1024 Hz (nominal) default"]
    #[inline(always)]
    pub fn pgfs_7(self) -> &'a mut crate::W<REG> {
        self.variant(Pgfs::Pgfs7)
    }
}
#[doc = "Field `PGPW` reader - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
pub type PgpwR = crate::FieldReader;
#[doc = "Field `PGPW` writer - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
pub type PgpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PG sub module enable. This bit enables the PG sub module when set to one"]
    #[inline(always)]
    pub fn pgen(&self) -> PgenR {
        PgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PG pulse counter clear. This bit allows to clear the pulse generator (PGEN has to be set to one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. PCEN =0 and LFXTOFF=1 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
    #[inline(always)]
    pub fn pgclr(&self) -> PgclrR {
        PgclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PG pulse grid frequency select. This value determines at which time grid pulses are generated. The pulse generator frame frequency is an 1/256th of this (PGEN has to be one to perform a change)."]
    #[inline(always)]
    pub fn pgfs(&self) -> PgfsR {
        PgfsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
    #[inline(always)]
    pub fn pgpw(&self) -> PgpwR {
        PgpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PG sub module enable. This bit enables the PG sub module when set to one"]
    #[inline(always)]
    pub fn pgen(&mut self) -> PgenW<'_, MtifpgcnfSpec> {
        PgenW::new(self, 0)
    }
    #[doc = "Bit 2 - PG pulse counter clear. This bit allows to clear the pulse generator (PGEN has to be set to one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. PCEN =0 and LFXTOFF=1 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
    #[inline(always)]
    pub fn pgclr(&mut self) -> PgclrW<'_, MtifpgcnfSpec> {
        PgclrW::new(self, 2)
    }
    #[doc = "Bits 4:6 - PG pulse grid frequency select. This value determines at which time grid pulses are generated. The pulse generator frame frequency is an 1/256th of this (PGEN has to be one to perform a change)."]
    #[inline(always)]
    pub fn pgfs(&mut self) -> PgfsW<'_, MtifpgcnfSpec> {
        PgfsW::new(self, 4)
    }
    #[doc = "Bits 8:15 - PG password. Always reads as 0x69. Must be written as 0x5A for register changes to be effective. This password differs from the pin configuration and pulse counter passwords."]
    #[inline(always)]
    pub fn pgpw(&mut self) -> PgpwW<'_, MtifpgcnfSpec> {
        PgpwW::new(self, 8)
    }
}
#[doc = "Pulse Generator Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgcnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgcnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpgcnfSpec;
impl crate::RegisterSpec for MtifpgcnfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpgcnf::R`](R) reader structure"]
impl crate::Readable for MtifpgcnfSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpgcnf::W`](W) writer structure"]
impl crate::Writable for MtifpgcnfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPGCNF to value 0"]
impl crate::Resettable for MtifpgcnfSpec {}
