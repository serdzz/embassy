#[doc = "Register `HSPLLDESCHI` reader"]
pub type R = crate::R<HsplldeschiSpec>;
#[doc = "Register `HSPLLDESCHI` writer"]
pub type W = crate::W<HsplldeschiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "HSPLL Descriptor Register H.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsplldeschi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsplldeschi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsplldeschiSpec;
impl crate::RegisterSpec for HsplldeschiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hsplldeschi::R`](R) reader structure"]
impl crate::Readable for HsplldeschiSpec {}
#[doc = "`write(|w| ..)` method takes [`hsplldeschi::W`](W) writer structure"]
impl crate::Writable for HsplldeschiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPLLDESCHI to value 0"]
impl crate::Resettable for HsplldeschiSpec {}
