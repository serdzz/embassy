#[doc = "Register `LCDM24W` reader"]
pub type R = crate::R<Lcdm24wSpec>;
#[doc = "Register `LCDM24W` writer"]
pub type W = crate::W<Lcdm24wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Memory 24/25\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm24w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm24w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm24wSpec;
impl crate::RegisterSpec for Lcdm24wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdm24w::R`](R) reader structure"]
impl crate::Readable for Lcdm24wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdm24w::W`](W) writer structure"]
impl crate::Writable for Lcdm24wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM24W to value 0"]
impl crate::Resettable for Lcdm24wSpec {}
