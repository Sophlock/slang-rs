use shader_slang_sys::{slang_SpecializationArg, slang_SpecializationArg_Kind, slang_SpecializationArg__bindgen_ty_1};
use crate::reflection::Type;

#[repr(transparent)]
pub struct SpecializationArg(slang_SpecializationArg);

impl SpecializationArg {
    pub fn new(type_reflection: &Type) -> Self {
        Self{ 0: slang_SpecializationArg {
            kind: slang_SpecializationArg_Kind::Type,
            __bindgen_anon_1: slang_SpecializationArg__bindgen_ty_1 {
                type_: type_reflection as *const _ as *mut _
            }
        }
        }
    }
}