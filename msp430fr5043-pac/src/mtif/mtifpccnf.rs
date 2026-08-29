#[doc = "Register `MTIFPCCNF` reader"]
pub type R = crate::R<MtifpccnfSpec>;
#[doc = "Register `MTIFPCCNF` writer"]
pub type W = crate::W<MtifpccnfSpec>;
#[doc = "Field `PCEN` reader - PC sub module enable. This bit enables the PC sub module when set to one"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - PC sub module enable. This bit enables the PC sub module when set to one"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCLR` reader - Pulse counter clear. This bit allows to clear the pulse counter when set to one (PCEN has to be one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. LFXTOFF=1 and PCEN=0 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
pub type PcclrR = crate::BitReader;
#[doc = "Field `PCCLR` writer - Pulse counter clear. This bit allows to clear the pulse counter when set to one (PCEN has to be one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. LFXTOFF=1 and PCEN=0 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
pub type PcclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPW` reader - Pulse counter password. Always reads as 0x96. Must be written as 0xA5 for register changes to be effective. This password differs from the pin configuration and pulse generator passwords"]
pub type PcpwR = crate::FieldReader;
#[doc = "Field `PCPW` writer - Pulse counter password. Always reads as 0x96. Must be written as 0xA5 for register changes to be effective. This password differs from the pin configuration and pulse generator passwords"]
pub type PcpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PC sub module enable. This bit enables the PC sub module when set to one"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse counter clear. This bit allows to clear the pulse counter when set to one (PCEN has to be one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. LFXTOFF=1 and PCEN=0 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
    #[inline(always)]
    pub fn pcclr(&self) -> PcclrR {
        PcclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Pulse counter password. Always reads as 0x96. Must be written as 0xA5 for register changes to be effective. This password differs from the pin configuration and pulse generator passwords"]
    #[inline(always)]
    pub fn pcpw(&self) -> PcpwR {
        PcpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PC sub module enable. This bit enables the PC sub module when set to one"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PcenW<'_, MtifpccnfSpec> {
        PcenW::new(self, 0)
    }
    #[doc = "Bit 2 - Pulse counter clear. This bit allows to clear the pulse counter when set to one (PCEN has to be one to perform a clear). Note!: A clear request is being latched and released after the clear is executed. LFXTOFF=1 and PCEN=0 will prevent that. The clear occurs then after the clock is reenabled. This bit is for triggering only; it's state cannot be read back"]
    #[inline(always)]
    pub fn pcclr(&mut self) -> PcclrW<'_, MtifpccnfSpec> {
        PcclrW::new(self, 2)
    }
    #[doc = "Bits 8:15 - Pulse counter password. Always reads as 0x96. Must be written as 0xA5 for register changes to be effective. This password differs from the pin configuration and pulse generator passwords"]
    #[inline(always)]
    pub fn pcpw(&mut self) -> PcpwW<'_, MtifpccnfSpec> {
        PcpwW::new(self, 8)
    }
}
#[doc = "Pulse Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpccnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpccnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpccnfSpec;
impl crate::RegisterSpec for MtifpccnfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpccnf::R`](R) reader structure"]
impl crate::Readable for MtifpccnfSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpccnf::W`](W) writer structure"]
impl crate::Writable for MtifpccnfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPCCNF to value 0"]
impl crate::Resettable for MtifpccnfSpec {}
