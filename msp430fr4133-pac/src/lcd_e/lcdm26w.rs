#[doc = "Register `LCDM26W` reader"]
pub type R = crate::R<Lcdm26wSpec>;
#[doc = "Register `LCDM26W` writer"]
pub type W = crate::W<Lcdm26wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 26/27\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm26w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm26w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm26wSpec;
impl crate::RegisterSpec for Lcdm26wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm26w::R`](R) reader structure"]
impl crate::Readable for Lcdm26wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm26w::W`](W) writer structure"]
impl crate::Writable for Lcdm26wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM26W to value 0"]
impl crate::Resettable for Lcdm26wSpec {}
