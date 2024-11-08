#[doc = "Register `PCF0` reader"]
pub type R = crate::R<Pcf0Spec>;
#[doc = "Register `PCF0` writer"]
pub type W = crate::W<Pcf0Spec>;
#[doc = "Field `SPI0_REMAP` reader - SPI0 remapping"]
pub type Spi0RemapR = crate::BitReader;
#[doc = "Field `SPI0_REMAP` writer - SPI0 remapping"]
pub type Spi0RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_REMAP` reader - I2C0 remapping"]
pub type I2c0RemapR = crate::BitReader;
#[doc = "Field `I2C0_REMAP` writer - I2C0 remapping"]
pub type I2c0RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0_REMAP` reader - USART0 remapping"]
pub type Usart0RemapR = crate::BitReader;
#[doc = "Field `USART0_REMAP` writer - USART0 remapping"]
pub type Usart0RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_REMAP` reader - USART1 remapping"]
pub type Usart1RemapR = crate::BitReader;
#[doc = "Field `USART1_REMAP` writer - USART1 remapping"]
pub type Usart1RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2_REMAP` reader - USART2 remapping"]
pub type Usart2RemapR = crate::FieldReader;
#[doc = "Field `USART2_REMAP` writer - USART2 remapping"]
pub type Usart2RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER0_REMAP` reader - TIMER0 remapping"]
pub type Timer0RemapR = crate::FieldReader;
#[doc = "Field `TIMER0_REMAP` writer - TIMER0 remapping"]
pub type Timer0RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER1_REMAP` reader - TIMER1 remapping"]
pub type Timer1RemapR = crate::FieldReader;
#[doc = "Field `TIMER1_REMAP` writer - TIMER1 remapping"]
pub type Timer1RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER2_REMAP` reader - TIMER2 remapping"]
pub type Timer2RemapR = crate::FieldReader;
#[doc = "Field `TIMER2_REMAP` writer - TIMER2 remapping"]
pub type Timer2RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER3_REMAP` reader - TIMER3 remapping"]
pub type Timer3RemapR = crate::BitReader;
#[doc = "Field `TIMER3_REMAP` writer - TIMER3 remapping"]
pub type Timer3RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN_REMAP` reader - CAN alternate interface remapping"]
pub type CanRemapR = crate::FieldReader;
#[doc = "Field `CAN_REMAP` writer - CAN alternate interface remapping"]
pub type CanRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD01_REMAP` reader - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
pub type Pd01RemapR = crate::BitReader;
#[doc = "Field `PD01_REMAP` writer - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
pub type Pd01RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4CH3_IREMAP` reader - TIMER4 channel3 internal remapping"]
pub type Timer4ch3IremapR = crate::BitReader;
#[doc = "Field `TIMER4CH3_IREMAP` writer - TIMER4 channel3 internal remapping"]
pub type Timer4ch3IremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0_ETRGINS_REMAP` reader - ADC0 external trigger inserted conversion remapping"]
pub type Adc0EtrginsRemapR = crate::BitReader;
#[doc = "Field `ADC0_ETRGINS_REMAP` writer - ADC0 external trigger inserted conversion remapping"]
pub type Adc0EtrginsRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0_ETRGREG_REMAP` reader - ADC0 external trigger regular conversion remapping"]
pub type Adc0EtrgregRemapR = crate::BitReader;
#[doc = "Field `ADC0_ETRGREG_REMAP` writer - ADC0 external trigger regular conversion remapping"]
pub type Adc0EtrgregRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETRGINS_REMAP` reader - ADC1 external trigger inserted conversion remapping"]
pub type Adc1EtrginsRemapR = crate::BitReader;
#[doc = "Field `ADC1_ETRGINS_REMAP` writer - ADC1 external trigger inserted conversion remapping"]
pub type Adc1EtrginsRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETRGREG_REMAP` reader - ADC1 external trigger regular conversion remapping"]
pub type Adc1EtrgregRemapR = crate::BitReader;
#[doc = "Field `ADC1_ETRGREG_REMAP` writer - ADC1 external trigger regular conversion remapping"]
pub type Adc1EtrgregRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJ_CFG` reader - Serial wire JTAG configuration"]
pub type SwjCfgR = crate::FieldReader;
#[doc = "Field `SWJ_CFG` writer - Serial wire JTAG configuration"]
pub type SwjCfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline(always)]
    pub fn spi0_remap(&self) -> Spi0RemapR {
        Spi0RemapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline(always)]
    pub fn i2c0_remap(&self) -> I2c0RemapR {
        I2c0RemapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline(always)]
    pub fn usart0_remap(&self) -> Usart0RemapR {
        Usart0RemapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&self) -> Usart1RemapR {
        Usart1RemapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&self) -> Usart2RemapR {
        Usart2RemapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline(always)]
    pub fn timer0_remap(&self) -> Timer0RemapR {
        Timer0RemapR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline(always)]
    pub fn timer1_remap(&self) -> Timer1RemapR {
        Timer1RemapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline(always)]
    pub fn timer2_remap(&self) -> Timer2RemapR {
        Timer2RemapR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline(always)]
    pub fn timer3_remap(&self) -> Timer3RemapR {
        Timer3RemapR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN alternate interface remapping"]
    #[inline(always)]
    pub fn can_remap(&self) -> CanRemapR {
        CanRemapR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_remap(&self) -> Pd01RemapR {
        Pd01RemapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline(always)]
    pub fn timer4ch3_iremap(&self) -> Timer4ch3IremapR {
        Timer4ch3IremapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC0 external trigger inserted conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrgins_remap(&self) -> Adc0EtrginsRemapR {
        Adc0EtrginsRemapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC0 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrgreg_remap(&self) -> Adc0EtrgregRemapR {
        Adc0EtrgregRemapR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC1 external trigger inserted conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgins_remap(&self) -> Adc1EtrginsRemapR {
        Adc1EtrginsRemapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&self) -> Adc1EtrgregRemapR {
        Adc1EtrgregRemapR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    pub fn swj_cfg(&self) -> SwjCfgR {
        SwjCfgR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi0_remap(&mut self) -> Spi0RemapW<Pcf0Spec> {
        Spi0RemapW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_remap(&mut self) -> I2c0RemapW<Pcf0Spec> {
        I2c0RemapW::new(self, 1)
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart0_remap(&mut self) -> Usart0RemapW<Pcf0Spec> {
        Usart0RemapW::new(self, 2)
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_remap(&mut self) -> Usart1RemapW<Pcf0Spec> {
        Usart1RemapW::new(self, 3)
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart2_remap(&mut self) -> Usart2RemapW<Pcf0Spec> {
        Usart2RemapW::new(self, 4)
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_remap(&mut self) -> Timer0RemapW<Pcf0Spec> {
        Timer0RemapW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_remap(&mut self) -> Timer1RemapW<Pcf0Spec> {
        Timer1RemapW::new(self, 8)
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_remap(&mut self) -> Timer2RemapW<Pcf0Spec> {
        Timer2RemapW::new(self, 10)
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_remap(&mut self) -> Timer3RemapW<Pcf0Spec> {
        Timer3RemapW::new(self, 12)
    }
    #[doc = "Bits 13:14 - CAN alternate interface remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can_remap(&mut self) -> CanRemapW<Pcf0Spec> {
        CanRemapW::new(self, 13)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_remap(&mut self) -> Pd01RemapW<Pcf0Spec> {
        Pd01RemapW::new(self, 15)
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline(always)]
    #[must_use]
    pub fn timer4ch3_iremap(&mut self) -> Timer4ch3IremapW<Pcf0Spec> {
        Timer4ch3IremapW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC0 external trigger inserted conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_etrgins_remap(&mut self) -> Adc0EtrginsRemapW<Pcf0Spec> {
        Adc0EtrginsRemapW::new(self, 17)
    }
    #[doc = "Bit 18 - ADC0 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_etrgreg_remap(&mut self) -> Adc0EtrgregRemapW<Pcf0Spec> {
        Adc0EtrgregRemapW::new(self, 18)
    }
    #[doc = "Bit 19 - ADC1 external trigger inserted conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrgins_remap(&mut self) -> Adc1EtrginsRemapW<Pcf0Spec> {
        Adc1EtrginsRemapW::new(self, 19)
    }
    #[doc = "Bit 20 - ADC1 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrgreg_remap(&mut self) -> Adc1EtrgregRemapW<Pcf0Spec> {
        Adc1EtrgregRemapW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swj_cfg(&mut self) -> SwjCfgW<Pcf0Spec> {
        SwjCfgW::new(self, 24)
    }
}
#[doc = "AFIO port configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcf0Spec;
impl crate::RegisterSpec for Pcf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf0::R`](R) reader structure"]
impl crate::Readable for Pcf0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcf0::W`](W) writer structure"]
impl crate::Writable for Pcf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCF0 to value 0"]
impl crate::Resettable for Pcf0Spec {
    const RESET_VALUE: u32 = 0;
}
