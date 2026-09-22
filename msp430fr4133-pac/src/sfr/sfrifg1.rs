#[doc = "Register `SFRIFG1` reader"]
pub type R = crate::R<Sfrifg1Spec>;
#[doc = "Register `SFRIFG1` writer"]
pub type W = crate::W<Sfrifg1Spec>;
#[doc = "Field `WDTIFG` reader - WDT Interrupt Flag"]
pub type WdtifgR = crate::BitReader;
#[doc = "Field `WDTIFG` writer - WDT Interrupt Flag"]
pub type WdtifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFIFG` reader - Osc Fault Flag"]
pub type OfifgR = crate::BitReader;
#[doc = "Field `OFIFG` writer - Osc Fault Flag"]
pub type OfifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMAIFG` reader - Vacant Memory Interrupt Flag"]
pub type VmaifgR = crate::BitReader;
#[doc = "Field `VMAIFG` writer - Vacant Memory Interrupt Flag"]
pub type VmaifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub type NmiifgR = crate::BitReader;
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub type NmiifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBINIFG` reader - JTAG Mail Box input Interrupt Flag"]
pub type JmbinifgR = crate::BitReader;
#[doc = "Field `JMBINIFG` writer - JTAG Mail Box input Interrupt Flag"]
pub type JmbinifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUTIFG` reader - JTAG Mail Box output Interrupt Flag"]
pub type JmboutifgR = crate::BitReader;
#[doc = "Field `JMBOUTIFG` writer - JTAG Mail Box output Interrupt Flag"]
pub type JmboutifgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WdtifgR {
        WdtifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OfifgR {
        OfifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    pub fn vmaifg(&self) -> VmaifgR {
        VmaifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NmiifgR {
        NmiifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    pub fn jmbinifg(&self) -> JmbinifgR {
        JmbinifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    pub fn jmboutifg(&self) -> JmboutifgR {
        JmboutifgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WdtifgW<'_, Sfrifg1Spec> {
        WdtifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OfifgW<'_, Sfrifg1Spec> {
        OfifgW::new(self, 1)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    pub fn vmaifg(&mut self) -> VmaifgW<'_, Sfrifg1Spec> {
        VmaifgW::new(self, 3)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NmiifgW<'_, Sfrifg1Spec> {
        NmiifgW::new(self, 4)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    pub fn jmbinifg(&mut self) -> JmbinifgW<'_, Sfrifg1Spec> {
        JmbinifgW::new(self, 6)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    pub fn jmboutifg(&mut self) -> JmboutifgW<'_, Sfrifg1Spec> {
        JmboutifgW::new(self, 7)
    }
}
#[doc = "Interrupt Flag 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrifg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrifg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sfrifg1Spec;
impl crate::RegisterSpec for Sfrifg1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrifg1::R`](R) reader structure"]
impl crate::Readable for Sfrifg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfrifg1::W`](W) writer structure"]
impl crate::Writable for Sfrifg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRIFG1 to value 0"]
impl crate::Resettable for Sfrifg1Spec {}
