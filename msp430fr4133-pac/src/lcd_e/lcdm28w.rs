#[doc = "Register `LCDM28W` reader"]
pub type R = crate::R<Lcdm28wSpec>;
#[doc = "Register `LCDM28W` writer"]
pub type W = crate::W<Lcdm28wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 28/29\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm28w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm28w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm28wSpec;
impl crate::RegisterSpec for Lcdm28wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm28w::R`](R) reader structure"]
impl crate::Readable for Lcdm28wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm28w::W`](W) writer structure"]
impl crate::Writable for Lcdm28wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM28W to value 0"]
impl crate::Resettable for Lcdm28wSpec {}
