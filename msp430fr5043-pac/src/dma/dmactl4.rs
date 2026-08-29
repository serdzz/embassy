#[doc = "Register `DMACTL4` reader"]
pub type R = crate::R<Dmactl4Spec>;
#[doc = "Register `DMACTL4` writer"]
pub type W = crate::W<Dmactl4Spec>;
#[doc = "Enable NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ennmi {
    #[doc = "0: NMI does not interrupt DMA transfer"]
    Ennmi0 = 0,
    #[doc = "1: NMI interrupts a DMA transfer"]
    Ennmi1 = 1,
}
impl From<Ennmi> for bool {
    #[inline(always)]
    fn from(variant: Ennmi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENNMI` reader - Enable NMI"]
pub type EnnmiR = crate::BitReader<Ennmi>;
impl EnnmiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ennmi {
        match self.bits {
            false => Ennmi::Ennmi0,
            true => Ennmi::Ennmi1,
        }
    }
    #[doc = "NMI does not interrupt DMA transfer"]
    #[inline(always)]
    pub fn is_ennmi_0(&self) -> bool {
        *self == Ennmi::Ennmi0
    }
    #[doc = "NMI interrupts a DMA transfer"]
    #[inline(always)]
    pub fn is_ennmi_1(&self) -> bool {
        *self == Ennmi::Ennmi1
    }
}
#[doc = "Field `ENNMI` writer - Enable NMI"]
pub type EnnmiW<'a, REG> = crate::BitWriter<'a, REG, Ennmi>;
impl<'a, REG> EnnmiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NMI does not interrupt DMA transfer"]
    #[inline(always)]
    pub fn ennmi_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ennmi::Ennmi0)
    }
    #[doc = "NMI interrupts a DMA transfer"]
    #[inline(always)]
    pub fn ennmi_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ennmi::Ennmi1)
    }
}
#[doc = "Round robin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Roundrobin {
    #[doc = "0: DMA channel priority is DMA0-DMA1-DMA2 - ... - DMA7"]
    Roundrobin0 = 0,
    #[doc = "1: DMA channel priority changes with each transfer"]
    Roundrobin1 = 1,
}
impl From<Roundrobin> for bool {
    #[inline(always)]
    fn from(variant: Roundrobin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROUNDROBIN` reader - Round robin"]
pub type RoundrobinR = crate::BitReader<Roundrobin>;
impl RoundrobinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Roundrobin {
        match self.bits {
            false => Roundrobin::Roundrobin0,
            true => Roundrobin::Roundrobin1,
        }
    }
    #[doc = "DMA channel priority is DMA0-DMA1-DMA2 - ... - DMA7"]
    #[inline(always)]
    pub fn is_roundrobin_0(&self) -> bool {
        *self == Roundrobin::Roundrobin0
    }
    #[doc = "DMA channel priority changes with each transfer"]
    #[inline(always)]
    pub fn is_roundrobin_1(&self) -> bool {
        *self == Roundrobin::Roundrobin1
    }
}
#[doc = "Field `ROUNDROBIN` writer - Round robin"]
pub type RoundrobinW<'a, REG> = crate::BitWriter<'a, REG, Roundrobin>;
impl<'a, REG> RoundrobinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA channel priority is DMA0-DMA1-DMA2 - ... - DMA7"]
    #[inline(always)]
    pub fn roundrobin_0(self) -> &'a mut crate::W<REG> {
        self.variant(Roundrobin::Roundrobin0)
    }
    #[doc = "DMA channel priority changes with each transfer"]
    #[inline(always)]
    pub fn roundrobin_1(self) -> &'a mut crate::W<REG> {
        self.variant(Roundrobin::Roundrobin1)
    }
}
#[doc = "Read-modify-write disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarmwdis {
    #[doc = "0: DMA transfers can occur during read-modify-write CPU operations"]
    Dmarmwdis0 = 0,
    #[doc = "1: DMA transfers inhibited during read-modify-write CPU operations"]
    Dmarmwdis1 = 1,
}
impl From<Dmarmwdis> for bool {
    #[inline(always)]
    fn from(variant: Dmarmwdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMARMWDIS` reader - Read-modify-write disable"]
pub type DmarmwdisR = crate::BitReader<Dmarmwdis>;
impl DmarmwdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarmwdis {
        match self.bits {
            false => Dmarmwdis::Dmarmwdis0,
            true => Dmarmwdis::Dmarmwdis1,
        }
    }
    #[doc = "DMA transfers can occur during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn is_dmarmwdis_0(&self) -> bool {
        *self == Dmarmwdis::Dmarmwdis0
    }
    #[doc = "DMA transfers inhibited during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn is_dmarmwdis_1(&self) -> bool {
        *self == Dmarmwdis::Dmarmwdis1
    }
}
#[doc = "Field `DMARMWDIS` writer - Read-modify-write disable"]
pub type DmarmwdisW<'a, REG> = crate::BitWriter<'a, REG, Dmarmwdis>;
impl<'a, REG> DmarmwdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfers can occur during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn dmarmwdis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarmwdis::Dmarmwdis0)
    }
    #[doc = "DMA transfers inhibited during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn dmarmwdis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmarmwdis::Dmarmwdis1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable NMI"]
    #[inline(always)]
    pub fn ennmi(&self) -> EnnmiR {
        EnnmiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Round robin"]
    #[inline(always)]
    pub fn roundrobin(&self) -> RoundrobinR {
        RoundrobinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read-modify-write disable"]
    #[inline(always)]
    pub fn dmarmwdis(&self) -> DmarmwdisR {
        DmarmwdisR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable NMI"]
    #[inline(always)]
    pub fn ennmi(&mut self) -> EnnmiW<'_, Dmactl4Spec> {
        EnnmiW::new(self, 0)
    }
    #[doc = "Bit 1 - Round robin"]
    #[inline(always)]
    pub fn roundrobin(&mut self) -> RoundrobinW<'_, Dmactl4Spec> {
        RoundrobinW::new(self, 1)
    }
    #[doc = "Bit 2 - Read-modify-write disable"]
    #[inline(always)]
    pub fn dmarmwdis(&mut self) -> DmarmwdisW<'_, Dmactl4Spec> {
        DmarmwdisW::new(self, 2)
    }
}
#[doc = "DMA Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmactl4Spec;
impl crate::RegisterSpec for Dmactl4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmactl4::R`](R) reader structure"]
impl crate::Readable for Dmactl4Spec {}
#[doc = "`write(|w| ..)` method takes [`dmactl4::W`](W) writer structure"]
impl crate::Writable for Dmactl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTL4 to value 0"]
impl crate::Resettable for Dmactl4Spec {}
