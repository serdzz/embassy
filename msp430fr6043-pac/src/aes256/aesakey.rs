#[doc = "Register `AESAKEY` reader"]
pub type R = crate::R<AesakeySpec>;
#[doc = "Register `AESAKEY` writer"]
pub type W = crate::W<AesakeySpec>;
#[doc = "Field `AESKEY0` reader - AES key byte n when AESAKEY is written as half-word"]
pub type Aeskey0R = crate::FieldReader;
#[doc = "Field `AESKEY0` writer - AES key byte n when AESAKEY is written as half-word"]
pub type Aeskey0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESKEY1` reader - AES key byte n+1 when AESAKEY is written as half-word"]
pub type Aeskey1R = crate::FieldReader;
#[doc = "Field `AESKEY1` writer - AES key byte n+1 when AESAKEY is written as half-word"]
pub type Aeskey1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES key byte n when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey0(&self) -> Aeskey0R {
        Aeskey0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AES key byte n+1 when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey1(&self) -> Aeskey1R {
        Aeskey1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES key byte n when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey0(&mut self) -> Aeskey0W<'_, AesakeySpec> {
        Aeskey0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES key byte n+1 when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey1(&mut self) -> Aeskey1W<'_, AesakeySpec> {
        Aeskey1W::new(self, 8)
    }
}
#[doc = "AES Accelerator Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesakey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesakey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesakeySpec;
impl crate::RegisterSpec for AesakeySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesakey::R`](R) reader structure"]
impl crate::Readable for AesakeySpec {}
#[doc = "`write(|w| ..)` method takes [`aesakey::W`](W) writer structure"]
impl crate::Writable for AesakeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESAKEY to value 0"]
impl crate::Resettable for AesakeySpec {}
