#[doc = "Register `P8SELC` reader"]
pub type R = crate::R<P8selcSpec>;
#[doc = "Register `P8SELC` writer"]
pub type W = crate::W<P8selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 8 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p8selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8selcSpec;
impl crate::RegisterSpec for P8selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8selc::R`](R) reader structure"]
impl crate::Readable for P8selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p8selc::W`](W) writer structure"]
impl crate::Writable for P8selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8SELC to value 0"]
impl crate::Resettable for P8selcSpec {}
