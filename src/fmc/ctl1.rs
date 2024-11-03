#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `PG` reader - Main flash program for bank1 command bit"]
pub type PgR = crate::BitReader;
#[doc = "Field `PG` writer - Main flash program for bank1 command bit"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Main flash page erase for bank1 command bit"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Main flash page erase for bank1 command bit"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Main flash mass erase for bank1 command bit"]
pub type MerR = crate::BitReader;
#[doc = "Field `MER` writer - Main flash mass erase for bank1 command bit"]
pub type MerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Send erase command to FMC bit"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Send erase command to FMC bit"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK` reader - FMC_CTL1 lock bit"]
pub type LkR = crate::BitReader;
#[doc = "Field `LK` writer - FMC_CTL1 lock bit"]
pub type LkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable bit"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable bit"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type EndieR = crate::BitReader;
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type EndieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main flash program for bank1 command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank1 command bit"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank1 command bit"]
    #[inline(always)]
    pub fn mer(&self) -> MerR {
        MerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FMC_CTL1 lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LkR {
        LkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&self) -> EndieR {
        EndieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main flash program for bank1 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PgW<Ctl1Spec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank1 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<Ctl1Spec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank1 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MerW<Ctl1Spec> {
        MerW::new(self, 2)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<Ctl1Spec> {
        StartW::new(self, 6)
    }
    #[doc = "Bit 7 - FMC_CTL1 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LkW<Ctl1Spec> {
        LkW::new(self, 7)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ctl1Spec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn endie(&mut self) -> EndieW<Ctl1Spec> {
        EndieW::new(self, 12)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x80"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x80;
}
