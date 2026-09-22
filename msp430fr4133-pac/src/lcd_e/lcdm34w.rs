#[doc = "Register `LCDM34W` reader"]
pub type R = crate::R<Lcdm34wSpec>;
#[doc = "Register `LCDM34W` writer"]
pub type W = crate::W<Lcdm34wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 34/35\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm34w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm34w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm34wSpec;
impl crate::RegisterSpec for Lcdm34wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm34w::R`](R) reader structure"]
impl crate::Readable for Lcdm34wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm34w::W`](W) writer structure"]
impl crate::Writable for Lcdm34wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM34W to value 0"]
impl crate::Resettable for Lcdm34wSpec {}
