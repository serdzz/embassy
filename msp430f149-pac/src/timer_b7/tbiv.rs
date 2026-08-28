#[doc = "Register `TBIV` reader"]
pub type R = crate::R<TbivSpec>;
#[doc = "Register `TBIV` writer"]
pub type W = crate::W<TbivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer B Interrupt Vector Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tbiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbivSpec;
impl crate::RegisterSpec for TbivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tbiv::R`](R) reader structure"]
impl crate::Readable for TbivSpec {}
#[doc = "`write(|w| ..)` method takes [`tbiv::W`](W) writer structure"]
impl crate::Writable for TbivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBIV to value 0"]
impl crate::Resettable for TbivSpec {}
