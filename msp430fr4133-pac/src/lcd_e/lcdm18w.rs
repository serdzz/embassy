#[doc = "Register `LCDM18W` reader"]
pub type R = crate::R<Lcdm18wSpec>;
#[doc = "Register `LCDM18W` writer"]
pub type W = crate::W<Lcdm18wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 18/19\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm18w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm18w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm18wSpec;
impl crate::RegisterSpec for Lcdm18wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm18w::R`](R) reader structure"]
impl crate::Readable for Lcdm18wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm18w::W`](W) writer structure"]
impl crate::Writable for Lcdm18wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM18W to value 0"]
impl crate::Resettable for Lcdm18wSpec {}
