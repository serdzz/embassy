#[doc = "Register `PJDIR` reader"]
pub type R = crate::R<PjdirSpec>;
#[doc = "Register `PJDIR` writer"]
pub type W = crate::W<PjdirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port J Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pjdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjdirSpec;
impl crate::RegisterSpec for PjdirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjdir::R`](R) reader structure"]
impl crate::Readable for PjdirSpec {}
#[doc = "`write(|w| ..)` method takes [`pjdir::W`](W) writer structure"]
impl crate::Writable for PjdirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJDIR to value 0"]
impl crate::Resettable for PjdirSpec {}
