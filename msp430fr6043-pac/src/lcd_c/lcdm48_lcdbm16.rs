#[doc = "Register `LCDM48_LCDBM16` reader"]
pub type R = crate::R<Lcdm48Lcdbm16Spec>;
#[doc = "Register `LCDM48_LCDBM16` writer"]
pub type W = crate::W<Lcdm48Lcdbm16Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD memory 48 / LCD blinking memory 16\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm48_lcdbm16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm48_lcdbm16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm48Lcdbm16Spec;
impl crate::RegisterSpec for Lcdm48Lcdbm16Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm48_lcdbm16::R`](R) reader structure"]
impl crate::Readable for Lcdm48Lcdbm16Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm48_lcdbm16::W`](W) writer structure"]
impl crate::Writable for Lcdm48Lcdbm16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM48_LCDBM16 to value 0"]
impl crate::Resettable for Lcdm48Lcdbm16Spec {}
