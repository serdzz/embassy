#[doc = "Register `PJREN` reader"]
pub type R = crate::R<PjrenSpec>;
#[doc = "Register `PJREN` writer"]
pub type W = crate::W<PjrenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port J Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pjren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjrenSpec;
impl crate::RegisterSpec for PjrenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjren::R`](R) reader structure"]
impl crate::Readable for PjrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pjren::W`](W) writer structure"]
impl crate::Writable for PjrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJREN to value 0"]
impl crate::Resettable for PjrenSpec {}
