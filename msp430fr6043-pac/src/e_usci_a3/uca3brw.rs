#[doc = "Register `UCA3BRW` reader"]
pub type R = crate::R<Uca3brwSpec>;
#[doc = "Register `UCA3BRW` writer"]
pub type W = crate::W<Uca3brwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3brw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3brw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca3brwSpec;
impl crate::RegisterSpec for Uca3brwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca3brw::R`](R) reader structure"]
impl crate::Readable for Uca3brwSpec {}
#[doc = "`write(|w| ..)` method takes [`uca3brw::W`](W) writer structure"]
impl crate::Writable for Uca3brwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA3BRW to value 0"]
impl crate::Resettable for Uca3brwSpec {}
