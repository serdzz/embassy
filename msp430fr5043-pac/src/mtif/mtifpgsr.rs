#[doc = "Register `MTIFPGSR` reader"]
pub type R = crate::R<MtifpgsrSpec>;
#[doc = "Register `MTIFPGSR` writer"]
pub type W = crate::W<MtifpgsrSpec>;
#[doc = "Field `PKUA` reader - Pulse K-Count Update Acknowledge. This acknowledges a PCUR directly after the K-values has been updated."]
pub type PkuaR = crate::BitReader;
#[doc = "Field `PKUA` writer - Pulse K-Count Update Acknowledge. This acknowledges a PCUR directly after the K-values has been updated."]
pub type PkuaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGUA` reader - Pulse Grid Frequency Update Acknowledge. This acknowledges a PGUR directly after the PGFS has been updated."]
pub type PguaR = crate::BitReader;
#[doc = "Field `PGUA` writer - Pulse Grid Frequency Update Acknowledge. This acknowledges a PGUR directly after the PGFS has been updated."]
pub type PguaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse K-Count Update Acknowledge. This acknowledges a PCUR directly after the K-values has been updated."]
    #[inline(always)]
    pub fn pkua(&self) -> PkuaR {
        PkuaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Grid Frequency Update Acknowledge. This acknowledges a PGUR directly after the PGFS has been updated."]
    #[inline(always)]
    pub fn pgua(&self) -> PguaR {
        PguaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse K-Count Update Acknowledge. This acknowledges a PCUR directly after the K-values has been updated."]
    #[inline(always)]
    pub fn pkua(&mut self) -> PkuaW<'_, MtifpgsrSpec> {
        PkuaW::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse Grid Frequency Update Acknowledge. This acknowledges a PGUR directly after the PGFS has been updated."]
    #[inline(always)]
    pub fn pgua(&mut self) -> PguaW<'_, MtifpgsrSpec> {
        PguaW::new(self, 1)
    }
}
#[doc = "Pulse Generator Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpgsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpgsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpgsrSpec;
impl crate::RegisterSpec for MtifpgsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpgsr::R`](R) reader structure"]
impl crate::Readable for MtifpgsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpgsr::W`](W) writer structure"]
impl crate::Writable for MtifpgsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPGSR to value 0"]
impl crate::Resettable for MtifpgsrSpec {}
