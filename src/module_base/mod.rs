pub struct StructBase<T> {
    field_base: T,
}

impl<T: std::fmt::Display> StructBase<T> {
    pub fn new(field_base: T) -> Self {
        Self { field_base }
    }

    pub fn fn_struct_base_print_field_base(&self) {
        println!("{}", self.field_base);
    }

    pub fn fn_struct_base() {
        println!("Print from fn_struct_base");
    }
}
