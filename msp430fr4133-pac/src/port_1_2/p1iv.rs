#[doc = "Register `P1IV` reader"]
pub type R = crate::R<P1ivSpec>;
#[doc = "Register `P1IV` writer"]
pub type W = crate::W<P1ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`p1iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ivSpec;
impl crate::RegisterSpec for P1ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p1iv::R`](R) reader structure"]
impl crate::Readable for P1ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p1iv::W`](W) writer structure"]
impl crate::Writable for P1ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1IV to value 0"]
impl crate::Resettable for P1ivSpec {}
