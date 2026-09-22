#[doc = "Register `LCDM36W` reader"]
pub type R = crate::R<Lcdm36wSpec>;
#[doc = "Register `LCDM36W` writer"]
pub type W = crate::W<Lcdm36wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 36/37\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm36w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm36w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm36wSpec;
impl crate::RegisterSpec for Lcdm36wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm36w::R`](R) reader structure"]
impl crate::Readable for Lcdm36wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm36w::W`](W) writer structure"]
impl crate::Writable for Lcdm36wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM36W to value 0"]
impl crate::Resettable for Lcdm36wSpec {}
