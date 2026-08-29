#[doc = "Register `PJSEL1` reader"]
pub type R = crate::R<Pjsel1Spec>;
#[doc = "Register `PJSEL1` writer"]
pub type W = crate::W<Pjsel1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port J Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pjsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pjsel1Spec;
impl crate::RegisterSpec for Pjsel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjsel1::R`](R) reader structure"]
impl crate::Readable for Pjsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pjsel1::W`](W) writer structure"]
impl crate::Writable for Pjsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJSEL1 to value 0"]
impl crate::Resettable for Pjsel1Spec {}
