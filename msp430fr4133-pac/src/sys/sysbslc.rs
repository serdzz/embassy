#[doc = "Register `SYSBSLC` reader"]
pub type R = crate::R<SysbslcSpec>;
#[doc = "Register `SYSBSLC` writer"]
pub type W = crate::W<SysbslcSpec>;
#[doc = "Field `SYSBSLR` reader - SYS - RAM assigned to BSL"]
pub type SysbslrR = crate::BitReader;
#[doc = "Field `SYSBSLR` writer - SYS - RAM assigned to BSL"]
pub type SysbslrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLOFF` reader - SYS - BSL Memory disabled"]
pub type SysbsloffR = crate::BitReader;
#[doc = "Field `SYSBSLOFF` writer - SYS - BSL Memory disabled"]
pub type SysbsloffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBSLPE` reader - SYS - BSL Memory protection enabled"]
pub type SysbslpeR = crate::BitReader;
#[doc = "Field `SYSBSLPE` writer - SYS - BSL Memory protection enabled"]
pub type SysbslpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SysbslrR {
        SysbslrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SysbsloffR {
        SysbsloffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SysbslpeR {
        SysbslpeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&mut self) -> SysbslrW<'_, SysbslcSpec> {
        SysbslrW::new(self, 2)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&mut self) -> SysbsloffW<'_, SysbslcSpec> {
        SysbsloffW::new(self, 14)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&mut self) -> SysbslpeW<'_, SysbslcSpec> {
        SysbslpeW::new(self, 15)
    }
}
#[doc = "Boot strap configuration area\n\nYou can [`read`](crate::Reg::read) this register and get [`sysbslc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysbslc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysbslcSpec;
impl crate::RegisterSpec for SysbslcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysbslc::R`](R) reader structure"]
impl crate::Readable for SysbslcSpec {}
#[doc = "`write(|w| ..)` method takes [`sysbslc::W`](W) writer structure"]
impl crate::Writable for SysbslcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSBSLC to value 0"]
impl crate::Resettable for SysbslcSpec {}
