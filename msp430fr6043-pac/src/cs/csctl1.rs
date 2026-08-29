#[doc = "Register `CSCTL1` reader"]
pub type R = crate::R<Csctl1Spec>;
#[doc = "Register `CSCTL1` writer"]
pub type W = crate::W<Csctl1Spec>;
#[doc = "DCO frequency select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcofsel {
    #[doc = "0: If DCORSEL = 0: 1 MHz; If DCORSEL = 1: 1 MHz"]
    Dcofsel0 = 0,
    #[doc = "1: If DCORSEL = 0: 2.67 MHz; If DCORSEL = 1: 5.33 MHz"]
    Dcofsel1 = 1,
    #[doc = "2: If DCORSEL = 0: 3.33 MHz; If DCORSEL = 1: 6.67 MHz"]
    Dcofsel2 = 2,
    #[doc = "3: If DCORSEL = 0: 4 MHz; If DCORSEL = 1: 8 MHz"]
    Dcofsel3 = 3,
    #[doc = "4: If DCORSEL = 0: 5.33 MHz; If DCORSEL = 1: 16 MHz"]
    Dcofsel4 = 4,
    #[doc = "5: If DCORSEL = 0: 6.67 MHz; If DCORSEL = 1: 21 MHz"]
    Dcofsel5 = 5,
    #[doc = "6: If DCORSEL = 0: 8 MHz; If DCORSEL = 1: 24 MHz"]
    Dcofsel6 = 6,
    #[doc = "7: If DCORSEL = 0: Reserved. Defaults to 8. It is not recommended to use this setting; If DCORSEL = 1: Reserved. Defaults to 24. It is not recommended to use this setting"]
    Dcofsel7 = 7,
}
impl From<Dcofsel> for u8 {
    #[inline(always)]
    fn from(variant: Dcofsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcofsel {
    type Ux = u8;
}
impl crate::IsEnum for Dcofsel {}
#[doc = "Field `DCOFSEL` reader - DCO frequency select"]
pub type DcofselR = crate::FieldReader<Dcofsel>;
impl DcofselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcofsel {
        match self.bits {
            0 => Dcofsel::Dcofsel0,
            1 => Dcofsel::Dcofsel1,
            2 => Dcofsel::Dcofsel2,
            3 => Dcofsel::Dcofsel3,
            4 => Dcofsel::Dcofsel4,
            5 => Dcofsel::Dcofsel5,
            6 => Dcofsel::Dcofsel6,
            7 => Dcofsel::Dcofsel7,
            _ => unreachable!(),
        }
    }
    #[doc = "If DCORSEL = 0: 1 MHz; If DCORSEL = 1: 1 MHz"]
    #[inline(always)]
    pub fn is_dcofsel_0(&self) -> bool {
        *self == Dcofsel::Dcofsel0
    }
    #[doc = "If DCORSEL = 0: 2.67 MHz; If DCORSEL = 1: 5.33 MHz"]
    #[inline(always)]
    pub fn is_dcofsel_1(&self) -> bool {
        *self == Dcofsel::Dcofsel1
    }
    #[doc = "If DCORSEL = 0: 3.33 MHz; If DCORSEL = 1: 6.67 MHz"]
    #[inline(always)]
    pub fn is_dcofsel_2(&self) -> bool {
        *self == Dcofsel::Dcofsel2
    }
    #[doc = "If DCORSEL = 0: 4 MHz; If DCORSEL = 1: 8 MHz"]
    #[inline(always)]
    pub fn is_dcofsel_3(&self) -> bool {
        *self == Dcofsel::Dcofsel3
    }
    #[doc = "If DCORSEL = 0: 5.33 MHz; If DCORSEL = 1: 16 MHz"]
    #[inline(always)]
    pub fn is_dcofsel_4(&self) -> bool {
        *self == Dcofsel::Dcofsel4
    }
    #[doc = "If DCORSEL = 0: 6.67 MHz; If DCORSEL = 1: 21 MHz"]
    #[inline(always)]
    pub fn is_dcofsel_5(&self) -> bool {
        *self == Dcofsel::Dcofsel5
    }
    #[doc = "If DCORSEL = 0: 8 MHz; If DCORSEL = 1: 24 MHz"]
    #[inline(always)]
    pub fn is_dcofsel_6(&self) -> bool {
        *self == Dcofsel::Dcofsel6
    }
    #[doc = "If DCORSEL = 0: Reserved. Defaults to 8. It is not recommended to use this setting; If DCORSEL = 1: Reserved. Defaults to 24. It is not recommended to use this setting"]
    #[inline(always)]
    pub fn is_dcofsel_7(&self) -> bool {
        *self == Dcofsel::Dcofsel7
    }
}
#[doc = "Field `DCOFSEL` writer - DCO frequency select"]
pub type DcofselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcofsel, crate::Safe>;
impl<'a, REG> DcofselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If DCORSEL = 0: 1 MHz; If DCORSEL = 1: 1 MHz"]
    #[inline(always)]
    pub fn dcofsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel0)
    }
    #[doc = "If DCORSEL = 0: 2.67 MHz; If DCORSEL = 1: 5.33 MHz"]
    #[inline(always)]
    pub fn dcofsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel1)
    }
    #[doc = "If DCORSEL = 0: 3.33 MHz; If DCORSEL = 1: 6.67 MHz"]
    #[inline(always)]
    pub fn dcofsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel2)
    }
    #[doc = "If DCORSEL = 0: 4 MHz; If DCORSEL = 1: 8 MHz"]
    #[inline(always)]
    pub fn dcofsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel3)
    }
    #[doc = "If DCORSEL = 0: 5.33 MHz; If DCORSEL = 1: 16 MHz"]
    #[inline(always)]
    pub fn dcofsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel4)
    }
    #[doc = "If DCORSEL = 0: 6.67 MHz; If DCORSEL = 1: 21 MHz"]
    #[inline(always)]
    pub fn dcofsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel5)
    }
    #[doc = "If DCORSEL = 0: 8 MHz; If DCORSEL = 1: 24 MHz"]
    #[inline(always)]
    pub fn dcofsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel6)
    }
    #[doc = "If DCORSEL = 0: Reserved. Defaults to 8. It is not recommended to use this setting; If DCORSEL = 1: Reserved. Defaults to 24. It is not recommended to use this setting"]
    #[inline(always)]
    pub fn dcofsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dcofsel::Dcofsel7)
    }
}
#[doc = "Field `DCORSEL` reader - DCO range select"]
pub type DcorselR = crate::BitReader;
#[doc = "Field `DCORSEL` writer - DCO range select"]
pub type DcorselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:3 - DCO frequency select"]
    #[inline(always)]
    pub fn dcofsel(&self) -> DcofselR {
        DcofselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 6 - DCO range select"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DcorselR {
        DcorselR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - DCO frequency select"]
    #[inline(always)]
    pub fn dcofsel(&mut self) -> DcofselW<'_, Csctl1Spec> {
        DcofselW::new(self, 1)
    }
    #[doc = "Bit 6 - DCO range select"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DcorselW<'_, Csctl1Spec> {
        DcorselW::new(self, 6)
    }
}
#[doc = "Clock System Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl1Spec;
impl crate::RegisterSpec for Csctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl1::R`](R) reader structure"]
impl crate::Readable for Csctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl1::W`](W) writer structure"]
impl crate::Writable for Csctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL1 to value 0"]
impl crate::Resettable for Csctl1Spec {}
