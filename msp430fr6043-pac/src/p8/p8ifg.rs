#[doc = "Register `P8IFG` reader"]
pub type R = crate::R<P8ifgSpec>;
#[doc = "Register `P8IFG` writer"]
pub type W = crate::W<P8ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 8 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p8ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8ifgSpec;
impl crate::RegisterSpec for P8ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8ifg::R`](R) reader structure"]
impl crate::Readable for P8ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p8ifg::W`](W) writer structure"]
impl crate::Writable for P8ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8IFG to value 0"]
impl crate::Resettable for P8ifgSpec {}
