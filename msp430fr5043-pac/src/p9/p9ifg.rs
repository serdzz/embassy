#[doc = "Register `P9IFG` reader"]
pub type R = crate::R<P9ifgSpec>;
#[doc = "Register `P9IFG` writer"]
pub type W = crate::W<P9ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p9ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9ifgSpec;
impl crate::RegisterSpec for P9ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9ifg::R`](R) reader structure"]
impl crate::Readable for P9ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p9ifg::W`](W) writer structure"]
impl crate::Writable for P9ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9IFG to value 0"]
impl crate::Resettable for P9ifgSpec {}
