#[doc = "Register `SFRRPCR` reader"]
pub type R = crate::R<SfrrpcrSpec>;
#[doc = "Register `SFRRPCR` writer"]
pub type W = crate::W<SfrrpcrSpec>;
#[doc = "Field `SYSNMI` reader - NMI select"]
pub type SysnmiR = crate::BitReader;
#[doc = "Field `SYSNMI` writer - NMI select"]
pub type SysnmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSNMIIES` reader - NMI edge select"]
pub type SysnmiiesR = crate::BitReader;
#[doc = "Field `SYSNMIIES` writer - NMI edge select"]
pub type SysnmiiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSTUP` reader - RESET Pin pull down/up select"]
pub type SysrstupR = crate::BitReader;
#[doc = "Field `SYSRSTUP` writer - RESET Pin pull down/up select"]
pub type SysrstupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSTRE` reader - RESET Pin Resistor enable"]
pub type SysrstreR = crate::BitReader;
#[doc = "Field `SYSRSTRE` writer - RESET Pin Resistor enable"]
pub type SysrstreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SysnmiR {
        SysnmiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SysnmiiesR {
        SysnmiiesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SysrstupR {
        SysrstupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SysrstreR {
        SysrstreR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&mut self) -> SysnmiW<'_, SfrrpcrSpec> {
        SysnmiW::new(self, 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&mut self) -> SysnmiiesW<'_, SfrrpcrSpec> {
        SysnmiiesW::new(self, 1)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&mut self) -> SysrstupW<'_, SfrrpcrSpec> {
        SysrstupW::new(self, 2)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&mut self) -> SysrstreW<'_, SfrrpcrSpec> {
        SysrstreW::new(self, 3)
    }
}
#[doc = "RESET Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrrpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrrpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrrpcrSpec;
impl crate::RegisterSpec for SfrrpcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sfrrpcr::R`](R) reader structure"]
impl crate::Readable for SfrrpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrrpcr::W`](W) writer structure"]
impl crate::Writable for SfrrpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRRPCR to value 0"]
impl crate::Resettable for SfrrpcrSpec {}
