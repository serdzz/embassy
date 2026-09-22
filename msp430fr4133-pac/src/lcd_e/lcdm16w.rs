#[doc = "Register `LCDM16W` reader"]
pub type R = crate::R<Lcdm16wSpec>;
#[doc = "Register `LCDM16W` writer"]
pub type W = crate::W<Lcdm16wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 16/17\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm16w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm16w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm16wSpec;
impl crate::RegisterSpec for Lcdm16wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm16w::R`](R) reader structure"]
impl crate::Readable for Lcdm16wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm16w::W`](W) writer structure"]
impl crate::Writable for Lcdm16wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM16W to value 0"]
impl crate::Resettable for Lcdm16wSpec {}
