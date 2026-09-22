#[doc = "Register `LCDBM10W` reader"]
pub type R = crate::R<Lcdbm10wSpec>;
#[doc = "Register `LCDBM10W` writer"]
pub type W = crate::W<Lcdbm10wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 10/11\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm10w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm10w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm10wSpec;
impl crate::RegisterSpec for Lcdbm10wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm10w::R`](R) reader structure"]
impl crate::Readable for Lcdbm10wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm10w::W`](W) writer structure"]
impl crate::Writable for Lcdbm10wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM10W to value 0"]
impl crate::Resettable for Lcdbm10wSpec {}
