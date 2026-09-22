#[doc = "Register `SYSCTL` reader"]
pub type R = crate::R<SysctlSpec>;
#[doc = "Register `SYSCTL` writer"]
pub type W = crate::W<SysctlSpec>;
#[doc = "Field `SYSRIVECT` reader - SYS - RAM based interrupt vectors"]
pub type SysrivectR = crate::BitReader;
#[doc = "Field `SYSRIVECT` writer - SYS - RAM based interrupt vectors"]
pub type SysrivectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSPMMPE` reader - SYS - PMM access protect"]
pub type SyspmmpeR = crate::BitReader;
#[doc = "Field `SYSPMMPE` writer - SYS - PMM access protect"]
pub type SyspmmpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLIND` reader - SYS - TCK/RST indication detected"]
pub type SysbslindR = crate::BitReader;
#[doc = "Field `SYSBSLIND` writer - SYS - TCK/RST indication detected"]
pub type SysbslindW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSJTAGPIN` reader - SYS - Dedicated JTAG pins enabled"]
pub type SysjtagpinR = crate::BitReader;
#[doc = "Field `SYSJTAGPIN` writer - SYS - Dedicated JTAG pins enabled"]
pub type SysjtagpinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SysrivectR {
        SysrivectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SyspmmpeR {
        SyspmmpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SysbslindR {
        SysbslindR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SysjtagpinR {
        SysjtagpinR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&mut self) -> SysrivectW<'_, SysctlSpec> {
        SysrivectW::new(self, 0)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&mut self) -> SyspmmpeW<'_, SysctlSpec> {
        SyspmmpeW::new(self, 2)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&mut self) -> SysbslindW<'_, SysctlSpec> {
        SysbslindW::new(self, 4)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&mut self) -> SysjtagpinW<'_, SysctlSpec> {
        SysjtagpinW::new(self, 5)
    }
}
#[doc = "System control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctlSpec;
impl crate::RegisterSpec for SysctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysctl::R`](R) reader structure"]
impl crate::Readable for SysctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctl::W`](W) writer structure"]
impl crate::Writable for SysctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCTL to value 0"]
impl crate::Resettable for SysctlSpec {}
