#[doc = "Register `P9SELC` reader"]
pub type R = crate::R<P9selcSpec>;
#[doc = "Register `P9SELC` writer"]
pub type W = crate::W<P9selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 9 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p9selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p9selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9selcSpec;
impl crate::RegisterSpec for P9selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p9selc::R`](R) reader structure"]
impl crate::Readable for P9selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p9selc::W`](W) writer structure"]
impl crate::Writable for P9selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P9SELC to value 0"]
impl crate::Resettable for P9selcSpec {}
