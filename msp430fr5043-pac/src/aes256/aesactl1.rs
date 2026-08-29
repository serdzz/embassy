#[doc = "Register `AESACTL1` reader"]
pub type R = crate::R<Aesactl1Spec>;
#[doc = "Register `AESACTL1` writer"]
pub type W = crate::W<Aesactl1Spec>;
#[doc = "Field `AESBLKCNT` reader - Cipher Block Counter"]
pub type AesblkcntR = crate::FieldReader;
#[doc = "Field `AESBLKCNT` writer - Cipher Block Counter"]
pub type AesblkcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcnt(&self) -> AesblkcntR {
        AesblkcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcnt(&mut self) -> AesblkcntW<'_, Aesactl1Spec> {
        AesblkcntW::new(self, 0)
    }
}
#[doc = "AES Accelerator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesactl1Spec;
impl crate::RegisterSpec for Aesactl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesactl1::R`](R) reader structure"]
impl crate::Readable for Aesactl1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesactl1::W`](W) writer structure"]
impl crate::Writable for Aesactl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESACTL1 to value 0"]
impl crate::Resettable for Aesactl1Spec {}
