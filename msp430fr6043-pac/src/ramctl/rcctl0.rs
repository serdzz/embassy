#[doc = "Register `RCCTL0` reader"]
pub type R = crate::R<Rcctl0Spec>;
#[doc = "Register `RCCTL0` writer"]
pub type W = crate::W<Rcctl0Spec>;
#[doc = "RAM controller RAM sector 0 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcrs0off {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    Rcrs0off0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off2 = 2,
}
impl From<Rcrs0off> for u8 {
    #[inline(always)]
    fn from(variant: Rcrs0off) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcrs0off {
    type Ux = u8;
}
impl crate::IsEnum for Rcrs0off {}
#[doc = "Field `RCRS0OFF` reader - RAM controller RAM sector 0 off"]
pub type Rcrs0offR = crate::FieldReader<Rcrs0off>;
impl Rcrs0offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rcrs0off> {
        match self.bits {
            0 => Some(Rcrs0off::Rcrs0off0),
            1 => Some(Rcrs0off::Rcrs0off1),
            2 => Some(Rcrs0off::Rcrs0off2),
            _ => None,
        }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == Rcrs0off::Rcrs0off0
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == Rcrs0off::Rcrs0off1
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == Rcrs0off::Rcrs0off2
    }
}
#[doc = "Field `RCRS0OFF` writer - RAM controller RAM sector 0 off"]
pub type Rcrs0offW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rcrs0off>;
impl<'a, REG> Rcrs0offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs0off::Rcrs0off0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs0off::Rcrs0off1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs0off::Rcrs0off2)
    }
}
#[doc = "RAM controller RAM sector 1 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcrs1off {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    Rcrs0off0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off2 = 2,
}
impl From<Rcrs1off> for u8 {
    #[inline(always)]
    fn from(variant: Rcrs1off) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcrs1off {
    type Ux = u8;
}
impl crate::IsEnum for Rcrs1off {}
#[doc = "Field `RCRS1OFF` reader - RAM controller RAM sector 1 off"]
pub type Rcrs1offR = crate::FieldReader<Rcrs1off>;
impl Rcrs1offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rcrs1off> {
        match self.bits {
            0 => Some(Rcrs1off::Rcrs0off0),
            1 => Some(Rcrs1off::Rcrs0off1),
            2 => Some(Rcrs1off::Rcrs0off2),
            _ => None,
        }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == Rcrs1off::Rcrs0off0
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == Rcrs1off::Rcrs0off1
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == Rcrs1off::Rcrs0off2
    }
}
#[doc = "Field `RCRS1OFF` writer - RAM controller RAM sector 1 off"]
pub type Rcrs1offW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rcrs1off>;
impl<'a, REG> Rcrs1offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs1off::Rcrs0off0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs1off::Rcrs0off1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs1off::Rcrs0off2)
    }
}
#[doc = "RAM controller RAM sector 2 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcrs2off {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    Rcrs0off0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off2 = 2,
}
impl From<Rcrs2off> for u8 {
    #[inline(always)]
    fn from(variant: Rcrs2off) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcrs2off {
    type Ux = u8;
}
impl crate::IsEnum for Rcrs2off {}
#[doc = "Field `RCRS2OFF` reader - RAM controller RAM sector 2 off"]
pub type Rcrs2offR = crate::FieldReader<Rcrs2off>;
impl Rcrs2offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rcrs2off> {
        match self.bits {
            0 => Some(Rcrs2off::Rcrs0off0),
            1 => Some(Rcrs2off::Rcrs0off1),
            2 => Some(Rcrs2off::Rcrs0off2),
            _ => None,
        }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == Rcrs2off::Rcrs0off0
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == Rcrs2off::Rcrs0off1
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == Rcrs2off::Rcrs0off2
    }
}
#[doc = "Field `RCRS2OFF` writer - RAM controller RAM sector 2 off"]
pub type Rcrs2offW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rcrs2off>;
impl<'a, REG> Rcrs2offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs2off::Rcrs0off0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs2off::Rcrs0off1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs2off::Rcrs0off2)
    }
}
#[doc = "RAM controller RAM sector 3 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcrs3off {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    Rcrs0off0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    Rcrs0off2 = 2,
}
impl From<Rcrs3off> for u8 {
    #[inline(always)]
    fn from(variant: Rcrs3off) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcrs3off {
    type Ux = u8;
}
impl crate::IsEnum for Rcrs3off {}
#[doc = "Field `RCRS3OFF` reader - RAM controller RAM sector 3 off"]
pub type Rcrs3offR = crate::FieldReader<Rcrs3off>;
impl Rcrs3offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rcrs3off> {
        match self.bits {
            0 => Some(Rcrs3off::Rcrs0off0),
            1 => Some(Rcrs3off::Rcrs0off1),
            2 => Some(Rcrs3off::Rcrs0off2),
            _ => None,
        }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == Rcrs3off::Rcrs0off0
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == Rcrs3off::Rcrs0off1
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == Rcrs3off::Rcrs0off2
    }
}
#[doc = "Field `RCRS3OFF` writer - RAM controller RAM sector 3 off"]
pub type Rcrs3offW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rcrs3off>;
impl<'a, REG> Rcrs3offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs3off::Rcrs0off0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs3off::Rcrs0off1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rcrs3off::Rcrs0off2)
    }
}
#[doc = "Field `RCKEY` reader - RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored."]
pub type RckeyR = crate::FieldReader;
#[doc = "Field `RCKEY` writer - RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored."]
pub type RckeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs0off(&self) -> Rcrs0offR {
        Rcrs0offR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RAM controller RAM sector 1 off"]
    #[inline(always)]
    pub fn rcrs1off(&self) -> Rcrs1offR {
        Rcrs1offR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RAM controller RAM sector 2 off"]
    #[inline(always)]
    pub fn rcrs2off(&self) -> Rcrs2offR {
        Rcrs2offR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RAM controller RAM sector 3 off"]
    #[inline(always)]
    pub fn rcrs3off(&self) -> Rcrs3offR {
        Rcrs3offR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored."]
    #[inline(always)]
    pub fn rckey(&self) -> RckeyR {
        RckeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs0off(&mut self) -> Rcrs0offW<'_, Rcctl0Spec> {
        Rcrs0offW::new(self, 0)
    }
    #[doc = "Bits 2:3 - RAM controller RAM sector 1 off"]
    #[inline(always)]
    pub fn rcrs1off(&mut self) -> Rcrs1offW<'_, Rcctl0Spec> {
        Rcrs1offW::new(self, 2)
    }
    #[doc = "Bits 4:5 - RAM controller RAM sector 2 off"]
    #[inline(always)]
    pub fn rcrs2off(&mut self) -> Rcrs2offW<'_, Rcctl0Spec> {
        Rcrs2offW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RAM controller RAM sector 3 off"]
    #[inline(always)]
    pub fn rcrs3off(&mut self) -> Rcrs3offW<'_, Rcctl0Spec> {
        Rcrs3offW::new(self, 6)
    }
    #[doc = "Bits 8:15 - RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored."]
    #[inline(always)]
    pub fn rckey(&mut self) -> RckeyW<'_, Rcctl0Spec> {
        RckeyW::new(self, 8)
    }
}
#[doc = "RAM Controller Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcctl0Spec;
impl crate::RegisterSpec for Rcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcctl0::R`](R) reader structure"]
impl crate::Readable for Rcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rcctl0::W`](W) writer structure"]
impl crate::Writable for Rcctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCCTL0 to value 0"]
impl crate::Resettable for Rcctl0Spec {}
