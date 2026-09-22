#[doc = "Register `LCDBM0W` reader"]
pub type R = crate::R<Lcdbm0wSpec>;
#[doc = "Register `LCDBM0W` writer"]
pub type W = crate::W<Lcdbm0wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm0w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm0w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm0wSpec;
impl crate::RegisterSpec for Lcdbm0wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm0w::R`](R) reader structure"]
impl crate::Readable for Lcdbm0wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm0w::W`](W) writer structure"]
impl crate::Writable for Lcdbm0wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM0W to value 0"]
impl crate::Resettable for Lcdbm0wSpec {}
