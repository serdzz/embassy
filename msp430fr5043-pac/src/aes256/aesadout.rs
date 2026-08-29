#[doc = "Register `AESADOUT` reader"]
pub type R = crate::R<AesadoutSpec>;
#[doc = "Register `AESADOUT` writer"]
pub type W = crate::W<AesadoutSpec>;
#[doc = "Field `AESDOUT0` reader - AES data out byte n when AESADOUT is read as half-word"]
pub type Aesdout0R = crate::FieldReader;
#[doc = "Field `AESDOUT0` writer - AES data out byte n when AESADOUT is read as half-word"]
pub type Aesdout0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESDOUT1` reader - AES data out byte n+1 when AESADOUT is read as half-word"]
pub type Aesdout1R = crate::FieldReader;
#[doc = "Field `AESDOUT1` writer - AES data out byte n+1 when AESADOUT is read as half-word"]
pub type Aesdout1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES data out byte n when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout0(&self) -> Aesdout0R {
        Aesdout0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AES data out byte n+1 when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout1(&self) -> Aesdout1R {
        Aesdout1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data out byte n when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout0(&mut self) -> Aesdout0W<'_, AesadoutSpec> {
        Aesdout0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data out byte n+1 when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout1(&mut self) -> Aesdout1W<'_, AesadoutSpec> {
        Aesdout1W::new(self, 8)
    }
}
#[doc = "AES Accelerator Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadoutSpec;
impl crate::RegisterSpec for AesadoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesadout::R`](R) reader structure"]
impl crate::Readable for AesadoutSpec {}
#[doc = "`write(|w| ..)` method takes [`aesadout::W`](W) writer structure"]
impl crate::Writable for AesadoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESADOUT to value 0"]
impl crate::Resettable for AesadoutSpec {}
