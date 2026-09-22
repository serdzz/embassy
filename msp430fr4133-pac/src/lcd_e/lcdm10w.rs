#[doc = "Register `LCDM10W` reader"]
pub type R = crate::R<Lcdm10wSpec>;
#[doc = "Register `LCDM10W` writer"]
pub type W = crate::W<Lcdm10wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 10/11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm10w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm10w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm10wSpec;
impl crate::RegisterSpec for Lcdm10wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm10w::R`](R) reader structure"]
impl crate::Readable for Lcdm10wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm10w::W`](W) writer structure"]
impl crate::Writable for Lcdm10wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM10W to value 0"]
impl crate::Resettable for Lcdm10wSpec {}
