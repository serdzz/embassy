#[doc = "Register `LCDVCTL` reader"]
pub type R = crate::R<LcdvctlSpec>;
#[doc = "Register `LCDVCTL` writer"]
pub type W = crate::W<LcdvctlSpec>;
#[doc = "Field `LCDREFMODE` reader - Selects wether R13 voltage is switched or in static mode"]
pub type LcdrefmodeR = crate::BitReader;
#[doc = "Field `LCDREFMODE` writer - Selects wether R13 voltage is switched or in static mode"]
pub type LcdrefmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDSELVDD` reader - selects if R33 is supplied either from Vcc internally or from charge pump"]
pub type LcdselvddR = crate::BitReader;
#[doc = "Field `LCDSELVDD` writer - selects if R33 is supplied either from Vcc internally or from charge pump"]
pub type LcdselvddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDREFEN` reader - Internal reference voltage enable on R13"]
pub type LcdrefenR = crate::BitReader;
#[doc = "Field `LCDREFEN` writer - Internal reference voltage enable on R13"]
pub type LcdrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCPEN` reader - Charge pump enable"]
pub type LcdcpenR = crate::BitReader;
#[doc = "Field `LCDCPEN` writer - Charge pump enable"]
pub type LcdcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VLCD select: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vlcd {
    #[doc = "0: VLCD = 2.60V"]
    Vlcd0 = 0,
    #[doc = "1: VLCD = 2.66V"]
    Vlcd1 = 1,
    #[doc = "2: VLCD = 2.72V"]
    Vlcd2 = 2,
    #[doc = "3: VLCD = 2.78V"]
    Vlcd3 = 3,
    #[doc = "4: VLCD = 2.84V"]
    Vlcd4 = 4,
    #[doc = "5: VLCD = 2.90V"]
    Vlcd5 = 5,
    #[doc = "6: VLCD = 2.96V"]
    Vlcd6 = 6,
    #[doc = "7: VLCD = 3.02V"]
    Vlcd7 = 7,
    #[doc = "8: VLCD = 3.08V"]
    Vlcd8 = 8,
    #[doc = "9: VLCD = 3.14V"]
    Vlcd9 = 9,
    #[doc = "10: VLCD = 3.20V"]
    Vlcd10 = 10,
    #[doc = "11: VLCD = 3.26V"]
    Vlcd11 = 11,
    #[doc = "12: VLCD = 3.32V"]
    Vlcd12 = 12,
    #[doc = "13: VLCD = 3.38V"]
    Vlcd13 = 13,
    #[doc = "14: VLCD = 3.44V"]
    Vlcd14 = 14,
    #[doc = "15: VLCD = 3.50V"]
    Vlcd15 = 15,
}
impl From<Vlcd> for u8 {
    #[inline(always)]
    fn from(variant: Vlcd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vlcd {
    type Ux = u8;
}
impl crate::IsEnum for Vlcd {}
#[doc = "Field `VLCD` reader - VLCD select: 0"]
pub type VlcdR = crate::FieldReader<Vlcd>;
impl VlcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlcd {
        match self.bits {
            0 => Vlcd::Vlcd0,
            1 => Vlcd::Vlcd1,
            2 => Vlcd::Vlcd2,
            3 => Vlcd::Vlcd3,
            4 => Vlcd::Vlcd4,
            5 => Vlcd::Vlcd5,
            6 => Vlcd::Vlcd6,
            7 => Vlcd::Vlcd7,
            8 => Vlcd::Vlcd8,
            9 => Vlcd::Vlcd9,
            10 => Vlcd::Vlcd10,
            11 => Vlcd::Vlcd11,
            12 => Vlcd::Vlcd12,
            13 => Vlcd::Vlcd13,
            14 => Vlcd::Vlcd14,
            15 => Vlcd::Vlcd15,
            _ => unreachable!(),
        }
    }
    #[doc = "VLCD = 2.60V"]
    #[inline(always)]
    pub fn is_vlcd_0(&self) -> bool {
        *self == Vlcd::Vlcd0
    }
    #[doc = "VLCD = 2.66V"]
    #[inline(always)]
    pub fn is_vlcd_1(&self) -> bool {
        *self == Vlcd::Vlcd1
    }
    #[doc = "VLCD = 2.72V"]
    #[inline(always)]
    pub fn is_vlcd_2(&self) -> bool {
        *self == Vlcd::Vlcd2
    }
    #[doc = "VLCD = 2.78V"]
    #[inline(always)]
    pub fn is_vlcd_3(&self) -> bool {
        *self == Vlcd::Vlcd3
    }
    #[doc = "VLCD = 2.84V"]
    #[inline(always)]
    pub fn is_vlcd_4(&self) -> bool {
        *self == Vlcd::Vlcd4
    }
    #[doc = "VLCD = 2.90V"]
    #[inline(always)]
    pub fn is_vlcd_5(&self) -> bool {
        *self == Vlcd::Vlcd5
    }
    #[doc = "VLCD = 2.96V"]
    #[inline(always)]
    pub fn is_vlcd_6(&self) -> bool {
        *self == Vlcd::Vlcd6
    }
    #[doc = "VLCD = 3.02V"]
    #[inline(always)]
    pub fn is_vlcd_7(&self) -> bool {
        *self == Vlcd::Vlcd7
    }
    #[doc = "VLCD = 3.08V"]
    #[inline(always)]
    pub fn is_vlcd_8(&self) -> bool {
        *self == Vlcd::Vlcd8
    }
    #[doc = "VLCD = 3.14V"]
    #[inline(always)]
    pub fn is_vlcd_9(&self) -> bool {
        *self == Vlcd::Vlcd9
    }
    #[doc = "VLCD = 3.20V"]
    #[inline(always)]
    pub fn is_vlcd_10(&self) -> bool {
        *self == Vlcd::Vlcd10
    }
    #[doc = "VLCD = 3.26V"]
    #[inline(always)]
    pub fn is_vlcd_11(&self) -> bool {
        *self == Vlcd::Vlcd11
    }
    #[doc = "VLCD = 3.32V"]
    #[inline(always)]
    pub fn is_vlcd_12(&self) -> bool {
        *self == Vlcd::Vlcd12
    }
    #[doc = "VLCD = 3.38V"]
    #[inline(always)]
    pub fn is_vlcd_13(&self) -> bool {
        *self == Vlcd::Vlcd13
    }
    #[doc = "VLCD = 3.44V"]
    #[inline(always)]
    pub fn is_vlcd_14(&self) -> bool {
        *self == Vlcd::Vlcd14
    }
    #[doc = "VLCD = 3.50V"]
    #[inline(always)]
    pub fn is_vlcd_15(&self) -> bool {
        *self == Vlcd::Vlcd15
    }
}
#[doc = "Field `VLCD` writer - VLCD select: 0"]
pub type VlcdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Vlcd, crate::Safe>;
impl<'a, REG> VlcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VLCD = 2.60V"]
    #[inline(always)]
    pub fn vlcd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd0)
    }
    #[doc = "VLCD = 2.66V"]
    #[inline(always)]
    pub fn vlcd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd1)
    }
    #[doc = "VLCD = 2.72V"]
    #[inline(always)]
    pub fn vlcd_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd2)
    }
    #[doc = "VLCD = 2.78V"]
    #[inline(always)]
    pub fn vlcd_3(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd3)
    }
    #[doc = "VLCD = 2.84V"]
    #[inline(always)]
    pub fn vlcd_4(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd4)
    }
    #[doc = "VLCD = 2.90V"]
    #[inline(always)]
    pub fn vlcd_5(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd5)
    }
    #[doc = "VLCD = 2.96V"]
    #[inline(always)]
    pub fn vlcd_6(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd6)
    }
    #[doc = "VLCD = 3.02V"]
    #[inline(always)]
    pub fn vlcd_7(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd7)
    }
    #[doc = "VLCD = 3.08V"]
    #[inline(always)]
    pub fn vlcd_8(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd8)
    }
    #[doc = "VLCD = 3.14V"]
    #[inline(always)]
    pub fn vlcd_9(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd9)
    }
    #[doc = "VLCD = 3.20V"]
    #[inline(always)]
    pub fn vlcd_10(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd10)
    }
    #[doc = "VLCD = 3.26V"]
    #[inline(always)]
    pub fn vlcd_11(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd11)
    }
    #[doc = "VLCD = 3.32V"]
    #[inline(always)]
    pub fn vlcd_12(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd12)
    }
    #[doc = "VLCD = 3.38V"]
    #[inline(always)]
    pub fn vlcd_13(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd13)
    }
    #[doc = "VLCD = 3.44V"]
    #[inline(always)]
    pub fn vlcd_14(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd14)
    }
    #[doc = "VLCD = 3.50V"]
    #[inline(always)]
    pub fn vlcd_15(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcd::Vlcd15)
    }
}
#[doc = "Field `LCDCPFSEL0` reader - Charge pump frequency selection Bit: 0"]
pub type Lcdcpfsel0R = crate::BitReader;
#[doc = "Field `LCDCPFSEL0` writer - Charge pump frequency selection Bit: 0"]
pub type Lcdcpfsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCPFSEL1` reader - Charge pump frequency selection Bit: 1"]
pub type Lcdcpfsel1R = crate::BitReader;
#[doc = "Field `LCDCPFSEL1` writer - Charge pump frequency selection Bit: 1"]
pub type Lcdcpfsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCPFSEL2` reader - Charge pump frequency selection Bit: 2"]
pub type Lcdcpfsel2R = crate::BitReader;
#[doc = "Field `LCDCPFSEL2` writer - Charge pump frequency selection Bit: 2"]
pub type Lcdcpfsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCPFSEL3` reader - Charge pump frequency selection Bit: 3"]
pub type Lcdcpfsel3R = crate::BitReader;
#[doc = "Field `LCDCPFSEL3` writer - Charge pump frequency selection Bit: 3"]
pub type Lcdcpfsel3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects wether R13 voltage is switched or in static mode"]
    #[inline(always)]
    pub fn lcdrefmode(&self) -> LcdrefmodeR {
        LcdrefmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - selects if R33 is supplied either from Vcc internally or from charge pump"]
    #[inline(always)]
    pub fn lcdselvdd(&self) -> LcdselvddR {
        LcdselvddR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Internal reference voltage enable on R13"]
    #[inline(always)]
    pub fn lcdrefen(&self) -> LcdrefenR {
        LcdrefenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Charge pump enable"]
    #[inline(always)]
    pub fn lcdcpen(&self) -> LcdcpenR {
        LcdcpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - VLCD select: 0"]
    #[inline(always)]
    pub fn vlcd(&self) -> VlcdR {
        VlcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Charge pump frequency selection Bit: 0"]
    #[inline(always)]
    pub fn lcdcpfsel0(&self) -> Lcdcpfsel0R {
        Lcdcpfsel0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Charge pump frequency selection Bit: 1"]
    #[inline(always)]
    pub fn lcdcpfsel1(&self) -> Lcdcpfsel1R {
        Lcdcpfsel1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Charge pump frequency selection Bit: 2"]
    #[inline(always)]
    pub fn lcdcpfsel2(&self) -> Lcdcpfsel2R {
        Lcdcpfsel2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Charge pump frequency selection Bit: 3"]
    #[inline(always)]
    pub fn lcdcpfsel3(&self) -> Lcdcpfsel3R {
        Lcdcpfsel3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects wether R13 voltage is switched or in static mode"]
    #[inline(always)]
    pub fn lcdrefmode(&mut self) -> LcdrefmodeW<'_, LcdvctlSpec> {
        LcdrefmodeW::new(self, 0)
    }
    #[doc = "Bit 5 - selects if R33 is supplied either from Vcc internally or from charge pump"]
    #[inline(always)]
    pub fn lcdselvdd(&mut self) -> LcdselvddW<'_, LcdvctlSpec> {
        LcdselvddW::new(self, 5)
    }
    #[doc = "Bit 6 - Internal reference voltage enable on R13"]
    #[inline(always)]
    pub fn lcdrefen(&mut self) -> LcdrefenW<'_, LcdvctlSpec> {
        LcdrefenW::new(self, 6)
    }
    #[doc = "Bit 7 - Charge pump enable"]
    #[inline(always)]
    pub fn lcdcpen(&mut self) -> LcdcpenW<'_, LcdvctlSpec> {
        LcdcpenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - VLCD select: 0"]
    #[inline(always)]
    pub fn vlcd(&mut self) -> VlcdW<'_, LcdvctlSpec> {
        VlcdW::new(self, 8)
    }
    #[doc = "Bit 12 - Charge pump frequency selection Bit: 0"]
    #[inline(always)]
    pub fn lcdcpfsel0(&mut self) -> Lcdcpfsel0W<'_, LcdvctlSpec> {
        Lcdcpfsel0W::new(self, 12)
    }
    #[doc = "Bit 13 - Charge pump frequency selection Bit: 1"]
    #[inline(always)]
    pub fn lcdcpfsel1(&mut self) -> Lcdcpfsel1W<'_, LcdvctlSpec> {
        Lcdcpfsel1W::new(self, 13)
    }
    #[doc = "Bit 14 - Charge pump frequency selection Bit: 2"]
    #[inline(always)]
    pub fn lcdcpfsel2(&mut self) -> Lcdcpfsel2W<'_, LcdvctlSpec> {
        Lcdcpfsel2W::new(self, 14)
    }
    #[doc = "Bit 15 - Charge pump frequency selection Bit: 3"]
    #[inline(always)]
    pub fn lcdcpfsel3(&mut self) -> Lcdcpfsel3W<'_, LcdvctlSpec> {
        Lcdcpfsel3W::new(self, 15)
    }
}
#[doc = "LCD_E Voltage Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdvctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdvctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdvctlSpec;
impl crate::RegisterSpec for LcdvctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lcdvctl::R`](R) reader structure"]
impl crate::Readable for LcdvctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdvctl::W`](W) writer structure"]
impl crate::Writable for LcdvctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDVCTL to value 0"]
impl crate::Resettable for LcdvctlSpec {}
