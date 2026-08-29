#[doc = "Register `MTIFPCSR` reader"]
pub type R = crate::R<MtifpcsrSpec>;
#[doc = "Register `MTIFPCSR` writer"]
pub type W = crate::W<MtifpcsrSpec>;
#[doc = "Field `PCRA` reader - Pulse counter read acknowledge. This acknowledges the update of the PCR register as response to the PCRR read request. Note!: A read request is being latched. LFXTOFF=1 and PCEN=0 will prevent that.The read will then be performed and acknowledged after the clock is reenabled."]
pub type PcraR = crate::BitReader;
#[doc = "Field `PCRA` writer - Pulse counter read acknowledge. This acknowledges the update of the PCR register as response to the PCRR read request. Note!: A read request is being latched. LFXTOFF=1 and PCEN=0 will prevent that.The read will then be performed and acknowledged after the clock is reenabled."]
pub type PcraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCOFL` reader - Pulse counter overflow. This bit indicates an overflow of the pulse counter when its value changes since the last read request procedure. It is basically the 17th bit of the counter"]
pub type PcoflR = crate::BitReader;
#[doc = "Field `PCOFL` writer - Pulse counter overflow. This bit indicates an overflow of the pulse counter when its value changes since the last read request procedure. It is basically the 17th bit of the counter"]
pub type PcoflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse counter read acknowledge. This acknowledges the update of the PCR register as response to the PCRR read request. Note!: A read request is being latched. LFXTOFF=1 and PCEN=0 will prevent that.The read will then be performed and acknowledged after the clock is reenabled."]
    #[inline(always)]
    pub fn pcra(&self) -> PcraR {
        PcraR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse counter overflow. This bit indicates an overflow of the pulse counter when its value changes since the last read request procedure. It is basically the 17th bit of the counter"]
    #[inline(always)]
    pub fn pcofl(&self) -> PcoflR {
        PcoflR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse counter read acknowledge. This acknowledges the update of the PCR register as response to the PCRR read request. Note!: A read request is being latched. LFXTOFF=1 and PCEN=0 will prevent that.The read will then be performed and acknowledged after the clock is reenabled."]
    #[inline(always)]
    pub fn pcra(&mut self) -> PcraW<'_, MtifpcsrSpec> {
        PcraW::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse counter overflow. This bit indicates an overflow of the pulse counter when its value changes since the last read request procedure. It is basically the 17th bit of the counter"]
    #[inline(always)]
    pub fn pcofl(&mut self) -> PcoflW<'_, MtifpcsrSpec> {
        PcoflW::new(self, 1)
    }
}
#[doc = "Pulse Counter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpcsrSpec;
impl crate::RegisterSpec for MtifpcsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpcsr::R`](R) reader structure"]
impl crate::Readable for MtifpcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpcsr::W`](W) writer structure"]
impl crate::Writable for MtifpcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPCSR to value 0"]
impl crate::Resettable for MtifpcsrSpec {}
