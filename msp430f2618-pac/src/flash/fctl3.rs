#[doc = "Register `FCTL3` reader"]
pub type R = crate::R<Fctl3Spec>;
#[doc = "Register `FCTL3` writer"]
pub type W = crate::W<Fctl3Spec>;
#[doc = "Field `BUSY` reader - Flash busy: 1"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Flash busy: 1"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYV` reader - Flash Key violation flag"]
pub type KeyvR = crate::BitReader;
#[doc = "Field `KEYV` writer - Flash Key violation flag"]
pub type KeyvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCVIFG` reader - Flash Access violation flag"]
pub type AccvifgR = crate::BitReader;
#[doc = "Field `ACCVIFG` writer - Flash Access violation flag"]
pub type AccvifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT` reader - Wait flag for segment write"]
pub type WaitR = crate::BitReader;
#[doc = "Field `WAIT` writer - Wait flag for segment write"]
pub type WaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock bit: 1 - Flash is locked (read only)"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock bit: 1 - Flash is locked (read only)"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMEX` reader - Flash Emergency Exit"]
pub type EmexR = crate::BitReader;
#[doc = "Field `EMEX` writer - Flash Emergency Exit"]
pub type EmexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKA` reader - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LockaR = crate::BitReader;
#[doc = "Field `LOCKA` writer - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LockaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAIL` reader - Last Program or Erase failed"]
pub type FailR = crate::BitReader;
#[doc = "Field `FAIL` writer - Last Program or Erase failed"]
pub type FailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    pub fn keyv(&self) -> KeyvR {
        KeyvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    pub fn accvifg(&self) -> AccvifgR {
        AccvifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    pub fn emex(&self) -> EmexR {
        EmexR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn locka(&self) -> LockaR {
        LockaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Last Program or Erase failed"]
    #[inline(always)]
    pub fn fail(&self) -> FailR {
        FailR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<'_, Fctl3Spec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    pub fn keyv(&mut self) -> KeyvW<'_, Fctl3Spec> {
        KeyvW::new(self, 1)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    pub fn accvifg(&mut self) -> AccvifgW<'_, Fctl3Spec> {
        AccvifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<'_, Fctl3Spec> {
        WaitW::new(self, 3)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Fctl3Spec> {
        LockW::new(self, 4)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    pub fn emex(&mut self) -> EmexW<'_, Fctl3Spec> {
        EmexW::new(self, 5)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn locka(&mut self) -> LockaW<'_, Fctl3Spec> {
        LockaW::new(self, 6)
    }
    #[doc = "Bit 7 - Last Program or Erase failed"]
    #[inline(always)]
    pub fn fail(&mut self) -> FailW<'_, Fctl3Spec> {
        FailW::new(self, 7)
    }
}
#[doc = "FLASH Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctl3Spec;
impl crate::RegisterSpec for Fctl3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fctl3::R`](R) reader structure"]
impl crate::Readable for Fctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`fctl3::W`](W) writer structure"]
impl crate::Writable for Fctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTL3 to value 0"]
impl crate::Resettable for Fctl3Spec {}
