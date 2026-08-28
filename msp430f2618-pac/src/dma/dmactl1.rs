#[doc = "Register `DMACTL1` reader"]
pub type R = crate::R<Dmactl1Spec>;
#[doc = "Register `DMACTL1` writer"]
pub type W = crate::W<Dmactl1Spec>;
#[doc = "Field `ENNMI` reader - Enable NMI interruption of DMA"]
pub type EnnmiR = crate::BitReader;
#[doc = "Field `ENNMI` writer - Enable NMI interruption of DMA"]
pub type EnnmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROUNDROBIN` reader - Round-Robin DMA channel priorities"]
pub type RoundrobinR = crate::BitReader;
#[doc = "Field `ROUNDROBIN` writer - Round-Robin DMA channel priorities"]
pub type RoundrobinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAONFETCH` reader - DMA transfer on instruction fetch"]
pub type DmaonfetchR = crate::BitReader;
#[doc = "Field `DMAONFETCH` writer - DMA transfer on instruction fetch"]
pub type DmaonfetchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    pub fn ennmi(&self) -> EnnmiR {
        EnnmiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    pub fn roundrobin(&self) -> RoundrobinR {
        RoundrobinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA transfer on instruction fetch"]
    #[inline(always)]
    pub fn dmaonfetch(&self) -> DmaonfetchR {
        DmaonfetchR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    pub fn ennmi(&mut self) -> EnnmiW<'_, Dmactl1Spec> {
        EnnmiW::new(self, 0)
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    pub fn roundrobin(&mut self) -> RoundrobinW<'_, Dmactl1Spec> {
        RoundrobinW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA transfer on instruction fetch"]
    #[inline(always)]
    pub fn dmaonfetch(&mut self) -> DmaonfetchW<'_, Dmactl1Spec> {
        DmaonfetchW::new(self, 2)
    }
}
#[doc = "DMA Module Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmactl1Spec;
impl crate::RegisterSpec for Dmactl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl1::R`](R) reader structure"]
impl crate::Readable for Dmactl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmactl1::W`](W) writer structure"]
impl crate::Writable for Dmactl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTL1 to value 0"]
impl crate::Resettable for Dmactl1Spec {}
