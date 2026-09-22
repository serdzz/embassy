#[doc = "Register `LCDM32W` reader"]
pub type R = crate::R<Lcdm32wSpec>;
#[doc = "Register `LCDM32W` writer"]
pub type W = crate::W<Lcdm32wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 32/33\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm32w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm32w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm32wSpec;
impl crate::RegisterSpec for Lcdm32wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm32w::R`](R) reader structure"]
impl crate::Readable for Lcdm32wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm32w::W`](W) writer structure"]
impl crate::Writable for Lcdm32wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM32W to value 0"]
impl crate::Resettable for Lcdm32wSpec {}
