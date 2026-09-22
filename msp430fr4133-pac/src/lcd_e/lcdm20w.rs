#[doc = "Register `LCDM20W` reader"]
pub type R = crate::R<Lcdm20wSpec>;
#[doc = "Register `LCDM20W` writer"]
pub type W = crate::W<Lcdm20wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 20/21\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm20w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm20w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm20wSpec;
impl crate::RegisterSpec for Lcdm20wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm20w::R`](R) reader structure"]
impl crate::Readable for Lcdm20wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm20w::W`](W) writer structure"]
impl crate::Writable for Lcdm20wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM20W to value 0"]
impl crate::Resettable for Lcdm20wSpec {}
