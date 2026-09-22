#[doc = "Register `LCDBM2W` reader"]
pub type R = crate::R<Lcdbm2wSpec>;
#[doc = "Register `LCDBM2W` writer"]
pub type W = crate::W<Lcdbm2wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm2w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm2w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm2wSpec;
impl crate::RegisterSpec for Lcdbm2wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm2w::R`](R) reader structure"]
impl crate::Readable for Lcdbm2wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm2w::W`](W) writer structure"]
impl crate::Writable for Lcdbm2wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM2W to value 0"]
impl crate::Resettable for Lcdbm2wSpec {}
