#[doc = "Register `LCDBM16W` reader"]
pub type R = crate::R<Lcdbm16wSpec>;
#[doc = "Register `LCDBM16W` writer"]
pub type W = crate::W<Lcdbm16wSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCD Blinking Memory 16/17\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdbm16w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdbm16w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdbm16wSpec;
impl crate::RegisterSpec for Lcdbm16wSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdbm16w::R`](R) reader structure"]
impl crate::Readable for Lcdbm16wSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdbm16w::W`](W) writer structure"]
impl crate::Writable for Lcdbm16wSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDBM16W to value 0"]
impl crate::Resettable for Lcdbm16wSpec {}
