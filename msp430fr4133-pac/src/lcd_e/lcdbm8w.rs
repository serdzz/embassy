#[doc = "Register `LCDBM8W` reader"]
pub type R = crate::R<Lcdbm8wSpec>;
#[doc = "Register `LCDBM8W` writer"]
pub type W = crate::W<Lcdbm8wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 8/9\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm8w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm8w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm8wSpec;
impl crate::RegisterSpec for Lcdbm8wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm8w::R`](R) reader structure"]
impl crate::Readable for Lcdbm8wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm8w::W`](W) writer structure"]
impl crate::Writable for Lcdbm8wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM8W to value 0"]
impl crate::Resettable for Lcdbm8wSpec {}
