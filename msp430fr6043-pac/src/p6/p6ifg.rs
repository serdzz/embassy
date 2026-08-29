#[doc = "Register `P6IFG` reader"]
pub type R = crate::R<P6ifgSpec>;
#[doc = "Register `P6IFG` writer"]
pub type W = crate::W<P6ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6ifgSpec;
impl crate::RegisterSpec for P6ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6ifg::R`](R) reader structure"]
impl crate::Readable for P6ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p6ifg::W`](W) writer structure"]
impl crate::Writable for P6ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6IFG to value 0"]
impl crate::Resettable for P6ifgSpec {}
