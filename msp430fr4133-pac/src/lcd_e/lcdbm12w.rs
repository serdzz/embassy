#[doc = "Register `LCDBM12W` reader"]
pub type R = crate::R<Lcdbm12wSpec>;
#[doc = "Register `LCDBM12W` writer"]
pub type W = crate::W<Lcdbm12wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 12/13\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm12w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm12w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm12wSpec;
impl crate::RegisterSpec for Lcdbm12wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm12w::R`](R) reader structure"]
impl crate::Readable for Lcdbm12wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm12w::W`](W) writer structure"]
impl crate::Writable for Lcdbm12wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM12W to value 0"]
impl crate::Resettable for Lcdbm12wSpec {}
