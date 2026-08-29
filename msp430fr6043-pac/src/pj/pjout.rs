#[doc = "Register `PJOUT` reader"]
pub type R = crate::R<PjoutSpec>;
#[doc = "Register `PJOUT` writer"]
pub type W = crate::W<PjoutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port J Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pjout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjoutSpec;
impl crate::RegisterSpec for PjoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjout::R`](R) reader structure"]
impl crate::Readable for PjoutSpec {}
#[doc = "`write(|w| ..)` method takes [`pjout::W`](W) writer structure"]
impl crate::Writable for PjoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJOUT to value 0"]
impl crate::Resettable for PjoutSpec {}
