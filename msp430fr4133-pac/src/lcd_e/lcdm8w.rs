#[doc = "Register `LCDM8W` reader"]
pub type R = crate::R<Lcdm8wSpec>;
#[doc = "Register `LCDM8W` writer"]
pub type W = crate::W<Lcdm8wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 8/9\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm8w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm8w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm8wSpec;
impl crate::RegisterSpec for Lcdm8wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm8w::R`](R) reader structure"]
impl crate::Readable for Lcdm8wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm8w::W`](W) writer structure"]
impl crate::Writable for Lcdm8wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM8W to value 0"]
impl crate::Resettable for Lcdm8wSpec {}
