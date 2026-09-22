#[doc = "Register `LCDM30W` reader"]
pub type R = crate::R<Lcdm30wSpec>;
#[doc = "Register `LCDM30W` writer"]
pub type W = crate::W<Lcdm30wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 30/31\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm30w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm30w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm30wSpec;
impl crate::RegisterSpec for Lcdm30wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm30w::R`](R) reader structure"]
impl crate::Readable for Lcdm30wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm30w::W`](W) writer structure"]
impl crate::Writable for Lcdm30wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM30W to value 0"]
impl crate::Resettable for Lcdm30wSpec {}
