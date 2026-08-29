#[doc = "Register `CRC32DIRBW1` reader"]
pub type R = crate::R<Crc32dirbw1Spec>;
#[doc = "Register `CRC32DIRBW1` writer"]
pub type W = crate::W<Crc32dirbw1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC32 Data In Reverse Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32dirbw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32dirbw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc32dirbw1Spec;
impl crate::RegisterSpec for Crc32dirbw1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc32dirbw1::R`](R) reader structure"]
impl crate::Readable for Crc32dirbw1Spec {}
#[doc = "`write(|w| ..)` method takes [`crc32dirbw1::W`](W) writer structure"]
impl crate::Writable for Crc32dirbw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC32DIRBW1 to value 0"]
impl crate::Resettable for Crc32dirbw1Spec {}
