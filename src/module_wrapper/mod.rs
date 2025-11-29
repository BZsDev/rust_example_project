use crate::module_base;

pub struct TupleWrapper<T>(pub module_base::StructBase<T>);

impl<T: std::fmt::Display> TupleWrapper<T> {
    pub fn new(field_base: T) -> Self {
        Self {
            0: module_base::StructBase::new(field_base),
        }
    }

    pub fn fn_tuple_wrapper(&self) {
        println!("Print from fn_tuple_wrapper");
    }
}
