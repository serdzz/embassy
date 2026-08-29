#[doc = "Register `AESAXDIN` reader"]
pub type R = crate::R<AesaxdinSpec>;
#[doc = "Register `AESAXDIN` writer"]
pub type W = crate::W<AesaxdinSpec>;
#[doc = "Field `AESXDIN0` reader - AES data in byte n when AESAXDIN is written as half-word"]
pub type Aesxdin0R = crate::FieldReader;
#[doc = "Field `AESXDIN0` writer - AES data in byte n when AESAXDIN is written as half-word"]
pub type Aesxdin0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESXDIN1` reader - AES data in byte n+1 when AESAXDIN is written as half-word"]
pub type Aesxdin1R = crate::FieldReader;
#[doc = "Field `AESXDIN1` writer - AES data in byte n+1 when AESAXDIN is written as half-word"]
pub type Aesxdin1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin0(&self) -> Aesxdin0R {
        Aesxdin0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin1(&self) -> Aesxdin1R {
        Aesxdin1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin0(&mut self) -> Aesxdin0W<'_, AesaxdinSpec> {
        Aesxdin0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin1(&mut self) -> Aesxdin1W<'_, AesaxdinSpec> {
        Aesxdin1W::new(self, 8)
    }
}
#[doc = "AES Accelerator XORed Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesaxdin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxdin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesaxdinSpec;
impl crate::RegisterSpec for AesaxdinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesaxdin::R`](R) reader structure"]
impl crate::Readable for AesaxdinSpec {}
#[doc = "`write(|w| ..)` method takes [`aesaxdin::W`](W) writer structure"]
impl crate::Writable for AesaxdinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESAXDIN to value 0"]
impl crate::Resettable for AesaxdinSpec {}
