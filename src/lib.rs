mod module_base;
mod module_trait;
mod module_wrapper;

use module_trait::TraitA;

pub fn run() {
    let struct_base = module_base::StructBase::new("Base struct init string");

    struct_base.fn_struct_base_print_field_base();

    module_base::StructBase::<String>::fn_struct_base_no_self();

    struct_base.fn_trait_a();

    let struct_wrapper = module_wrapper::TupleWrapper::new("Base struct init string");

    struct_wrapper.fn_tuple_wrapper();
    struct_wrapper.0.fn_struct_base_print_field_base();
}
