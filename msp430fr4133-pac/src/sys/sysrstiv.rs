#[doc = "Register `SYSRSTIV` reader"]
pub type R = crate::R<SysrstivSpec>;
#[doc = "Register `SYSRSTIV` writer"]
pub type W = crate::W<SysrstivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reset vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrstiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrstiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysrstivSpec;
impl crate::RegisterSpec for SysrstivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysrstiv::R`](R) reader structure"]
impl crate::Readable for SysrstivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysrstiv::W`](W) writer structure"]
impl crate::Writable for SysrstivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSRSTIV to value 0"]
impl crate::Resettable for SysrstivSpec {}
