#[doc = "Register `AESAXIN` reader"]
pub type R = crate::R<AesaxinSpec>;
#[doc = "Register `AESAXIN` writer"]
pub type W = crate::W<AesaxinSpec>;
#[doc = "Field `AESXIN0` reader - AES data in byte n when AESAXIN is written as half-word"]
pub type Aesxin0R = crate::FieldReader;
#[doc = "Field `AESXIN0` writer - AES data in byte n when AESAXIN is written as half-word"]
pub type Aesxin0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESXIN1` reader - AES data in byte n+1 when AESAXIN is written as half-word"]
pub type Aesxin1R = crate::FieldReader;
#[doc = "Field `AESXIN1` writer - AES data in byte n+1 when AESAXIN is written as half-word"]
pub type Aesxin1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin0(&self) -> Aesxin0R {
        Aesxin0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin1(&self) -> Aesxin1R {
        Aesxin1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin0(&mut self) -> Aesxin0W<'_, AesaxinSpec> {
        Aesxin0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin1(&mut self) -> Aesxin1W<'_, AesaxinSpec> {
        Aesxin1W::new(self, 8)
    }
}
#[doc = "AES Accelerator XORed Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesaxin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesaxinSpec;
impl crate::RegisterSpec for AesaxinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesaxin::R`](R) reader structure"]
impl crate::Readable for AesaxinSpec {}
#[doc = "`write(|w| ..)` method takes [`aesaxin::W`](W) writer structure"]
impl crate::Writable for AesaxinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESAXIN to value 0"]
impl crate::Resettable for AesaxinSpec {}
