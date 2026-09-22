#[doc = "Register `LCDM6W` reader"]
pub type R = crate::R<Lcdm6wSpec>;
#[doc = "Register `LCDM6W` writer"]
pub type W = crate::W<Lcdm6wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 6/7\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm6w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm6w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm6wSpec;
impl crate::RegisterSpec for Lcdm6wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm6w::R`](R) reader structure"]
impl crate::Readable for Lcdm6wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm6w::W`](W) writer structure"]
impl crate::Writable for Lcdm6wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM6W to value 0"]
impl crate::Resettable for Lcdm6wSpec {}
