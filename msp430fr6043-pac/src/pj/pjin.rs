#[doc = "Register `PJIN` reader"]
pub type R = crate::R<PjinSpec>;
#[doc = "Register `PJIN` writer"]
pub type W = crate::W<PjinSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port J Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pjin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjinSpec;
impl crate::RegisterSpec for PjinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjin::R`](R) reader structure"]
impl crate::Readable for PjinSpec {}
#[doc = "`write(|w| ..)` method takes [`pjin::W`](W) writer structure"]
impl crate::Writable for PjinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJIN to value 0"]
impl crate::Resettable for PjinSpec {}
