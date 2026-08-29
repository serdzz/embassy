#[doc = "Register `P5IFG` reader"]
pub type R = crate::R<P5ifgSpec>;
#[doc = "Register `P5IFG` writer"]
pub type W = crate::W<P5ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5ifgSpec;
impl crate::RegisterSpec for P5ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5ifg::R`](R) reader structure"]
impl crate::Readable for P5ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p5ifg::W`](W) writer structure"]
impl crate::Writable for P5ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5IFG to value 0"]
impl crate::Resettable for P5ifgSpec {}
