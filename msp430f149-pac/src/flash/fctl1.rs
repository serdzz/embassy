#[doc = "Register `FCTL1` reader"]
pub type R = crate::R<Fctl1Spec>;
#[doc = "Register `FCTL1` writer"]
pub type W = crate::W<Fctl1Spec>;
#[doc = "Field `ERASE` reader - Enable bit for Flash segment erase"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - Enable bit for Flash segment erase"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERAS` reader - Enable bit for Flash mass erase"]
pub type MerasR = crate::BitReader;
#[doc = "Field `MERAS` writer - Enable bit for Flash mass erase"]
pub type MerasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRT` reader - Enable bit for Flash write"]
pub type WrtR = crate::BitReader;
#[doc = "Field `WRT` writer - Enable bit for Flash write"]
pub type WrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKWRT` reader - Enable bit for Flash segment write"]
pub type BlkwrtR = crate::BitReader;
#[doc = "Field `BLKWRT` writer - Enable bit for Flash segment write"]
pub type BlkwrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MerasR {
        MerasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WrtR {
        WrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BlkwrtR {
        BlkwrtR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, Fctl1Spec> {
        EraseW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&mut self) -> MerasW<'_, Fctl1Spec> {
        MerasW::new(self, 2)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&mut self) -> WrtW<'_, Fctl1Spec> {
        WrtW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&mut self) -> BlkwrtW<'_, Fctl1Spec> {
        BlkwrtW::new(self, 7)
    }
}
#[doc = "FLASH Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctl1Spec;
impl crate::RegisterSpec for Fctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl1::R`](R) reader structure"]
impl crate::Readable for Fctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`fctl1::W`](W) writer structure"]
impl crate::Writable for Fctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTL1 to value 0"]
impl crate::Resettable for Fctl1Spec {}
