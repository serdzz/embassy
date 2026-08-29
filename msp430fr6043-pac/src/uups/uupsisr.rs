#[doc = "Register `UUPSISR` reader"]
pub type R = crate::R<UupsisrSpec>;
#[doc = "Register `UUPSISR` writer"]
pub type W = crate::W<UupsisrSpec>;
#[doc = "Field `PTMOUT` reader - UUPS Power Up Time Out Interrupt Set bit."]
pub type PtmoutR = crate::BitReader;
#[doc = "Field `PTMOUT` writer - UUPS Power Up Time Out Interrupt Set bit."]
pub type PtmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREQIG` reader - Power Request Ignored Interrupt Set bit."]
pub type PreqigR = crate::BitReader;
#[doc = "Field `PREQIG` writer - Power Request Ignored Interrupt Set bit."]
pub type PreqigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPBYDB` reader - USS has been interrupted by debug mode Interrupt Set bit."]
pub type StpbydbR = crate::BitReader;
#[doc = "Field `STPBYDB` writer - USS has been interrupted by debug mode Interrupt Set bit."]
pub type StpbydbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UUPS Power Up Time Out Interrupt Set bit."]
    #[inline(always)]
    pub fn ptmout(&self) -> PtmoutR {
        PtmoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Request Ignored Interrupt Set bit."]
    #[inline(always)]
    pub fn preqig(&self) -> PreqigR {
        PreqigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USS has been interrupted by debug mode Interrupt Set bit."]
    #[inline(always)]
    pub fn stpbydb(&self) -> StpbydbR {
        StpbydbR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UUPS Power Up Time Out Interrupt Set bit."]
    #[inline(always)]
    pub fn ptmout(&mut self) -> PtmoutW<'_, UupsisrSpec> {
        PtmoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Request Ignored Interrupt Set bit."]
    #[inline(always)]
    pub fn preqig(&mut self) -> PreqigW<'_, UupsisrSpec> {
        PreqigW::new(self, 1)
    }
    #[doc = "Bit 2 - USS has been interrupted by debug mode Interrupt Set bit."]
    #[inline(always)]
    pub fn stpbydb(&mut self) -> StpbydbW<'_, UupsisrSpec> {
        StpbydbW::new(self, 2)
    }
}
#[doc = "Interrupt Flag Set Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`uupsisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uupsisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UupsisrSpec;
impl crate::RegisterSpec for UupsisrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uupsisr::R`](R) reader structure"]
impl crate::Readable for UupsisrSpec {}
#[doc = "`write(|w| ..)` method takes [`uupsisr::W`](W) writer structure"]
impl crate::Writable for UupsisrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UUPSISR to value 0"]
impl crate::Resettable for UupsisrSpec {}
