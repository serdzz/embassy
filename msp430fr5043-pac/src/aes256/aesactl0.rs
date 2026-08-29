#[doc = "Register `AESACTL0` reader"]
pub type R = crate::R<Aesactl0Spec>;
#[doc = "Register `AESACTL0` writer"]
pub type W = crate::W<Aesactl0Spec>;
#[doc = "AES operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesop {
    #[doc = "0: Encryption"]
    Aesop0 = 0,
    #[doc = "1: Decryption. The provided key is the same key used for encryption"]
    Aesop1 = 1,
    #[doc = "2: Generate first round key required for decryption"]
    Aesop2 = 2,
    #[doc = "3: Decryption. The provided key is the first round key required for decryption"]
    Aesop3 = 3,
}
impl From<Aesop> for u8 {
    #[inline(always)]
    fn from(variant: Aesop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesop {
    type Ux = u8;
}
impl crate::IsEnum for Aesop {}
#[doc = "Field `AESOP` reader - AES operation"]
pub type AesopR = crate::FieldReader<Aesop>;
impl AesopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesop {
        match self.bits {
            0 => Aesop::Aesop0,
            1 => Aesop::Aesop1,
            2 => Aesop::Aesop2,
            3 => Aesop::Aesop3,
            _ => unreachable!(),
        }
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn is_aesop_0(&self) -> bool {
        *self == Aesop::Aesop0
    }
    #[doc = "Decryption. The provided key is the same key used for encryption"]
    #[inline(always)]
    pub fn is_aesop_1(&self) -> bool {
        *self == Aesop::Aesop1
    }
    #[doc = "Generate first round key required for decryption"]
    #[inline(always)]
    pub fn is_aesop_2(&self) -> bool {
        *self == Aesop::Aesop2
    }
    #[doc = "Decryption. The provided key is the first round key required for decryption"]
    #[inline(always)]
    pub fn is_aesop_3(&self) -> bool {
        *self == Aesop::Aesop3
    }
}
#[doc = "Field `AESOP` writer - AES operation"]
pub type AesopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aesop, crate::Safe>;
impl<'a, REG> AesopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn aesop_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesop::Aesop0)
    }
    #[doc = "Decryption. The provided key is the same key used for encryption"]
    #[inline(always)]
    pub fn aesop_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesop::Aesop1)
    }
    #[doc = "Generate first round key required for decryption"]
    #[inline(always)]
    pub fn aesop_2(self) -> &'a mut crate::W<REG> {
        self.variant(Aesop::Aesop2)
    }
    #[doc = "Decryption. The provided key is the first round key required for decryption"]
    #[inline(always)]
    pub fn aesop_3(self) -> &'a mut crate::W<REG> {
        self.variant(Aesop::Aesop3)
    }
}
#[doc = "AES key length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aeskl {
    #[doc = "0: AES128. The key size is 128 bit"]
    _128 = 0,
    #[doc = "1: AES192. The key size is 192 bit."]
    _192 = 1,
    #[doc = "2: AES256. The key size is 256 bit"]
    _256 = 2,
}
impl From<Aeskl> for u8 {
    #[inline(always)]
    fn from(variant: Aeskl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aeskl {
    type Ux = u8;
}
impl crate::IsEnum for Aeskl {}
#[doc = "Field `AESKL` reader - AES key length"]
pub type AesklR = crate::FieldReader<Aeskl>;
impl AesklR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aeskl> {
        match self.bits {
            0 => Some(Aeskl::_128),
            1 => Some(Aeskl::_192),
            2 => Some(Aeskl::_256),
            _ => None,
        }
    }
    #[doc = "AES128. The key size is 128 bit"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Aeskl::_128
    }
    #[doc = "AES192. The key size is 192 bit."]
    #[inline(always)]
    pub fn is_192(&self) -> bool {
        *self == Aeskl::_192
    }
    #[doc = "AES256. The key size is 256 bit"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Aeskl::_256
    }
}
#[doc = "Field `AESKL` writer - AES key length"]
pub type AesklW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aeskl>;
impl<'a, REG> AesklW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES128. The key size is 128 bit"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskl::_128)
    }
    #[doc = "AES192. The key size is 192 bit."]
    #[inline(always)]
    pub fn _192(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskl::_192)
    }
    #[doc = "AES256. The key size is 256 bit"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Aeskl::_256)
    }
}
#[doc = "AES cipher mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aescm {
    #[doc = "0: ECB"]
    Ecb = 0,
    #[doc = "1: CBC"]
    Cbc = 1,
    #[doc = "2: OFB"]
    Ofb = 2,
    #[doc = "3: CFB"]
    Cfb = 3,
}
impl From<Aescm> for u8 {
    #[inline(always)]
    fn from(variant: Aescm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aescm {
    type Ux = u8;
}
impl crate::IsEnum for Aescm {}
#[doc = "Field `AESCM` reader - AES cipher mode select"]
pub type AescmR = crate::FieldReader<Aescm>;
impl AescmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aescm {
        match self.bits {
            0 => Aescm::Ecb,
            1 => Aescm::Cbc,
            2 => Aescm::Ofb,
            3 => Aescm::Cfb,
            _ => unreachable!(),
        }
    }
    #[doc = "ECB"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == Aescm::Ecb
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == Aescm::Cbc
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == Aescm::Ofb
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == Aescm::Cfb
    }
}
#[doc = "Field `AESCM` writer - AES cipher mode select"]
pub type AescmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aescm, crate::Safe>;
impl<'a, REG> AescmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Aescm::Ecb)
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Aescm::Cbc)
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut crate::W<REG> {
        self.variant(Aescm::Ofb)
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut crate::W<REG> {
        self.variant(Aescm::Cfb)
    }
}
#[doc = "AES software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesswrst {
    #[doc = "0: No reset"]
    Aesswrst0 = 0,
    #[doc = "1: Reset AES accelerator module"]
    Reset = 1,
}
impl From<Aesswrst> for bool {
    #[inline(always)]
    fn from(variant: Aesswrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESSWRST` reader - AES software reset"]
pub type AesswrstR = crate::BitReader<Aesswrst>;
impl AesswrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesswrst {
        match self.bits {
            false => Aesswrst::Aesswrst0,
            true => Aesswrst::Reset,
        }
    }
    #[doc = "No reset"]
    #[inline(always)]
    pub fn is_aesswrst_0(&self) -> bool {
        *self == Aesswrst::Aesswrst0
    }
    #[doc = "Reset AES accelerator module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Aesswrst::Reset
    }
}
#[doc = "Field `AESSWRST` writer - AES software reset"]
pub type AesswrstW<'a, REG> = crate::BitWriter<'a, REG, Aesswrst>;
impl<'a, REG> AesswrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset"]
    #[inline(always)]
    pub fn aesswrst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesswrst::Aesswrst0)
    }
    #[doc = "Reset AES accelerator module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Aesswrst::Reset)
    }
}
#[doc = "AES ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesrdyifg {
    #[doc = "0: No interrupt pending"]
    Aesrdyifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Aesrdyifg1 = 1,
}
impl From<Aesrdyifg> for bool {
    #[inline(always)]
    fn from(variant: Aesrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRDYIFG` reader - AES ready interrupt flag"]
pub type AesrdyifgR = crate::BitReader<Aesrdyifg>;
impl AesrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesrdyifg {
        match self.bits {
            false => Aesrdyifg::Aesrdyifg0,
            true => Aesrdyifg::Aesrdyifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_aesrdyifg_0(&self) -> bool {
        *self == Aesrdyifg::Aesrdyifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_aesrdyifg_1(&self) -> bool {
        *self == Aesrdyifg::Aesrdyifg1
    }
}
#[doc = "Field `AESRDYIFG` writer - AES ready interrupt flag"]
pub type AesrdyifgW<'a, REG> = crate::BitWriter<'a, REG, Aesrdyifg>;
impl<'a, REG> AesrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyifg::Aesrdyifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyifg::Aesrdyifg1)
    }
}
#[doc = "AES error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aeserrfg {
    #[doc = "0: No error"]
    Aeserrfg0 = 0,
    #[doc = "1: Error occurred"]
    Aeserrfg1 = 1,
}
impl From<Aeserrfg> for bool {
    #[inline(always)]
    fn from(variant: Aeserrfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESERRFG` reader - AES error flag"]
pub type AeserrfgR = crate::BitReader<Aeserrfg>;
impl AeserrfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aeserrfg {
        match self.bits {
            false => Aeserrfg::Aeserrfg0,
            true => Aeserrfg::Aeserrfg1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_aeserrfg_0(&self) -> bool {
        *self == Aeserrfg::Aeserrfg0
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_aeserrfg_1(&self) -> bool {
        *self == Aeserrfg::Aeserrfg1
    }
}
#[doc = "Field `AESERRFG` writer - AES error flag"]
pub type AeserrfgW<'a, REG> = crate::BitWriter<'a, REG, Aeserrfg>;
impl<'a, REG> AeserrfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn aeserrfg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeserrfg::Aeserrfg0)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn aeserrfg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aeserrfg::Aeserrfg1)
    }
}
#[doc = "AES ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesrdyie {
    #[doc = "0: Interrupt disabled"]
    Disable = 0,
    #[doc = "1: Interrupt enabled"]
    Enable = 1,
}
impl From<Aesrdyie> for bool {
    #[inline(always)]
    fn from(variant: Aesrdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRDYIE` reader - AES ready interrupt enable"]
pub type AesrdyieR = crate::BitReader<Aesrdyie>;
impl AesrdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesrdyie {
        match self.bits {
            false => Aesrdyie::Disable,
            true => Aesrdyie::Enable,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Aesrdyie::Disable
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Aesrdyie::Enable
    }
}
#[doc = "Field `AESRDYIE` writer - AES ready interrupt enable"]
pub type AesrdyieW<'a, REG> = crate::BitWriter<'a, REG, Aesrdyie>;
impl<'a, REG> AesrdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyie::Disable)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyie::Enable)
    }
}
#[doc = "AES cipher mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aescmen {
    #[doc = "0: No DMA triggers are generated"]
    Disable = 0,
    #[doc = "1: DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    Enable = 1,
}
impl From<Aescmen> for bool {
    #[inline(always)]
    fn from(variant: Aescmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESCMEN` reader - AES cipher mode enable"]
pub type AescmenR = crate::BitReader<Aescmen>;
impl AescmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aescmen {
        match self.bits {
            false => Aescmen::Disable,
            true => Aescmen::Enable,
        }
    }
    #[doc = "No DMA triggers are generated"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Aescmen::Disable
    }
    #[doc = "DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Aescmen::Enable
    }
}
#[doc = "Field `AESCMEN` writer - AES cipher mode enable"]
pub type AescmenW<'a, REG> = crate::BitWriter<'a, REG, Aescmen>;
impl<'a, REG> AescmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA triggers are generated"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmen::Disable)
    }
    #[doc = "DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesop(&self) -> AesopR {
        AesopR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aeskl(&self) -> AesklR {
        AesklR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescm(&self) -> AescmR {
        AescmR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&self) -> AesswrstR {
        AesswrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&self) -> AesrdyifgR {
        AesrdyifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&self) -> AeserrfgR {
        AeserrfgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&self) -> AesrdyieR {
        AesrdyieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&self) -> AescmenR {
        AescmenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesop(&mut self) -> AesopW<'_, Aesactl0Spec> {
        AesopW::new(self, 0)
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aeskl(&mut self) -> AesklW<'_, Aesactl0Spec> {
        AesklW::new(self, 2)
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescm(&mut self) -> AescmW<'_, Aesactl0Spec> {
        AescmW::new(self, 5)
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&mut self) -> AesswrstW<'_, Aesactl0Spec> {
        AesswrstW::new(self, 7)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&mut self) -> AesrdyifgW<'_, Aesactl0Spec> {
        AesrdyifgW::new(self, 8)
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&mut self) -> AeserrfgW<'_, Aesactl0Spec> {
        AeserrfgW::new(self, 11)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&mut self) -> AesrdyieW<'_, Aesactl0Spec> {
        AesrdyieW::new(self, 12)
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&mut self) -> AescmenW<'_, Aesactl0Spec> {
        AescmenW::new(self, 15)
    }
}
#[doc = "AES Accelerator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesactl0Spec;
impl crate::RegisterSpec for Aesactl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesactl0::R`](R) reader structure"]
impl crate::Readable for Aesactl0Spec {}
#[doc = "`write(|w| ..)` method takes [`aesactl0::W`](W) writer structure"]
impl crate::Writable for Aesactl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AESACTL0 to value 0"]
impl crate::Resettable for Aesactl0Spec {}
