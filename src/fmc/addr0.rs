#[doc = "Register `ADDR0` writer"]
pub type W = crate::W<Addr0Spec>;
#[doc = "Field `ADDR` writer - Flash erase/program command address bits"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash erase/program command address bits"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Addr0Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Address register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr0Spec;
impl crate::RegisterSpec for Addr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`addr0::W`](W) writer structure"]
impl crate::Writable for Addr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::Resettable for Addr0Spec {
    const RESET_VALUE: u32 = 0;
}
