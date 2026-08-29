#[doc = "Register `SYSSNIV` reader"]
pub type R = crate::R<SyssnivSpec>;
#[doc = "Register `SYSSNIV` writer"]
pub type W = crate::W<SyssnivSpec>;
#[doc = "System NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Syssniv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Reserved"]
    Syssniv2 = 2,
    #[doc = "4: Uncorrectable FRAM bit error detection"]
    Ubdifg = 4,
    #[doc = "6: FRAM Access Time Error"]
    Accteifg = 6,
    #[doc = "8: MPUSEGPIFG encapsulated IP memory segment violation"]
    Mpusegpifg = 8,
    #[doc = "10: MPUSEGIIFG information memory segment violation"]
    Mpusegiifg = 10,
    #[doc = "12: MPUSEG1IFG segment 1 memory violation"]
    Mpuseg1ifg = 12,
    #[doc = "14: MPUSEG2IFG segment 2 memory violation"]
    Mpuseg2ifg = 14,
    #[doc = "16: MPUSEG3IFG segment 3 memory violation"]
    Mpuseg3ifg = 16,
    #[doc = "18: VMAIFG Vacant memory access"]
    Vmaifg = 18,
    #[doc = "20: JMBINIFG JTAG mailbox input"]
    Jmbinifg = 20,
    #[doc = "22: JMBOUTIFG JTAG mailbox output"]
    Jmboutifg = 22,
    #[doc = "24: Correctable FRAM bit error detection"]
    Cbdifg = 24,
    #[doc = "26: FRAM write protection detection"]
    Wprot = 26,
    #[doc = "28: LEA time-out fault"]
    Leato = 28,
    #[doc = "30: LEA command fault"]
    Leacmd = 30,
}
impl From<Syssniv> for u16 {
    #[inline(always)]
    fn from(variant: Syssniv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syssniv {
    type Ux = u16;
}
impl crate::IsEnum for Syssniv {}
#[doc = "Field `SYSSNIV` reader - System NMI vector"]
pub type SyssnivR = crate::FieldReader<Syssniv>;
impl SyssnivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syssniv> {
        match self.bits {
            0 => Some(Syssniv::None),
            2 => Some(Syssniv::Syssniv2),
            4 => Some(Syssniv::Ubdifg),
            6 => Some(Syssniv::Accteifg),
            8 => Some(Syssniv::Mpusegpifg),
            10 => Some(Syssniv::Mpusegiifg),
            12 => Some(Syssniv::Mpuseg1ifg),
            14 => Some(Syssniv::Mpuseg2ifg),
            16 => Some(Syssniv::Mpuseg3ifg),
            18 => Some(Syssniv::Vmaifg),
            20 => Some(Syssniv::Jmbinifg),
            22 => Some(Syssniv::Jmboutifg),
            24 => Some(Syssniv::Cbdifg),
            26 => Some(Syssniv::Wprot),
            28 => Some(Syssniv::Leato),
            30 => Some(Syssniv::Leacmd),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Syssniv::None
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_syssniv_2(&self) -> bool {
        *self == Syssniv::Syssniv2
    }
    #[doc = "Uncorrectable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_ubdifg(&self) -> bool {
        *self == Syssniv::Ubdifg
    }
    #[doc = "FRAM Access Time Error"]
    #[inline(always)]
    pub fn is_accteifg(&self) -> bool {
        *self == Syssniv::Accteifg
    }
    #[doc = "MPUSEGPIFG encapsulated IP memory segment violation"]
    #[inline(always)]
    pub fn is_mpusegpifg(&self) -> bool {
        *self == Syssniv::Mpusegpifg
    }
    #[doc = "MPUSEGIIFG information memory segment violation"]
    #[inline(always)]
    pub fn is_mpusegiifg(&self) -> bool {
        *self == Syssniv::Mpusegiifg
    }
    #[doc = "MPUSEG1IFG segment 1 memory violation"]
    #[inline(always)]
    pub fn is_mpuseg1ifg(&self) -> bool {
        *self == Syssniv::Mpuseg1ifg
    }
    #[doc = "MPUSEG2IFG segment 2 memory violation"]
    #[inline(always)]
    pub fn is_mpuseg2ifg(&self) -> bool {
        *self == Syssniv::Mpuseg2ifg
    }
    #[doc = "MPUSEG3IFG segment 3 memory violation"]
    #[inline(always)]
    pub fn is_mpuseg3ifg(&self) -> bool {
        *self == Syssniv::Mpuseg3ifg
    }
    #[doc = "VMAIFG Vacant memory access"]
    #[inline(always)]
    pub fn is_vmaifg(&self) -> bool {
        *self == Syssniv::Vmaifg
    }
    #[doc = "JMBINIFG JTAG mailbox input"]
    #[inline(always)]
    pub fn is_jmbinifg(&self) -> bool {
        *self == Syssniv::Jmbinifg
    }
    #[doc = "JMBOUTIFG JTAG mailbox output"]
    #[inline(always)]
    pub fn is_jmboutifg(&self) -> bool {
        *self == Syssniv::Jmboutifg
    }
    #[doc = "Correctable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_cbdifg(&self) -> bool {
        *self == Syssniv::Cbdifg
    }
    #[doc = "FRAM write protection detection"]
    #[inline(always)]
    pub fn is_wprot(&self) -> bool {
        *self == Syssniv::Wprot
    }
    #[doc = "LEA time-out fault"]
    #[inline(always)]
    pub fn is_leato(&self) -> bool {
        *self == Syssniv::Leato
    }
    #[doc = "LEA command fault"]
    #[inline(always)]
    pub fn is_leacmd(&self) -> bool {
        *self == Syssniv::Leacmd
    }
}
impl R {
    #[doc = "Bits 0:15 - System NMI vector"]
    #[inline(always)]
    pub fn syssniv(&self) -> SyssnivR {
        SyssnivR::new(self.bits)
    }
}
impl W {}
#[doc = "System NMI Vector Generator\n\nYou can [`read`](crate::Reg::read) this register and get [`syssniv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syssniv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyssnivSpec;
impl crate::RegisterSpec for SyssnivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syssniv::R`](R) reader structure"]
impl crate::Readable for SyssnivSpec {}
#[doc = "`write(|w| ..)` method takes [`syssniv::W`](W) writer structure"]
impl crate::Writable for SyssnivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSSNIV to value 0"]
impl crate::Resettable for SyssnivSpec {}
