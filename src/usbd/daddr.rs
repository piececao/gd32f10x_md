#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DaddrSpec>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DaddrSpec>;
#[doc = "Field `USBDAR` reader - Device address"]
pub type UsbdarR = crate::FieldReader;
#[doc = "Field `USBDAR` writer - Device address"]
pub type UsbdarW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `USBEN` reader - USB device enable"]
pub type UsbenR = crate::BitReader;
#[doc = "Field `USBEN` writer - USB device enable"]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    pub fn usbdar(&self) -> UsbdarR {
        UsbdarR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn usbdar(&mut self) -> UsbdarW<DaddrSpec> {
        UsbdarW::new(self, 0)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> UsbenW<DaddrSpec> {
        UsbenW::new(self, 7)
    }
}
#[doc = "device address register\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaddrSpec;
impl crate::RegisterSpec for DaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DaddrSpec {
    const RESET_VALUE: u32 = 0;
}
