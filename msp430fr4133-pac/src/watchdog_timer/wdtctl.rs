#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WdtctlSpec>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WdtctlSpec>;
#[doc = "WDT - Timer Interval Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtis {
    #[doc = "0: WDT - Timer Interval Select: /2G"]
    Wdtis0 = 0,
    #[doc = "1: WDT - Timer Interval Select: /128M"]
    Wdtis1 = 1,
    #[doc = "2: WDT - Timer Interval Select: /8192k"]
    Wdtis2 = 2,
    #[doc = "3: WDT - Timer Interval Select: /512k"]
    Wdtis3 = 3,
    #[doc = "4: WDT - Timer Interval Select: /32k"]
    Wdtis4 = 4,
    #[doc = "5: WDT - Timer Interval Select: /8192"]
    Wdtis5 = 5,
    #[doc = "6: WDT - Timer Interval Select: /512"]
    Wdtis6 = 6,
    #[doc = "7: WDT - Timer Interval Select: /64"]
    Wdtis7 = 7,
}
impl From<Wdtis> for u8 {
    #[inline(always)]
    fn from(variant: Wdtis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtis {
    type Ux = u8;
}
impl crate::IsEnum for Wdtis {}
#[doc = "Field `WDTIS` reader - WDT - Timer Interval Select 0"]
pub type WdtisR = crate::FieldReader<Wdtis>;
impl WdtisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtis {
        match self.bits {
            0 => Wdtis::Wdtis0,
            1 => Wdtis::Wdtis1,
            2 => Wdtis::Wdtis2,
            3 => Wdtis::Wdtis3,
            4 => Wdtis::Wdtis4,
            5 => Wdtis::Wdtis5,
            6 => Wdtis::Wdtis6,
            7 => Wdtis::Wdtis7,
            _ => unreachable!(),
        }
    }
    #[doc = "WDT - Timer Interval Select: /2G"]
    #[inline(always)]
    pub fn is_wdtis_0(&self) -> bool {
        *self == Wdtis::Wdtis0
    }
    #[doc = "WDT - Timer Interval Select: /128M"]
    #[inline(always)]
    pub fn is_wdtis_1(&self) -> bool {
        *self == Wdtis::Wdtis1
    }
    #[doc = "WDT - Timer Interval Select: /8192k"]
    #[inline(always)]
    pub fn is_wdtis_2(&self) -> bool {
        *self == Wdtis::Wdtis2
    }
    #[doc = "WDT - Timer Interval Select: /512k"]
    #[inline(always)]
    pub fn is_wdtis_3(&self) -> bool {
        *self == Wdtis::Wdtis3
    }
    #[doc = "WDT - Timer Interval Select: /32k"]
    #[inline(always)]
    pub fn is_wdtis_4(&self) -> bool {
        *self == Wdtis::Wdtis4
    }
    #[doc = "WDT - Timer Interval Select: /8192"]
    #[inline(always)]
    pub fn is_wdtis_5(&self) -> bool {
        *self == Wdtis::Wdtis5
    }
    #[doc = "WDT - Timer Interval Select: /512"]
    #[inline(always)]
    pub fn is_wdtis_6(&self) -> bool {
        *self == Wdtis::Wdtis6
    }
    #[doc = "WDT - Timer Interval Select: /64"]
    #[inline(always)]
    pub fn is_wdtis_7(&self) -> bool {
        *self == Wdtis::Wdtis7
    }
}
#[doc = "Field `WDTIS` writer - WDT - Timer Interval Select 0"]
pub type WdtisW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdtis, crate::Safe>;
impl<'a, REG> WdtisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WDT - Timer Interval Select: /2G"]
    #[inline(always)]
    pub fn wdtis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis0)
    }
    #[doc = "WDT - Timer Interval Select: /128M"]
    #[inline(always)]
    pub fn wdtis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis1)
    }
    #[doc = "WDT - Timer Interval Select: /8192k"]
    #[inline(always)]
    pub fn wdtis_2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis2)
    }
    #[doc = "WDT - Timer Interval Select: /512k"]
    #[inline(always)]
    pub fn wdtis_3(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis3)
    }
    #[doc = "WDT - Timer Interval Select: /32k"]
    #[inline(always)]
    pub fn wdtis_4(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis4)
    }
    #[doc = "WDT - Timer Interval Select: /8192"]
    #[inline(always)]
    pub fn wdtis_5(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis5)
    }
    #[doc = "WDT - Timer Interval Select: /512"]
    #[inline(always)]
    pub fn wdtis_6(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis6)
    }
    #[doc = "WDT - Timer Interval Select: /64"]
    #[inline(always)]
    pub fn wdtis_7(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::Wdtis7)
    }
}
#[doc = "Field `WDTCNTCL` reader - WDT - Timer Clear"]
pub type WdtcntclR = crate::BitReader;
#[doc = "Field `WDTCNTCL` writer - WDT - Timer Clear"]
pub type WdtcntclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTTMSEL` reader - WDT - Timer Mode Select"]
pub type WdttmselR = crate::BitReader;
#[doc = "Field `WDTTMSEL` writer - WDT - Timer Mode Select"]
pub type WdttmselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "WDT - Timer Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtssel {
    #[doc = "0: WDT - Timer Clock Source Select: SMCLK"]
    Wdtssel0 = 0,
    #[doc = "1: WDT - Timer Clock Source Select: ACLK"]
    Wdtssel1 = 1,
    #[doc = "2: WDT - Timer Clock Source Select: VLO_CLK"]
    Wdtssel2 = 2,
    #[doc = "3: WDT - Timer Clock Source Select: reserved"]
    Wdtssel3 = 3,
}
impl From<Wdtssel> for u8 {
    #[inline(always)]
    fn from(variant: Wdtssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtssel {
    type Ux = u8;
}
impl crate::IsEnum for Wdtssel {}
#[doc = "Field `WDTSSEL` reader - WDT - Timer Clock Source Select 0"]
pub type WdtsselR = crate::FieldReader<Wdtssel>;
impl WdtsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtssel {
        match self.bits {
            0 => Wdtssel::Wdtssel0,
            1 => Wdtssel::Wdtssel1,
            2 => Wdtssel::Wdtssel2,
            3 => Wdtssel::Wdtssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn is_wdtssel_0(&self) -> bool {
        *self == Wdtssel::Wdtssel0
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn is_wdtssel_1(&self) -> bool {
        *self == Wdtssel::Wdtssel1
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn is_wdtssel_2(&self) -> bool {
        *self == Wdtssel::Wdtssel2
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn is_wdtssel_3(&self) -> bool {
        *self == Wdtssel::Wdtssel3
    }
}
#[doc = "Field `WDTSSEL` writer - WDT - Timer Clock Source Select 0"]
pub type WdtsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wdtssel, crate::Safe>;
impl<'a, REG> WdtsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn wdtssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel0)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn wdtssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel1)
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn wdtssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel2)
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn wdtssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Wdtssel3)
    }
}
#[doc = "Field `WDTHOLD` reader - WDT - Timer hold"]
pub type WdtholdR = crate::BitReader;
#[doc = "Field `WDTHOLD` writer - WDT - Timer hold"]
pub type WdtholdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WdtisR {
        WdtisR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WdtcntclR {
        WdtcntclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WdttmselR {
        WdttmselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WdtsselR {
        WdtsselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WdtholdR {
        WdtholdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WdtisW<'_, WdtctlSpec> {
        WdtisW::new(self, 0)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WdtcntclW<'_, WdtctlSpec> {
        WdtcntclW::new(self, 3)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WdttmselW<'_, WdtctlSpec> {
        WdttmselW::new(self, 4)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WdtsselW<'_, WdtctlSpec> {
        WdtsselW::new(self, 5)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WdtholdW<'_, WdtctlSpec> {
        WdtholdW::new(self, 7)
    }
}
#[doc = "Watchdog Timer Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtctlSpec;
impl crate::RegisterSpec for WdtctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtctl::R`](R) reader structure"]
impl crate::Readable for WdtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtctl::W`](W) writer structure"]
impl crate::Writable for WdtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WdtctlSpec {}
