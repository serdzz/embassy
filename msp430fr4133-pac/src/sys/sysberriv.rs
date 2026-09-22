#[doc = "Register `SYSBERRIV` reader"]
pub type R = crate::R<SysberrivSpec>;
#[doc = "Register `SYSBERRIV` writer"]
pub type W = crate::W<SysberrivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bus Error vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysberriv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysberriv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysberrivSpec;
impl crate::RegisterSpec for SysberrivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysberriv::R`](R) reader structure"]
impl crate::Readable for SysberrivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysberriv::W`](W) writer structure"]
impl crate::Writable for SysberrivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSBERRIV to value 0"]
impl crate::Resettable for SysberrivSpec {}
