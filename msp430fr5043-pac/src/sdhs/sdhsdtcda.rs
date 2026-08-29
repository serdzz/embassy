#[doc = "Register `SDHSDTCDA` reader"]
pub type R = crate::R<SdhsdtcdaSpec>;
#[doc = "Register `SDHSDTCDA` writer"]
pub type W = crate::W<SdhsdtcdaSpec>;
#[doc = "Field `DTCDA` reader - DTC destination address."]
pub type DtcdaR = crate::FieldReader<u16>;
#[doc = "Field `DTCDA` writer - DTC destination address."]
pub type DtcdaW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - DTC destination address."]
    #[inline(always)]
    pub fn dtcda(&self) -> DtcdaR {
        DtcdaR::new(self.bits & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 0:14 - DTC destination address."]
    #[inline(always)]
    pub fn dtcda(&mut self) -> DtcdaW<'_, SdhsdtcdaSpec> {
        DtcdaW::new(self, 0)
    }
}
#[doc = "DTC destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdhsdtcda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdhsdtcda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhsdtcdaSpec;
impl crate::RegisterSpec for SdhsdtcdaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhsdtcda::R`](R) reader structure"]
impl crate::Readable for SdhsdtcdaSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhsdtcda::W`](W) writer structure"]
impl crate::Writable for SdhsdtcdaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDHSDTCDA to value 0"]
impl crate::Resettable for SdhsdtcdaSpec {}
