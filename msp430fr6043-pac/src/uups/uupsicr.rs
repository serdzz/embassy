#[doc = "Register `UUPSICR` reader"]
pub type R = crate::R<UupsicrSpec>;
#[doc = "Register `UUPSICR` writer"]
pub type W = crate::W<UupsicrSpec>;
#[doc = "Field `PTMOUT` reader - UUPS Power Up Time Out Interrupt Clear bit."]
pub type PtmoutR = crate::BitReader;
#[doc = "Field `PTMOUT` writer - UUPS Power Up Time Out Interrupt Clear bit."]
pub type PtmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREQIG` reader - Power Request Ignored Interrupt Clear bit."]
pub type PreqigR = crate::BitReader;
#[doc = "Field `PREQIG` writer - Power Request Ignored Interrupt Clear bit."]
pub type PreqigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPBYDB` reader - USS has been interrupted by debug mode Interrupt Clear bit."]
pub type StpbydbR = crate::BitReader;
#[doc = "Field `STPBYDB` writer - USS has been interrupted by debug mode Interrupt Clear bit."]
pub type StpbydbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UUPS Power Up Time Out Interrupt Clear bit."]
    #[inline(always)]
    pub fn ptmout(&self) -> PtmoutR {
        PtmoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Request Ignored Interrupt Clear bit."]
    #[inline(always)]
    pub fn preqig(&self) -> PreqigR {
        PreqigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USS has been interrupted by debug mode Interrupt Clear bit."]
    #[inline(always)]
    pub fn stpbydb(&self) -> StpbydbR {
        StpbydbR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UUPS Power Up Time Out Interrupt Clear bit."]
    #[inline(always)]
    pub fn ptmout(&mut self) -> PtmoutW<'_, UupsicrSpec> {
        PtmoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Request Ignored Interrupt Clear bit."]
    #[inline(always)]
    pub fn preqig(&mut self) -> PreqigW<'_, UupsicrSpec> {
        PreqigW::new(self, 1)
    }
    #[doc = "Bit 2 - USS has been interrupted by debug mode Interrupt Clear bit."]
    #[inline(always)]
    pub fn stpbydb(&mut self) -> StpbydbW<'_, UupsicrSpec> {
        StpbydbW::new(self, 2)
    }
}
#[doc = "Interrupt Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UupsicrSpec;
impl crate::RegisterSpec for UupsicrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uupsicr::R`](R) reader structure"]
impl crate::Readable for UupsicrSpec {}
#[doc = "`write(|w| ..)` method takes [`uupsicr::W`](W) writer structure"]
impl crate::Writable for UupsicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUPSICR to value 0"]
impl crate::Resettable for UupsicrSpec {}
