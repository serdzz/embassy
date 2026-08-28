#[doc = "Register `DMACTL0` reader"]
pub type R = crate::R<Dmactl0Spec>;
#[doc = "Register `DMACTL0` writer"]
pub type W = crate::W<Dmactl0Spec>;
#[doc = "DMA channel 0 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0tsel {
    #[doc = "0: DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    Dma0tsel0 = 0,
    #[doc = "1: DMA channel 0 transfer select 1: Timer_A (TACCR2.IFG)"]
    Dma0tsel1 = 1,
    #[doc = "2: DMA channel 0 transfer select 2: Timer_B (TBCCR2.IFG)"]
    Dma0tsel2 = 2,
    #[doc = "3: DMA channel 0 transfer select 3: USCIA0 receive"]
    Dma0tsel3 = 3,
    #[doc = "4: DMA channel 0 transfer select 4: USCIA0 transmit"]
    Dma0tsel4 = 4,
    #[doc = "5: DMA channel 0 transfer select 5: DAC12_0CTL.DAC12IFG"]
    Dma0tsel5 = 5,
    #[doc = "6: DMA channel 0 transfer select 6: ADC12 (ADC12IFG)"]
    Dma0tsel6 = 6,
    #[doc = "7: DMA channel 0 transfer select 7: Timer_A (TACCR0.IFG)"]
    Dma0tsel7 = 7,
    #[doc = "8: DMA channel 0 transfer select 8: Timer_B (TBCCR0.IFG)"]
    Dma0tsel8 = 8,
    #[doc = "9: DMA channel 0 transfer select 9: USCIA1 receive"]
    Dma0tsel9 = 9,
    #[doc = "10: DMA channel 0 transfer select 10: USCIA1 transmit"]
    Dma0tsel10 = 10,
    #[doc = "11: DMA channel 0 transfer select 11: Multiplier ready"]
    Dma0tsel11 = 11,
    #[doc = "12: DMA channel 0 transfer select 12: USCIB0 receive"]
    Dma0tsel12 = 12,
    #[doc = "13: DMA channel 0 transfer select 13: USCIB0 transmit"]
    Dma0tsel13 = 13,
    #[doc = "14: DMA channel 0 transfer select 14: previous DMA channel DMA2IFG"]
    Dma0tsel14 = 14,
    #[doc = "15: DMA channel 0 transfer select 15: ext. Trigger (DMAE0)"]
    Dma0tsel15 = 15,
}
impl From<Dma0tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma0tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma0tsel {}
#[doc = "Field `DMA0TSEL` reader - DMA channel 0 transfer select bit 0"]
pub type Dma0tselR = crate::FieldReader<Dma0tsel>;
impl Dma0tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0tsel {
        match self.bits {
            0 => Dma0tsel::Dma0tsel0,
            1 => Dma0tsel::Dma0tsel1,
            2 => Dma0tsel::Dma0tsel2,
            3 => Dma0tsel::Dma0tsel3,
            4 => Dma0tsel::Dma0tsel4,
            5 => Dma0tsel::Dma0tsel5,
            6 => Dma0tsel::Dma0tsel6,
            7 => Dma0tsel::Dma0tsel7,
            8 => Dma0tsel::Dma0tsel8,
            9 => Dma0tsel::Dma0tsel9,
            10 => Dma0tsel::Dma0tsel10,
            11 => Dma0tsel::Dma0tsel11,
            12 => Dma0tsel::Dma0tsel12,
            13 => Dma0tsel::Dma0tsel13,
            14 => Dma0tsel::Dma0tsel14,
            15 => Dma0tsel::Dma0tsel15,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn is_dma0tsel_0(&self) -> bool {
        *self == Dma0tsel::Dma0tsel0
    }
    #[doc = "DMA channel 0 transfer select 1: Timer_A (TACCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_1(&self) -> bool {
        *self == Dma0tsel::Dma0tsel1
    }
    #[doc = "DMA channel 0 transfer select 2: Timer_B (TBCCR2.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_2(&self) -> bool {
        *self == Dma0tsel::Dma0tsel2
    }
    #[doc = "DMA channel 0 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn is_dma0tsel_3(&self) -> bool {
        *self == Dma0tsel::Dma0tsel3
    }
    #[doc = "DMA channel 0 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn is_dma0tsel_4(&self) -> bool {
        *self == Dma0tsel::Dma0tsel4
    }
    #[doc = "DMA channel 0 transfer select 5: DAC12_0CTL.DAC12IFG"]
    #[inline(always)]
    pub fn is_dma0tsel_5(&self) -> bool {
        *self == Dma0tsel::Dma0tsel5
    }
    #[doc = "DMA channel 0 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_6(&self) -> bool {
        *self == Dma0tsel::Dma0tsel6
    }
    #[doc = "DMA channel 0 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_7(&self) -> bool {
        *self == Dma0tsel::Dma0tsel7
    }
    #[doc = "DMA channel 0 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma0tsel_8(&self) -> bool {
        *self == Dma0tsel::Dma0tsel8
    }
    #[doc = "DMA channel 0 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn is_dma0tsel_9(&self) -> bool {
        *self == Dma0tsel::Dma0tsel9
    }
    #[doc = "DMA channel 0 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn is_dma0tsel_10(&self) -> bool {
        *self == Dma0tsel::Dma0tsel10
    }
    #[doc = "DMA channel 0 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn is_dma0tsel_11(&self) -> bool {
        *self == Dma0tsel::Dma0tsel11
    }
    #[doc = "DMA channel 0 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn is_dma0tsel_12(&self) -> bool {
        *self == Dma0tsel::Dma0tsel12
    }
    #[doc = "DMA channel 0 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn is_dma0tsel_13(&self) -> bool {
        *self == Dma0tsel::Dma0tsel13
    }
    #[doc = "DMA channel 0 transfer select 14: previous DMA channel DMA2IFG"]
    #[inline(always)]
    pub fn is_dma0tsel_14(&self) -> bool {
        *self == Dma0tsel::Dma0tsel14
    }
    #[doc = "DMA channel 0 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn is_dma0tsel_15(&self) -> bool {
        *self == Dma0tsel::Dma0tsel15
    }
}
#[doc = "Field `DMA0TSEL` writer - DMA channel 0 transfer select bit 0"]
pub type Dma0tselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dma0tsel, crate::Safe>;
impl<'a, REG> Dma0tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA channel 0 transfer select 0: DMA_REQ (sw)"]
    #[inline(always)]
    pub fn dma0tsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel0)
    }
    #[doc = "DMA channel 0 transfer select 1: Timer_A (TACCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel1)
    }
    #[doc = "DMA channel 0 transfer select 2: Timer_B (TBCCR2.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel2)
    }
    #[doc = "DMA channel 0 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn dma0tsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel3)
    }
    #[doc = "DMA channel 0 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma0tsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel4)
    }
    #[doc = "DMA channel 0 transfer select 5: DAC12_0CTL.DAC12IFG"]
    #[inline(always)]
    pub fn dma0tsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel5)
    }
    #[doc = "DMA channel 0 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn dma0tsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel6)
    }
    #[doc = "DMA channel 0 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel7)
    }
    #[doc = "DMA channel 0 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn dma0tsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel8)
    }
    #[doc = "DMA channel 0 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn dma0tsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel9)
    }
    #[doc = "DMA channel 0 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma0tsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel10)
    }
    #[doc = "DMA channel 0 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn dma0tsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel11)
    }
    #[doc = "DMA channel 0 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn dma0tsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel12)
    }
    #[doc = "DMA channel 0 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma0tsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel13)
    }
    #[doc = "DMA channel 0 transfer select 14: previous DMA channel DMA2IFG"]
    #[inline(always)]
    pub fn dma0tsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel14)
    }
    #[doc = "DMA channel 0 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma0tsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0tsel::Dma0tsel15)
    }
}
#[doc = "DMA channel 1 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1tsel {
    #[doc = "0: DMA channel 1 transfer select 0: DMA_REQ"]
    Dma1tsel0 = 0,
    #[doc = "1: DMA channel 1 transfer select 1: Timer_A CCRIFG.2"]
    Dma1tsel1 = 1,
    #[doc = "2: DMA channel 1 transfer select 2: Timer_B CCRIFG.2"]
    Dma1tsel2 = 2,
    #[doc = "3: DMA channel 1 transfer select 3: USCIA0 receive"]
    Dma1tsel3 = 3,
    #[doc = "4: DMA channel 1 transfer select 4: USCIA0 transmit"]
    Dma1tsel4 = 4,
    #[doc = "5: DMA channel 1 transfer select 5: DAC12.0IFG"]
    Dma1tsel5 = 5,
    #[doc = "6: DMA channel 1 transfer select 6: ADC12 (ADC12IFG)"]
    Dma1tsel6 = 6,
    #[doc = "7: DMA channel 1 transfer select 7: Timer_A (TACCR0.IFG)"]
    Dma1tsel7 = 7,
    #[doc = "8: DMA channel 1 transfer select 8: Timer_B (TBCCR0.IFG)"]
    Dma1tsel8 = 8,
    #[doc = "9: DMA channel 1 transfer select 9: USCIA1 receive"]
    Dma1tsel9 = 9,
    #[doc = "10: DMA channel 1 transfer select 10: USCIA1 transmit"]
    Dma1tsel10 = 10,
    #[doc = "11: DMA channel 1 transfer select 11: Multiplier ready"]
    Dma1tsel11 = 11,
    #[doc = "12: DMA channel 1 transfer select 12: USCIB0 receive"]
    Dma1tsel12 = 12,
    #[doc = "13: DMA channel 1 transfer select 13: USCIB0 transmit"]
    Dma1tsel13 = 13,
    #[doc = "14: DMA channel 1 transfer select 14: previous DMA channel DMA0IFG"]
    Dma1tsel14 = 14,
    #[doc = "15: DMA channel 1 transfer select 15: ext. Trigger (DMAE0)"]
    Dma1tsel15 = 15,
}
impl From<Dma1tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma1tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma1tsel {}
#[doc = "Field `DMA1TSEL` reader - DMA channel 1 transfer select bit 0"]
pub type Dma1tselR = crate::FieldReader<Dma1tsel>;
impl Dma1tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1tsel {
        match self.bits {
            0 => Dma1tsel::Dma1tsel0,
            1 => Dma1tsel::Dma1tsel1,
            2 => Dma1tsel::Dma1tsel2,
            3 => Dma1tsel::Dma1tsel3,
            4 => Dma1tsel::Dma1tsel4,
            5 => Dma1tsel::Dma1tsel5,
            6 => Dma1tsel::Dma1tsel6,
            7 => Dma1tsel::Dma1tsel7,
            8 => Dma1tsel::Dma1tsel8,
            9 => Dma1tsel::Dma1tsel9,
            10 => Dma1tsel::Dma1tsel10,
            11 => Dma1tsel::Dma1tsel11,
            12 => Dma1tsel::Dma1tsel12,
            13 => Dma1tsel::Dma1tsel13,
            14 => Dma1tsel::Dma1tsel14,
            15 => Dma1tsel::Dma1tsel15,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA channel 1 transfer select 0: DMA_REQ"]
    #[inline(always)]
    pub fn is_dma1tsel_0(&self) -> bool {
        *self == Dma1tsel::Dma1tsel0
    }
    #[doc = "DMA channel 1 transfer select 1: Timer_A CCRIFG.2"]
    #[inline(always)]
    pub fn is_dma1tsel_1(&self) -> bool {
        *self == Dma1tsel::Dma1tsel1
    }
    #[doc = "DMA channel 1 transfer select 2: Timer_B CCRIFG.2"]
    #[inline(always)]
    pub fn is_dma1tsel_2(&self) -> bool {
        *self == Dma1tsel::Dma1tsel2
    }
    #[doc = "DMA channel 1 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn is_dma1tsel_3(&self) -> bool {
        *self == Dma1tsel::Dma1tsel3
    }
    #[doc = "DMA channel 1 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn is_dma1tsel_4(&self) -> bool {
        *self == Dma1tsel::Dma1tsel4
    }
    #[doc = "DMA channel 1 transfer select 5: DAC12.0IFG"]
    #[inline(always)]
    pub fn is_dma1tsel_5(&self) -> bool {
        *self == Dma1tsel::Dma1tsel5
    }
    #[doc = "DMA channel 1 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_6(&self) -> bool {
        *self == Dma1tsel::Dma1tsel6
    }
    #[doc = "DMA channel 1 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_7(&self) -> bool {
        *self == Dma1tsel::Dma1tsel7
    }
    #[doc = "DMA channel 1 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma1tsel_8(&self) -> bool {
        *self == Dma1tsel::Dma1tsel8
    }
    #[doc = "DMA channel 1 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn is_dma1tsel_9(&self) -> bool {
        *self == Dma1tsel::Dma1tsel9
    }
    #[doc = "DMA channel 1 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn is_dma1tsel_10(&self) -> bool {
        *self == Dma1tsel::Dma1tsel10
    }
    #[doc = "DMA channel 1 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn is_dma1tsel_11(&self) -> bool {
        *self == Dma1tsel::Dma1tsel11
    }
    #[doc = "DMA channel 1 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn is_dma1tsel_12(&self) -> bool {
        *self == Dma1tsel::Dma1tsel12
    }
    #[doc = "DMA channel 1 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn is_dma1tsel_13(&self) -> bool {
        *self == Dma1tsel::Dma1tsel13
    }
    #[doc = "DMA channel 1 transfer select 14: previous DMA channel DMA0IFG"]
    #[inline(always)]
    pub fn is_dma1tsel_14(&self) -> bool {
        *self == Dma1tsel::Dma1tsel14
    }
    #[doc = "DMA channel 1 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn is_dma1tsel_15(&self) -> bool {
        *self == Dma1tsel::Dma1tsel15
    }
}
#[doc = "Field `DMA1TSEL` writer - DMA channel 1 transfer select bit 0"]
pub type Dma1tselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dma1tsel, crate::Safe>;
impl<'a, REG> Dma1tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA channel 1 transfer select 0: DMA_REQ"]
    #[inline(always)]
    pub fn dma1tsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel0)
    }
    #[doc = "DMA channel 1 transfer select 1: Timer_A CCRIFG.2"]
    #[inline(always)]
    pub fn dma1tsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel1)
    }
    #[doc = "DMA channel 1 transfer select 2: Timer_B CCRIFG.2"]
    #[inline(always)]
    pub fn dma1tsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel2)
    }
    #[doc = "DMA channel 1 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn dma1tsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel3)
    }
    #[doc = "DMA channel 1 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma1tsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel4)
    }
    #[doc = "DMA channel 1 transfer select 5: DAC12.0IFG"]
    #[inline(always)]
    pub fn dma1tsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel5)
    }
    #[doc = "DMA channel 1 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn dma1tsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel6)
    }
    #[doc = "DMA channel 1 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel7)
    }
    #[doc = "DMA channel 1 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn dma1tsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel8)
    }
    #[doc = "DMA channel 1 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn dma1tsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel9)
    }
    #[doc = "DMA channel 1 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma1tsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel10)
    }
    #[doc = "DMA channel 1 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn dma1tsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel11)
    }
    #[doc = "DMA channel 1 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn dma1tsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel12)
    }
    #[doc = "DMA channel 1 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma1tsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel13)
    }
    #[doc = "DMA channel 1 transfer select 14: previous DMA channel DMA0IFG"]
    #[inline(always)]
    pub fn dma1tsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel14)
    }
    #[doc = "DMA channel 1 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma1tsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1tsel::Dma1tsel15)
    }
}
#[doc = "DMA channel 2 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma2tsel {
    #[doc = "0: DMA channel 2 transfer select 0: DMA_REQ"]
    Dma2tsel0 = 0,
    #[doc = "1: DMA channel 2 transfer select 1: Timer_A CCRIFG.2"]
    Dma2tsel1 = 1,
    #[doc = "2: DMA channel 2 transfer select 2: Timer_B CCRIFG.2"]
    Dma2tsel2 = 2,
    #[doc = "3: DMA channel 2 transfer select 3: USCIA0 receive"]
    Dma2tsel3 = 3,
    #[doc = "4: DMA channel 2 transfer select 4: USCIA0 transmit"]
    Dma2tsel4 = 4,
    #[doc = "5: DMA channel 2 transfer select 5: DAC12.0IFG"]
    Dma2tsel5 = 5,
    #[doc = "6: DMA channel 2 transfer select 6: ADC12 (ADC12IFG)"]
    Dma2tsel6 = 6,
    #[doc = "7: DMA channel 2 transfer select 7: Timer_A (TACCR0.IFG)"]
    Dma2tsel7 = 7,
    #[doc = "8: DMA channel 2 transfer select 8: Timer_B (TBCCR0.IFG)"]
    Dma2tsel8 = 8,
    #[doc = "9: DMA channel 2 transfer select 9: USCIA1 receive"]
    Dma2tsel9 = 9,
    #[doc = "10: DMA channel 2 transfer select 10: USCIA1 transmit"]
    Dma2tsel10 = 10,
    #[doc = "11: DMA channel 2 transfer select 11: Multiplier ready"]
    Dma2tsel11 = 11,
    #[doc = "12: DMA channel 2 transfer select 12: USCIB0 receive"]
    Dma2tsel12 = 12,
    #[doc = "13: DMA channel 2 transfer select 13: USCIB0 transmit"]
    Dma2tsel13 = 13,
    #[doc = "14: DMA channel 2 transfer select 14: previous DMA channel DMA1IFG"]
    Dma2tsel14 = 14,
    #[doc = "15: DMA channel 2 transfer select 15: ext. Trigger (DMAE0)"]
    Dma2tsel15 = 15,
}
impl From<Dma2tsel> for u8 {
    #[inline(always)]
    fn from(variant: Dma2tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma2tsel {
    type Ux = u8;
}
impl crate::IsEnum for Dma2tsel {}
#[doc = "Field `DMA2TSEL` reader - DMA channel 2 transfer select bit 0"]
pub type Dma2tselR = crate::FieldReader<Dma2tsel>;
impl Dma2tselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2tsel {
        match self.bits {
            0 => Dma2tsel::Dma2tsel0,
            1 => Dma2tsel::Dma2tsel1,
            2 => Dma2tsel::Dma2tsel2,
            3 => Dma2tsel::Dma2tsel3,
            4 => Dma2tsel::Dma2tsel4,
            5 => Dma2tsel::Dma2tsel5,
            6 => Dma2tsel::Dma2tsel6,
            7 => Dma2tsel::Dma2tsel7,
            8 => Dma2tsel::Dma2tsel8,
            9 => Dma2tsel::Dma2tsel9,
            10 => Dma2tsel::Dma2tsel10,
            11 => Dma2tsel::Dma2tsel11,
            12 => Dma2tsel::Dma2tsel12,
            13 => Dma2tsel::Dma2tsel13,
            14 => Dma2tsel::Dma2tsel14,
            15 => Dma2tsel::Dma2tsel15,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA channel 2 transfer select 0: DMA_REQ"]
    #[inline(always)]
    pub fn is_dma2tsel_0(&self) -> bool {
        *self == Dma2tsel::Dma2tsel0
    }
    #[doc = "DMA channel 2 transfer select 1: Timer_A CCRIFG.2"]
    #[inline(always)]
    pub fn is_dma2tsel_1(&self) -> bool {
        *self == Dma2tsel::Dma2tsel1
    }
    #[doc = "DMA channel 2 transfer select 2: Timer_B CCRIFG.2"]
    #[inline(always)]
    pub fn is_dma2tsel_2(&self) -> bool {
        *self == Dma2tsel::Dma2tsel2
    }
    #[doc = "DMA channel 2 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn is_dma2tsel_3(&self) -> bool {
        *self == Dma2tsel::Dma2tsel3
    }
    #[doc = "DMA channel 2 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn is_dma2tsel_4(&self) -> bool {
        *self == Dma2tsel::Dma2tsel4
    }
    #[doc = "DMA channel 2 transfer select 5: DAC12.0IFG"]
    #[inline(always)]
    pub fn is_dma2tsel_5(&self) -> bool {
        *self == Dma2tsel::Dma2tsel5
    }
    #[doc = "DMA channel 2 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn is_dma2tsel_6(&self) -> bool {
        *self == Dma2tsel::Dma2tsel6
    }
    #[doc = "DMA channel 2 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma2tsel_7(&self) -> bool {
        *self == Dma2tsel::Dma2tsel7
    }
    #[doc = "DMA channel 2 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn is_dma2tsel_8(&self) -> bool {
        *self == Dma2tsel::Dma2tsel8
    }
    #[doc = "DMA channel 2 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn is_dma2tsel_9(&self) -> bool {
        *self == Dma2tsel::Dma2tsel9
    }
    #[doc = "DMA channel 2 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn is_dma2tsel_10(&self) -> bool {
        *self == Dma2tsel::Dma2tsel10
    }
    #[doc = "DMA channel 2 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn is_dma2tsel_11(&self) -> bool {
        *self == Dma2tsel::Dma2tsel11
    }
    #[doc = "DMA channel 2 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn is_dma2tsel_12(&self) -> bool {
        *self == Dma2tsel::Dma2tsel12
    }
    #[doc = "DMA channel 2 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn is_dma2tsel_13(&self) -> bool {
        *self == Dma2tsel::Dma2tsel13
    }
    #[doc = "DMA channel 2 transfer select 14: previous DMA channel DMA1IFG"]
    #[inline(always)]
    pub fn is_dma2tsel_14(&self) -> bool {
        *self == Dma2tsel::Dma2tsel14
    }
    #[doc = "DMA channel 2 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn is_dma2tsel_15(&self) -> bool {
        *self == Dma2tsel::Dma2tsel15
    }
}
#[doc = "Field `DMA2TSEL` writer - DMA channel 2 transfer select bit 0"]
pub type Dma2tselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dma2tsel, crate::Safe>;
impl<'a, REG> Dma2tselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA channel 2 transfer select 0: DMA_REQ"]
    #[inline(always)]
    pub fn dma2tsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel0)
    }
    #[doc = "DMA channel 2 transfer select 1: Timer_A CCRIFG.2"]
    #[inline(always)]
    pub fn dma2tsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel1)
    }
    #[doc = "DMA channel 2 transfer select 2: Timer_B CCRIFG.2"]
    #[inline(always)]
    pub fn dma2tsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel2)
    }
    #[doc = "DMA channel 2 transfer select 3: USCIA0 receive"]
    #[inline(always)]
    pub fn dma2tsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel3)
    }
    #[doc = "DMA channel 2 transfer select 4: USCIA0 transmit"]
    #[inline(always)]
    pub fn dma2tsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel4)
    }
    #[doc = "DMA channel 2 transfer select 5: DAC12.0IFG"]
    #[inline(always)]
    pub fn dma2tsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel5)
    }
    #[doc = "DMA channel 2 transfer select 6: ADC12 (ADC12IFG)"]
    #[inline(always)]
    pub fn dma2tsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel6)
    }
    #[doc = "DMA channel 2 transfer select 7: Timer_A (TACCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel7)
    }
    #[doc = "DMA channel 2 transfer select 8: Timer_B (TBCCR0.IFG)"]
    #[inline(always)]
    pub fn dma2tsel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel8)
    }
    #[doc = "DMA channel 2 transfer select 9: USCIA1 receive"]
    #[inline(always)]
    pub fn dma2tsel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel9)
    }
    #[doc = "DMA channel 2 transfer select 10: USCIA1 transmit"]
    #[inline(always)]
    pub fn dma2tsel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel10)
    }
    #[doc = "DMA channel 2 transfer select 11: Multiplier ready"]
    #[inline(always)]
    pub fn dma2tsel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel11)
    }
    #[doc = "DMA channel 2 transfer select 12: USCIB0 receive"]
    #[inline(always)]
    pub fn dma2tsel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel12)
    }
    #[doc = "DMA channel 2 transfer select 13: USCIB0 transmit"]
    #[inline(always)]
    pub fn dma2tsel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel13)
    }
    #[doc = "DMA channel 2 transfer select 14: previous DMA channel DMA1IFG"]
    #[inline(always)]
    pub fn dma2tsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel14)
    }
    #[doc = "DMA channel 2 transfer select 15: ext. Trigger (DMAE0)"]
    #[inline(always)]
    pub fn dma2tsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2tsel::Dma2tsel15)
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    pub fn dma0tsel(&self) -> Dma0tselR {
        Dma0tselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    pub fn dma1tsel(&self) -> Dma1tselR {
        Dma1tselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA channel 2 transfer select bit 0"]
    #[inline(always)]
    pub fn dma2tsel(&self) -> Dma2tselR {
        Dma2tselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    pub fn dma0tsel(&mut self) -> Dma0tselW<'_, Dmactl0Spec> {
        Dma0tselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    pub fn dma1tsel(&mut self) -> Dma1tselW<'_, Dmactl0Spec> {
        Dma1tselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - DMA channel 2 transfer select bit 0"]
    #[inline(always)]
    pub fn dma2tsel(&mut self) -> Dma2tselW<'_, Dmactl0Spec> {
        Dma2tselW::new(self, 8)
    }
}
#[doc = "DMA Module Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmactl0Spec;
impl crate::RegisterSpec for Dmactl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl0::R`](R) reader structure"]
impl crate::Readable for Dmactl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmactl0::W`](W) writer structure"]
impl crate::Writable for Dmactl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTL0 to value 0"]
impl crate::Resettable for Dmactl0Spec {}
