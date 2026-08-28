#[doc = "Register `DAC12_0DAT` reader"]
pub type R = crate::R<Dac12_0datSpec>;
#[doc = "Register `DAC12_0DAT` writer"]
pub type W = crate::W<Dac12_0datSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DAC12_0 Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dac12_0dat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac12_0dat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac12_0datSpec;
impl crate::RegisterSpec for Dac12_0datSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dac12_0dat::R`](R) reader structure"]
impl crate::Readable for Dac12_0datSpec {}
#[doc = "`write(|w| ..)` method takes [`dac12_0dat::W`](W) writer structure"]
impl crate::Writable for Dac12_0datSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC12_0DAT to value 0"]
impl crate::Resettable for Dac12_0datSpec {}
