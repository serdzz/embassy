#[doc = "Register `AESADIN` reader"]
pub type R = crate::R<AesadinSpec>;
#[doc = "Register `AESADIN` writer"]
pub type W = crate::W<AesadinSpec>;
#[doc = "Field `AESDIN0` reader - AES data in byte n when AESADIN is written as half-word"]
pub type Aesdin0R = crate::FieldReader;
#[doc = "Field `AESDIN0` writer - AES data in byte n when AESADIN is written as half-word"]
pub type Aesdin0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESDIN1` reader - AES data in byte n+1 when AESADIN is written as half-word"]
pub type Aesdin1R = crate::FieldReader;
#[doc = "Field `AESDIN1` writer - AES data in byte n+1 when AESADIN is written as half-word"]
pub type Aesdin1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES data in byte n when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin0(&self) -> Aesdin0R {
        Aesdin0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin1(&self) -> Aesdin1R {
        Aesdin1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin0(&mut self) -> Aesdin0W<'_, AesadinSpec> {
        Aesdin0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin1(&mut self) -> Aesdin1W<'_, AesadinSpec> {
        Aesdin1W::new(self, 8)
    }
}
#[doc = "AES Accelerator Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadinSpec;
impl crate::RegisterSpec for AesadinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesadin::R`](R) reader structure"]
impl crate::Readable for AesadinSpec {}
#[doc = "`write(|w| ..)` method takes [`aesadin::W`](W) writer structure"]
impl crate::Writable for AesadinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADIN to value 0"]
impl crate::Resettable for AesadinSpec {}
