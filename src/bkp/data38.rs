#[doc = "Register `DATA38` reader"]
pub type R = crate::R<Data38Spec>;
#[doc = "Register `DATA38` writer"]
pub type W = crate::W<Data38Spec>;
#[doc = "Field `DATA` reader - Backup data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Data38Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Backup data register 38\n\nYou can [`read`](crate::Reg::read) this register and get [`data38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data38Spec;
impl crate::RegisterSpec for Data38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data38::R`](R) reader structure"]
impl crate::Readable for Data38Spec {}
#[doc = "`write(|w| ..)` method takes [`data38::W`](W) writer structure"]
impl crate::Writable for Data38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA38 to value 0"]
impl crate::Resettable for Data38Spec {
    const RESET_VALUE: u32 = 0;
}
