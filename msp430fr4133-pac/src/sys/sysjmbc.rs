#[doc = "Register `SYSJMBC` reader"]
pub type R = crate::R<SysjmbcSpec>;
#[doc = "Register `SYSJMBC` writer"]
pub type W = crate::W<SysjmbcSpec>;
#[doc = "Field `JMBIN0FG` reader - SYS - Incoming JTAG Mailbox 0 Flag"]
pub type Jmbin0fgR = crate::BitReader;
#[doc = "Field `JMBIN0FG` writer - SYS - Incoming JTAG Mailbox 0 Flag"]
pub type Jmbin0fgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBIN1FG` reader - SYS - Incoming JTAG Mailbox 1 Flag"]
pub type Jmbin1fgR = crate::BitReader;
#[doc = "Field `JMBIN1FG` writer - SYS - Incoming JTAG Mailbox 1 Flag"]
pub type Jmbin1fgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUT0FG` reader - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub type Jmbout0fgR = crate::BitReader;
#[doc = "Field `JMBOUT0FG` writer - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub type Jmbout0fgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBOUT1FG` reader - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub type Jmbout1fgR = crate::BitReader;
#[doc = "Field `JMBOUT1FG` writer - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub type Jmbout1fgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBMODE` reader - SYS - JMB 16/32 Bit Mode"]
pub type JmbmodeR = crate::BitReader;
#[doc = "Field `JMBMODE` writer - SYS - JMB 16/32 Bit Mode"]
pub type JmbmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBCLR0OFF` reader - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub type Jmbclr0offR = crate::BitReader;
#[doc = "Field `JMBCLR0OFF` writer - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub type Jmbclr0offW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMBCLR1OFF` reader - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub type Jmbclr1offR = crate::BitReader;
#[doc = "Field `JMBCLR1OFF` writer - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub type Jmbclr1offW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> Jmbin0fgR {
        Jmbin0fgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> Jmbin1fgR {
        Jmbin1fgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> Jmbout0fgR {
        Jmbout0fgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> Jmbout1fgR {
        Jmbout1fgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JmbmodeR {
        JmbmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> Jmbclr0offR {
        Jmbclr0offR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> Jmbclr1offR {
        Jmbclr1offR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&mut self) -> Jmbin0fgW<'_, SysjmbcSpec> {
        Jmbin0fgW::new(self, 0)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&mut self) -> Jmbin1fgW<'_, SysjmbcSpec> {
        Jmbin1fgW::new(self, 1)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&mut self) -> Jmbout0fgW<'_, SysjmbcSpec> {
        Jmbout0fgW::new(self, 2)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&mut self) -> Jmbout1fgW<'_, SysjmbcSpec> {
        Jmbout1fgW::new(self, 3)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&mut self) -> JmbmodeW<'_, SysjmbcSpec> {
        JmbmodeW::new(self, 4)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&mut self) -> Jmbclr0offW<'_, SysjmbcSpec> {
        Jmbclr0offW::new(self, 6)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&mut self) -> Jmbclr1offW<'_, SysjmbcSpec> {
        Jmbclr1offW::new(self, 7)
    }
}
#[doc = "JTAG mailbox control\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysjmbcSpec;
impl crate::RegisterSpec for SysjmbcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbc::R`](R) reader structure"]
impl crate::Readable for SysjmbcSpec {}
#[doc = "`write(|w| ..)` method takes [`sysjmbc::W`](W) writer structure"]
impl crate::Writable for SysjmbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSJMBC to value 0"]
impl crate::Resettable for SysjmbcSpec {}
