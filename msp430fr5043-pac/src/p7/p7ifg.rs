#[doc = "Register `P7IFG` reader"]
pub type R = crate::R<P7ifgSpec>;
#[doc = "Register `P7IFG` writer"]
pub type W = crate::W<P7ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7ifgSpec;
impl crate::RegisterSpec for P7ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7ifg::R`](R) reader structure"]
impl crate::Readable for P7ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p7ifg::W`](W) writer structure"]
impl crate::Writable for P7ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7IFG to value 0"]
impl crate::Resettable for P7ifgSpec {}
