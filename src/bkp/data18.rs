#[doc = "Register `DATA18` reader"]
pub type R = crate::R<Data18Spec>;
#[doc = "Register `DATA18` writer"]
pub type W = crate::W<Data18Spec>;
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
    pub fn data(&mut self) -> DataW<Data18Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Backup data register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`data18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data18Spec;
impl crate::RegisterSpec for Data18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data18::R`](R) reader structure"]
impl crate::Readable for Data18Spec {}
#[doc = "`write(|w| ..)` method takes [`data18::W`](W) writer structure"]
impl crate::Writable for Data18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA18 to value 0"]
impl crate::Resettable for Data18Spec {
    const RESET_VALUE: u32 = 0;
}
