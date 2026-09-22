#[doc = "Register `LCDM2W` reader"]
pub type R = crate::R<Lcdm2wSpec>;
#[doc = "Register `LCDM2W` writer"]
pub type W = crate::W<Lcdm2wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm2w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm2w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm2wSpec;
impl crate::RegisterSpec for Lcdm2wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm2w::R`](R) reader structure"]
impl crate::Readable for Lcdm2wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm2w::W`](W) writer structure"]
impl crate::Writable for Lcdm2wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM2W to value 0"]
impl crate::Resettable for Lcdm2wSpec {}
