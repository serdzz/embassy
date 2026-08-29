#[doc = "Register `P6IES` reader"]
pub type R = crate::R<P6iesSpec>;
#[doc = "Register `P6IES` writer"]
pub type W = crate::W<P6iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6iesSpec;
impl crate::RegisterSpec for P6iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6ies::R`](R) reader structure"]
impl crate::Readable for P6iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p6ies::W`](W) writer structure"]
impl crate::Writable for P6iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6IES to value 0"]
impl crate::Resettable for P6iesSpec {}
