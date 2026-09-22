#[doc = "Register `LCDBM4W` reader"]
pub type R = crate::R<Lcdbm4wSpec>;
#[doc = "Register `LCDBM4W` writer"]
pub type W = crate::W<Lcdbm4wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 4/5\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm4w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm4w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm4wSpec;
impl crate::RegisterSpec for Lcdbm4wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm4w::R`](R) reader structure"]
impl crate::Readable for Lcdbm4wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm4w::W`](W) writer structure"]
impl crate::Writable for Lcdbm4wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM4W to value 0"]
impl crate::Resettable for Lcdbm4wSpec {}
