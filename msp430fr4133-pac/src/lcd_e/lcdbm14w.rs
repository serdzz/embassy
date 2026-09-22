#[doc = "Register `LCDBM14W` reader"]
pub type R = crate::R<Lcdbm14wSpec>;
#[doc = "Register `LCDBM14W` writer"]
pub type W = crate::W<Lcdbm14wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 14/15\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm14w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm14w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm14wSpec;
impl crate::RegisterSpec for Lcdbm14wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm14w::R`](R) reader structure"]
impl crate::Readable for Lcdbm14wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm14w::W`](W) writer structure"]
impl crate::Writable for Lcdbm14wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM14W to value 0"]
impl crate::Resettable for Lcdbm14wSpec {}
