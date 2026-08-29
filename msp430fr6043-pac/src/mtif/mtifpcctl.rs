#[doc = "Register `MTIFPCCTL` reader"]
pub type R = crate::R<MtifpcctlSpec>;
#[doc = "Register `MTIFPCCTL` writer"]
pub type W = crate::W<MtifpcctlSpec>;
#[doc = "Field `PCRR` reader - Pulse Counter Read Request. Set this to request an update of PCR read register from the actual counter."]
pub type PcrrR = crate::BitReader;
#[doc = "Field `PCRR` writer - Pulse Counter Read Request. Set this to request an update of PCR read register from the actual counter."]
pub type PcrrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse Counter Read Request. Set this to request an update of PCR read register from the actual counter."]
    #[inline(always)]
    pub fn pcrr(&self) -> PcrrR {
        PcrrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Counter Read Request. Set this to request an update of PCR read register from the actual counter."]
    #[inline(always)]
    pub fn pcrr(&mut self) -> PcrrW<'_, MtifpcctlSpec> {
        PcrrW::new(self, 0)
    }
}
#[doc = "Pulse Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpcctlSpec;
impl crate::RegisterSpec for MtifpcctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpcctl::R`](R) reader structure"]
impl crate::Readable for MtifpcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpcctl::W`](W) writer structure"]
impl crate::Writable for MtifpcctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPCCTL to value 0"]
impl crate::Resettable for MtifpcctlSpec {}
