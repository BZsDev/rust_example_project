use crate::module_base;

pub trait TraitA {
    fn fn_trait_a(&self);
}

impl<T> TraitA for module_base::StructBase<T> {
    fn fn_trait_a(&self) {
        println!("Print from fn_trait_a");
    }
}
