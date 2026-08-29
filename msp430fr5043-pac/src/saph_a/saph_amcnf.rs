#[doc = "Register `SAPH_AMCNF` reader"]
pub type R = crate::R<SaphAmcnfSpec>;
#[doc = "Register `SAPH_AMCNF` writer"]
pub type W = crate::W<SaphAmcnfSpec>;
#[doc = "These bits define the impedance of the buffers for RxBias and TxBias. While for resistive loads the lowest impedance shows the fastest settling; is this not the case for reactive loads.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bimp {
    #[doc = "0: 200 Ohms buffer impedance for RxBias and TxBias"]
    Bimp0 = 0,
    #[doc = "1: 600 Ohms buffer impedance for RxBias and TxBias"]
    Bimp1 = 1,
    #[doc = "2: 1200 Ohms buffer impedance for RxBias and TxBias (default)"]
    Bimp2 = 2,
    #[doc = "3: 2800 Ohms buffer impedance for RxBias and TxBias"]
    Bimp3 = 3,
}
impl From<Bimp> for u8 {
    #[inline(always)]
    fn from(variant: Bimp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bimp {
    type Ux = u8;
}
impl crate::IsEnum for Bimp {}
#[doc = "Field `BIMP` reader - These bits define the impedance of the buffers for RxBias and TxBias. While for resistive loads the lowest impedance shows the fastest settling; is this not the case for reactive loads."]
pub type BimpR = crate::FieldReader<Bimp>;
impl BimpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bimp {
        match self.bits {
            0 => Bimp::Bimp0,
            1 => Bimp::Bimp1,
            2 => Bimp::Bimp2,
            3 => Bimp::Bimp3,
            _ => unreachable!(),
        }
    }
    #[doc = "200 Ohms buffer impedance for RxBias and TxBias"]
    #[inline(always)]
    pub fn is_bimp_0(&self) -> bool {
        *self == Bimp::Bimp0
    }
    #[doc = "600 Ohms buffer impedance for RxBias and TxBias"]
    #[inline(always)]
    pub fn is_bimp_1(&self) -> bool {
        *self == Bimp::Bimp1
    }
    #[doc = "1200 Ohms buffer impedance for RxBias and TxBias (default)"]
    #[inline(always)]
    pub fn is_bimp_2(&self) -> bool {
        *self == Bimp::Bimp2
    }
    #[doc = "2800 Ohms buffer impedance for RxBias and TxBias"]
    #[inline(always)]
    pub fn is_bimp_3(&self) -> bool {
        *self == Bimp::Bimp3
    }
}
#[doc = "Field `BIMP` writer - These bits define the impedance of the buffers for RxBias and TxBias. While for resistive loads the lowest impedance shows the fastest settling; is this not the case for reactive loads."]
pub type BimpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bimp, crate::Safe>;
impl<'a, REG> BimpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "200 Ohms buffer impedance for RxBias and TxBias"]
    #[inline(always)]
    pub fn bimp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bimp::Bimp0)
    }
    #[doc = "600 Ohms buffer impedance for RxBias and TxBias"]
    #[inline(always)]
    pub fn bimp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bimp::Bimp1)
    }
    #[doc = "1200 Ohms buffer impedance for RxBias and TxBias (default)"]
    #[inline(always)]
    pub fn bimp_2(self) -> &'a mut crate::W<REG> {
        self.variant(Bimp::Bimp2)
    }
    #[doc = "2800 Ohms buffer impedance for RxBias and TxBias"]
    #[inline(always)]
    pub fn bimp_3(self) -> &'a mut crate::W<REG> {
        self.variant(Bimp::Bimp3)
    }
}
#[doc = "Field `RSV0` reader - Reserved for future use"]
pub type Rsv0R = crate::FieldReader;
#[doc = "Field `RSV0` writer - Reserved for future use"]
pub type Rsv0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "This bit enables the charge pump of the input multiplexer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpeo {
    #[doc = "0: Charge pump is turned on by SDHS and ASQ related requests only."]
    Cpeo0 = 0,
    #[doc = "1: Charge pump is turned on regardless of SDHS and ASQ related charge pump requests."]
    Cpeo1 = 1,
}
impl From<Cpeo> for bool {
    #[inline(always)]
    fn from(variant: Cpeo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPEO` reader - This bit enables the charge pump of the input multiplexer."]
pub type CpeoR = crate::BitReader<Cpeo>;
impl CpeoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpeo {
        match self.bits {
            false => Cpeo::Cpeo0,
            true => Cpeo::Cpeo1,
        }
    }
    #[doc = "Charge pump is turned on by SDHS and ASQ related requests only."]
    #[inline(always)]
    pub fn is_cpeo_0(&self) -> bool {
        *self == Cpeo::Cpeo0
    }
    #[doc = "Charge pump is turned on regardless of SDHS and ASQ related charge pump requests."]
    #[inline(always)]
    pub fn is_cpeo_1(&self) -> bool {
        *self == Cpeo::Cpeo1
    }
}
#[doc = "Field `CPEO` writer - This bit enables the charge pump of the input multiplexer."]
pub type CpeoW<'a, REG> = crate::BitWriter<'a, REG, Cpeo>;
impl<'a, REG> CpeoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Charge pump is turned on by SDHS and ASQ related requests only."]
    #[inline(always)]
    pub fn cpeo_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpeo::Cpeo0)
    }
    #[doc = "Charge pump is turned on regardless of SDHS and ASQ related charge pump requests."]
    #[inline(always)]
    pub fn cpeo_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpeo::Cpeo1)
    }
}
#[doc = "Field `RSV1` reader - Reserved for future use"]
pub type Rsv1R = crate::FieldReader;
#[doc = "Field `RSV1` writer - Reserved for future use"]
pub type Rsv1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "LPBE, low power bias mode enable. This bit enables the low power bias operation mode. The selection of the operation mode shall only be changed while the PSQ is in OFF state (changes during other states of the PSQ causes corrupt measurement results and irregular triggers of sub modules ba ASQ)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpbe {
    #[doc = "0: For manual bias mode and regular ASQ bias mode. In this configuration the user controls by the ASQBSW has full control over the TxBias and RxBias switches."]
    Lpbe0 = 0,
    #[doc = "1: Low power bias mode. In this mode the ASQ uses the CHxEBSW and PGABSW as auxiliary values to achieve faster channel setting on reactive input loads. The ASQ has full controls over the bias switch multiplexer."]
    Lpbe1 = 1,
}
impl From<Lpbe> for bool {
    #[inline(always)]
    fn from(variant: Lpbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBE` reader - LPBE, low power bias mode enable. This bit enables the low power bias operation mode. The selection of the operation mode shall only be changed while the PSQ is in OFF state (changes during other states of the PSQ causes corrupt measurement results and irregular triggers of sub modules ba ASQ)"]
pub type LpbeR = crate::BitReader<Lpbe>;
impl LpbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpbe {
        match self.bits {
            false => Lpbe::Lpbe0,
            true => Lpbe::Lpbe1,
        }
    }
    #[doc = "For manual bias mode and regular ASQ bias mode. In this configuration the user controls by the ASQBSW has full control over the TxBias and RxBias switches."]
    #[inline(always)]
    pub fn is_lpbe_0(&self) -> bool {
        *self == Lpbe::Lpbe0
    }
    #[doc = "Low power bias mode. In this mode the ASQ uses the CHxEBSW and PGABSW as auxiliary values to achieve faster channel setting on reactive input loads. The ASQ has full controls over the bias switch multiplexer."]
    #[inline(always)]
    pub fn is_lpbe_1(&self) -> bool {
        *self == Lpbe::Lpbe1
    }
}
#[doc = "Field `LPBE` writer - LPBE, low power bias mode enable. This bit enables the low power bias operation mode. The selection of the operation mode shall only be changed while the PSQ is in OFF state (changes during other states of the PSQ causes corrupt measurement results and irregular triggers of sub modules ba ASQ)"]
pub type LpbeW<'a, REG> = crate::BitWriter<'a, REG, Lpbe>;
impl<'a, REG> LpbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "For manual bias mode and regular ASQ bias mode. In this configuration the user controls by the ASQBSW has full control over the TxBias and RxBias switches."]
    #[inline(always)]
    pub fn lpbe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpbe::Lpbe0)
    }
    #[doc = "Low power bias mode. In this mode the ASQ uses the CHxEBSW and PGABSW as auxiliary values to achieve faster channel setting on reactive input loads. The ASQ has full controls over the bias switch multiplexer."]
    #[inline(always)]
    pub fn lpbe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpbe::Lpbe1)
    }
}
impl R {
    #[doc = "Bits 0:1 - These bits define the impedance of the buffers for RxBias and TxBias. While for resistive loads the lowest impedance shows the fastest settling; is this not the case for reactive loads."]
    #[inline(always)]
    pub fn bimp(&self) -> BimpR {
        BimpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Reserved for future use"]
    #[inline(always)]
    pub fn rsv0(&self) -> Rsv0R {
        Rsv0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - This bit enables the charge pump of the input multiplexer."]
    #[inline(always)]
    pub fn cpeo(&self) -> CpeoR {
        CpeoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Reserved for future use"]
    #[inline(always)]
    pub fn rsv1(&self) -> Rsv1R {
        Rsv1R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - LPBE, low power bias mode enable. This bit enables the low power bias operation mode. The selection of the operation mode shall only be changed while the PSQ is in OFF state (changes during other states of the PSQ causes corrupt measurement results and irregular triggers of sub modules ba ASQ)"]
    #[inline(always)]
    pub fn lpbe(&self) -> LpbeR {
        LpbeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits define the impedance of the buffers for RxBias and TxBias. While for resistive loads the lowest impedance shows the fastest settling; is this not the case for reactive loads."]
    #[inline(always)]
    pub fn bimp(&mut self) -> BimpW<'_, SaphAmcnfSpec> {
        BimpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Reserved for future use"]
    #[inline(always)]
    pub fn rsv0(&mut self) -> Rsv0W<'_, SaphAmcnfSpec> {
        Rsv0W::new(self, 2)
    }
    #[doc = "Bit 8 - This bit enables the charge pump of the input multiplexer."]
    #[inline(always)]
    pub fn cpeo(&mut self) -> CpeoW<'_, SaphAmcnfSpec> {
        CpeoW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Reserved for future use"]
    #[inline(always)]
    pub fn rsv1(&mut self) -> Rsv1W<'_, SaphAmcnfSpec> {
        Rsv1W::new(self, 9)
    }
    #[doc = "Bit 11 - LPBE, low power bias mode enable. This bit enables the low power bias operation mode. The selection of the operation mode shall only be changed while the PSQ is in OFF state (changes during other states of the PSQ causes corrupt measurement results and irregular triggers of sub modules ba ASQ)"]
    #[inline(always)]
    pub fn lpbe(&mut self) -> LpbeW<'_, SaphAmcnfSpec> {
        LpbeW::new(self, 11)
    }
}
#[doc = "Mode Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saph_amcnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saph_amcnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaphAmcnfSpec;
impl crate::RegisterSpec for SaphAmcnfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`saph_amcnf::R`](R) reader structure"]
impl crate::Readable for SaphAmcnfSpec {}
#[doc = "`write(|w| ..)` method takes [`saph_amcnf::W`](W) writer structure"]
impl crate::Writable for SaphAmcnfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAPH_AMCNF to value 0"]
impl crate::Resettable for SaphAmcnfSpec {}
