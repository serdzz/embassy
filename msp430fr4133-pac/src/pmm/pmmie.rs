#[doc = "Register `PMMIE` reader"]
pub type R = crate::R<PmmieSpec>;
#[doc = "Register `PMMIE` writer"]
pub type W = crate::W<PmmieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PMM Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmmieSpec;
impl crate::RegisterSpec for PmmieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmie::R`](R) reader structure"]
impl crate::Readable for PmmieSpec {}
#[doc = "`write(|w| ..)` method takes [`pmmie::W`](W) writer structure"]
impl crate::Writable for PmmieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMIE to value 0"]
impl crate::Resettable for PmmieSpec {}
