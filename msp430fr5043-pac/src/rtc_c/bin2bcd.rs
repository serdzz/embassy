#[doc = "Register `BIN2BCD` reader"]
pub type R = crate::R<Bin2bcdSpec>;
#[doc = "Register `BIN2BCD` writer"]
pub type W = crate::W<Bin2bcdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Binary-to-BCD Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bin2bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bin2bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bin2bcdSpec;
impl crate::RegisterSpec for Bin2bcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bin2bcd::R`](R) reader structure"]
impl crate::Readable for Bin2bcdSpec {}
#[doc = "`write(|w| ..)` method takes [`bin2bcd::W`](W) writer structure"]
impl crate::Writable for Bin2bcdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIN2BCD to value 0"]
impl crate::Resettable for Bin2bcdSpec {}
