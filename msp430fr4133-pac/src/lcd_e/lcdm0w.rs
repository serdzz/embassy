#[doc = "Register `LCDM0W` reader"]
pub type R = crate::R<Lcdm0wSpec>;
#[doc = "Register `LCDM0W` writer"]
pub type W = crate::W<Lcdm0wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm0w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm0w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm0wSpec;
impl crate::RegisterSpec for Lcdm0wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm0w::R`](R) reader structure"]
impl crate::Readable for Lcdm0wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm0w::W`](W) writer structure"]
impl crate::Writable for Lcdm0wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM0W to value 0"]
impl crate::Resettable for Lcdm0wSpec {}
