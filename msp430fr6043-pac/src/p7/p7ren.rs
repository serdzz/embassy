#[doc = "Register `P7REN` reader"]
pub type R = crate::R<P7renSpec>;
#[doc = "Register `P7REN` writer"]
pub type W = crate::W<P7renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7renSpec;
impl crate::RegisterSpec for P7renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7ren::R`](R) reader structure"]
impl crate::Readable for P7renSpec {}
#[doc = "`write(|w| ..)` method takes [`p7ren::W`](W) writer structure"]
impl crate::Writable for P7renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7REN to value 0"]
impl crate::Resettable for P7renSpec {}
