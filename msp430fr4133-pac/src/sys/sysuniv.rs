#[doc = "Register `SYSUNIV` reader"]
pub type R = crate::R<SysunivSpec>;
#[doc = "Register `SYSUNIV` writer"]
pub type W = crate::W<SysunivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "User NMI vector generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysuniv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysuniv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysunivSpec;
impl crate::RegisterSpec for SysunivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysuniv::R`](R) reader structure"]
impl crate::Readable for SysunivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysuniv::W`](W) writer structure"]
impl crate::Writable for SysunivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSUNIV to value 0"]
impl crate::Resettable for SysunivSpec {}
