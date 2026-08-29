#[doc = "Register `UUPSDESCHI` reader"]
pub type R = crate::R<UupsdeschiSpec>;
#[doc = "Register `UUPSDESCHI` writer"]
pub type W = crate::W<UupsdeschiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UUPS Descriptor Register H.\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsdeschi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsdeschi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UupsdeschiSpec;
impl crate::RegisterSpec for UupsdeschiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uupsdeschi::R`](R) reader structure"]
impl crate::Readable for UupsdeschiSpec {}
#[doc = "`write(|w| ..)` method takes [`uupsdeschi::W`](W) writer structure"]
impl crate::Writable for UupsdeschiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUPSDESCHI to value 0"]
impl crate::Resettable for UupsdeschiSpec {}
