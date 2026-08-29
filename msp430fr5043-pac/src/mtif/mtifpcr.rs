#[doc = "Register `MTIFPCR` reader"]
pub type R = crate::R<MtifpcrSpec>;
#[doc = "Register `MTIFPCR` writer"]
pub type W = crate::W<MtifpcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pulse Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtifpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtifpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtifpcrSpec;
impl crate::RegisterSpec for MtifpcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mtifpcr::R`](R) reader structure"]
impl crate::Readable for MtifpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mtifpcr::W`](W) writer structure"]
impl crate::Writable for MtifpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIFPCR to value 0"]
impl crate::Resettable for MtifpcrSpec {}
