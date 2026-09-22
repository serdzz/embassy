#[doc = "Register `CSCTL8` reader"]
pub type R = crate::R<Csctl8Spec>;
#[doc = "Register `CSCTL8` writer"]
pub type W = crate::W<Csctl8Spec>;
#[doc = "Field `ACLKREQEN` reader - ACLK Clock Request Enable"]
pub type AclkreqenR = crate::BitReader;
#[doc = "Field `ACLKREQEN` writer - ACLK Clock Request Enable"]
pub type AclkreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCLKREQEN` reader - MCLK Clock Request Enable"]
pub type MclkreqenR = crate::BitReader;
#[doc = "Field `MCLKREQEN` writer - MCLK Clock Request Enable"]
pub type MclkreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCLKREQEN` reader - SMCLK Clock Request Enable"]
pub type SmclkreqenR = crate::BitReader;
#[doc = "Field `SMCLKREQEN` writer - SMCLK Clock Request Enable"]
pub type SmclkreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODOSCREQEN` reader - MODOSC Clock Request Enable"]
pub type ModoscreqenR = crate::BitReader;
#[doc = "Field `MODOSCREQEN` writer - MODOSC Clock Request Enable"]
pub type ModoscreqenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> AclkreqenR {
        AclkreqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MclkreqenR {
        MclkreqenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SmclkreqenR {
        SmclkreqenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&self) -> ModoscreqenR {
        ModoscreqenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> AclkreqenW<'_, Csctl8Spec> {
        AclkreqenW::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MclkreqenW<'_, Csctl8Spec> {
        MclkreqenW::new(self, 1)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SmclkreqenW<'_, Csctl8Spec> {
        SmclkreqenW::new(self, 2)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&mut self) -> ModoscreqenW<'_, Csctl8Spec> {
        ModoscreqenW::new(self, 3)
    }
}
#[doc = "CS Control Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl8Spec;
impl crate::RegisterSpec for Csctl8Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl8::R`](R) reader structure"]
impl crate::Readable for Csctl8Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl8::W`](W) writer structure"]
impl crate::Writable for Csctl8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL8 to value 0"]
impl crate::Resettable for Csctl8Spec {}
