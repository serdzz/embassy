#[doc = "Register `LCDM4W` reader"]
pub type R = crate::R<Lcdm4wSpec>;
#[doc = "Register `LCDM4W` writer"]
pub type W = crate::W<Lcdm4wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 4/5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm4w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm4w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm4wSpec;
impl crate::RegisterSpec for Lcdm4wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm4w::R`](R) reader structure"]
impl crate::Readable for Lcdm4wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm4w::W`](W) writer structure"]
impl crate::Writable for Lcdm4wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM4W to value 0"]
impl crate::Resettable for Lcdm4wSpec {}
