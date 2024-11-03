#[doc = "Register `OUT1_DO` reader"]
pub type R = crate::R<Out1DoSpec>;
#[doc = "Field `OUT1_DO` reader - DACx_OUT1 12-bit output data"]
pub type Out1DoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DACx_OUT1 12-bit output data"]
    #[inline(always)]
    pub fn out1_do(&self) -> Out1DoR {
        Out1DoR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DACx_OUT1 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`out1_do::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Out1DoSpec;
impl crate::RegisterSpec for Out1DoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1_do::R`](R) reader structure"]
impl crate::Readable for Out1DoSpec {}
#[doc = "`reset()` method sets OUT1_DO to value 0"]
impl crate::Resettable for Out1DoSpec {
    const RESET_VALUE: u32 = 0;
}
