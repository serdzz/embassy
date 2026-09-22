#[doc = "Register `PMMCTL0` reader"]
pub type R = crate::R<Pmmctl0Spec>;
#[doc = "Register `PMMCTL0` writer"]
pub type W = crate::W<Pmmctl0Spec>;
#[doc = "Field `PMMSWBOR` reader - PMM Software BOR"]
pub type PmmswborR = crate::BitReader;
#[doc = "Field `PMMSWBOR` writer - PMM Software BOR"]
pub type PmmswborW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMSWPOR` reader - PMM Software POR"]
pub type PmmswporR = crate::BitReader;
#[doc = "Field `PMMSWPOR` writer - PMM Software POR"]
pub type PmmswporW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMREGOFF` reader - PMM Turn Regulator off"]
pub type PmmregoffR = crate::BitReader;
#[doc = "Field `PMMREGOFF` writer - PMM Turn Regulator off"]
pub type PmmregoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSHE` reader - SVS high side enable"]
pub type SvsheR = crate::BitReader;
#[doc = "Field `SVSHE` writer - SVS high side enable"]
pub type SvsheW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PmmswborR {
        PmmswborR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PmmswporR {
        PmmswporR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PmmregoffR {
        PmmregoffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SvsheR {
        SvsheR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PmmswborW<'_, Pmmctl0Spec> {
        PmmswborW::new(self, 2)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PmmswporW<'_, Pmmctl0Spec> {
        PmmswporW::new(self, 3)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PmmregoffW<'_, Pmmctl0Spec> {
        PmmregoffW::new(self, 4)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&mut self) -> SvsheW<'_, Pmmctl0Spec> {
        SvsheW::new(self, 6)
    }
}
#[doc = "PMM Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl0Spec;
impl crate::RegisterSpec for Pmmctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl0::R`](R) reader structure"]
impl crate::Readable for Pmmctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl0::W`](W) writer structure"]
impl crate::Writable for Pmmctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for Pmmctl0Spec {}
