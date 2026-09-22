#[doc = "Register `P2IV` reader"]
pub type R = crate::R<P2ivSpec>;
#[doc = "Register `P2IV` writer"]
pub type W = crate::W<P2ivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 2 Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`p2iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2ivSpec;
impl crate::RegisterSpec for P2ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p2iv::R`](R) reader structure"]
impl crate::Readable for P2ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p2iv::W`](W) writer structure"]
impl crate::Writable for P2ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2IV to value 0"]
impl crate::Resettable for P2ivSpec {}
