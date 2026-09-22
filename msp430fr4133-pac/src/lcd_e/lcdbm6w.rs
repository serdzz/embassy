#[doc = "Register `LCDBM6W` reader"]
pub type R = crate::R<Lcdbm6wSpec>;
#[doc = "Register `LCDBM6W` writer"]
pub type W = crate::W<Lcdbm6wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 6/7\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm6w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm6w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm6wSpec;
impl crate::RegisterSpec for Lcdbm6wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm6w::R`](R) reader structure"]
impl crate::Readable for Lcdbm6wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm6w::W`](W) writer structure"]
impl crate::Writable for Lcdbm6wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM6W to value 0"]
impl crate::Resettable for Lcdbm6wSpec {}
