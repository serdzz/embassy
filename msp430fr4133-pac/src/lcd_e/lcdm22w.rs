#[doc = "Register `LCDM22W` reader"]
pub type R = crate::R<Lcdm22wSpec>;
#[doc = "Register `LCDM22W` writer"]
pub type W = crate::W<Lcdm22wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 22/23\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm22w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm22w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm22wSpec;
impl crate::RegisterSpec for Lcdm22wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm22w::R`](R) reader structure"]
impl crate::Readable for Lcdm22wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm22w::W`](W) writer structure"]
impl crate::Writable for Lcdm22wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM22W to value 0"]
impl crate::Resettable for Lcdm22wSpec {}
