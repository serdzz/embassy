#[doc = "Register `P9REN` reader"]
pub type R = crate::R<P9renSpec>;
#[doc = "Register `P9REN` writer"]
pub type W = crate::W<P9renSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9renSpec;
impl crate::RegisterSpec for P9renSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9ren::R`](R) reader structure"]
impl crate::Readable for P9renSpec {}
#[doc = "`write(|w| ..)` method takes [`p9ren::W`](W) writer structure"]
impl crate::Writable for P9renSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9REN to value 0"]
impl crate::Resettable for P9renSpec {}
