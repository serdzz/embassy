#[doc = "Register `CRC16DIW0` reader"]
pub type R = crate::R<Crc16diw0Spec>;
#[doc = "Register `CRC16DIW0` writer"]
pub type W = crate::W<Crc16diw0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC16 Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16diw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16diw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc16diw0Spec;
impl crate::RegisterSpec for Crc16diw0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc16diw0::R`](R) reader structure"]
impl crate::Readable for Crc16diw0Spec {}
#[doc = "`write(|w| ..)` method takes [`crc16diw0::W`](W) writer structure"]
impl crate::Writable for Crc16diw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC16DIW0 to value 0"]
impl crate::Resettable for Crc16diw0Spec {}
