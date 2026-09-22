#[doc = "Register `LCDM38W` reader"]
pub type R = crate::R<Lcdm38wSpec>;
#[doc = "Register `LCDM38W` writer"]
pub type W = crate::W<Lcdm38wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 38/39\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm38w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm38w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm38wSpec;
impl crate::RegisterSpec for Lcdm38wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm38w::R`](R) reader structure"]
impl crate::Readable for Lcdm38wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm38w::W`](W) writer structure"]
impl crate::Writable for Lcdm38wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM38W to value 0"]
impl crate::Resettable for Lcdm38wSpec {}
