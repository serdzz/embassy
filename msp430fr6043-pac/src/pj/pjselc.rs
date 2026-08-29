#[doc = "Register `PJSELC` reader"]
pub type R = crate::R<PjselcSpec>;
#[doc = "Register `PJSELC` writer"]
pub type W = crate::W<PjselcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port J Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pjselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjselcSpec;
impl crate::RegisterSpec for PjselcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjselc::R`](R) reader structure"]
impl crate::Readable for PjselcSpec {}
#[doc = "`write(|w| ..)` method takes [`pjselc::W`](W) writer structure"]
impl crate::Writable for PjselcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJSELC to value 0"]
impl crate::Resettable for PjselcSpec {}
