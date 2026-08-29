#[doc = "Register `P7SELC` reader"]
pub type R = crate::R<P7selcSpec>;
#[doc = "Register `P7SELC` writer"]
pub type W = crate::W<P7selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p7selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7selcSpec;
impl crate::RegisterSpec for P7selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7selc::R`](R) reader structure"]
impl crate::Readable for P7selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p7selc::W`](W) writer structure"]
impl crate::Writable for P7selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7SELC to value 0"]
impl crate::Resettable for P7selcSpec {}
