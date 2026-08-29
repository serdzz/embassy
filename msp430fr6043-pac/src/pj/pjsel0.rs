#[doc = "Register `PJSEL0` reader"]
pub type R = crate::R<Pjsel0Spec>;
#[doc = "Register `PJSEL0` writer"]
pub type W = crate::W<Pjsel0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port J Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pjsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pjsel0Spec;
impl crate::RegisterSpec for Pjsel0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjsel0::R`](R) reader structure"]
impl crate::Readable for Pjsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pjsel0::W`](W) writer structure"]
impl crate::Writable for Pjsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJSEL0 to value 0"]
impl crate::Resettable for Pjsel0Spec {}
