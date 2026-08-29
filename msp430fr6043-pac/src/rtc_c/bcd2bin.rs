#[doc = "Register `BCD2BIN` reader"]
pub type R = crate::R<Bcd2binSpec>;
#[doc = "Register `BCD2BIN` writer"]
pub type W = crate::W<Bcd2binSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BCD-to-Binary Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcd2bin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcd2bin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcd2binSpec;
impl crate::RegisterSpec for Bcd2binSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bcd2bin::R`](R) reader structure"]
impl crate::Readable for Bcd2binSpec {}
#[doc = "`write(|w| ..)` method takes [`bcd2bin::W`](W) writer structure"]
impl crate::Writable for Bcd2binSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCD2BIN to value 0"]
impl crate::Resettable for Bcd2binSpec {}
