#[doc = "Register `LCDM12W` reader"]
pub type R = crate::R<Lcdm12wSpec>;
#[doc = "Register `LCDM12W` writer"]
pub type W = crate::W<Lcdm12wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 12/13\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm12w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm12w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm12wSpec;
impl crate::RegisterSpec for Lcdm12wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm12w::R`](R) reader structure"]
impl crate::Readable for Lcdm12wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm12w::W`](W) writer structure"]
impl crate::Writable for Lcdm12wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM12W to value 0"]
impl crate::Resettable for Lcdm12wSpec {}
