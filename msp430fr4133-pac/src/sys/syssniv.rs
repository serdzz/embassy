#[doc = "Register `SYSSNIV` reader"]
pub type R = crate::R<SyssnivSpec>;
#[doc = "Register `SYSSNIV` writer"]
pub type W = crate::W<SyssnivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "System NMI vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`syssniv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syssniv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyssnivSpec;
impl crate::RegisterSpec for SyssnivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syssniv::R`](R) reader structure"]
impl crate::Readable for SyssnivSpec {}
#[doc = "`write(|w| ..)` method takes [`syssniv::W`](W) writer structure"]
impl crate::Writable for SyssnivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSSNIV to value 0"]
impl crate::Resettable for SyssnivSpec {}
