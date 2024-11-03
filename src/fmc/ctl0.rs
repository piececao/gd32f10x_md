#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `PG` reader - Main flash program for bank0 command bit"]
pub type PgR = crate::BitReader;
#[doc = "Field `PG` writer - Main flash program for bank0 command bit"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Main flash page erase for bank0 command bit"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Main flash page erase for bank0 command bit"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Main flash mass erase for bank0 command bit"]
pub type MerR = crate::BitReader;
#[doc = "Field `MER` writer - Main flash mass erase for bank0 command bit"]
pub type MerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBPG` reader - Option bytes program command bit"]
pub type ObpgR = crate::BitReader;
#[doc = "Field `OBPG` writer - Option bytes program command bit"]
pub type ObpgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBER` reader - Option bytes erase command bit"]
pub type OberR = crate::BitReader;
#[doc = "Field `OBER` writer - Option bytes erase command bit"]
pub type OberW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Send erase command to FMC bit"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Send erase command to FMC bit"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK` reader - FMC_CTL0 lock bit"]
pub type LkR = crate::BitReader;
#[doc = "Field `LK` writer - FMC_CTL0 lock bit"]
pub type LkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBWEN` reader - Option byte erase/program enable bit"]
pub type ObwenR = crate::BitReader;
#[doc = "Field `OBWEN` writer - Option byte erase/program enable bit"]
pub type ObwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable bit"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable bit"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type EndieR = crate::BitReader;
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type EndieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    pub fn mer(&self) -> MerR {
        MerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option bytes program command bit"]
    #[inline(always)]
    pub fn obpg(&self) -> ObpgR {
        ObpgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option bytes erase command bit"]
    #[inline(always)]
    pub fn ober(&self) -> OberR {
        OberR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LkR {
        LkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option byte erase/program enable bit"]
    #[inline(always)]
    pub fn obwen(&self) -> ObwenR {
        ObwenR::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PgW<Ctl0Spec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<Ctl0Spec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MerW<Ctl0Spec> {
        MerW::new(self, 2)
    }
    #[doc = "Bit 4 - Option bytes program command bit"]
    #[inline(always)]
    #[must_use]
    pub fn obpg(&mut self) -> ObpgW<Ctl0Spec> {
        ObpgW::new(self, 4)
    }
    #[doc = "Bit 5 - Option bytes erase command bit"]
    #[inline(always)]
    #[must_use]
    pub fn ober(&mut self) -> OberW<Ctl0Spec> {
        OberW::new(self, 5)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<Ctl0Spec> {
        StartW::new(self, 6)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LkW<Ctl0Spec> {
        LkW::new(self, 7)
    }
    #[doc = "Bit 9 - Option byte erase/program enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn obwen(&mut self) -> ObwenW<Ctl0Spec> {
        ObwenW::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ctl0Spec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn endie(&mut self) -> EndieW<Ctl0Spec> {
        EndieW::new(self, 12)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x80"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x80;
}
