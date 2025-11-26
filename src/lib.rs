mod module_base;
mod module_trait;

use module_trait::TraitA;

pub fn run() {
    let struct_base = module_base::StructBase::new("Base struct init string");

    struct_base.fn_struct_base_print_field_base();

    module_base::StructBase::<String>::fn_struct_base();

    struct_base.fn_trait_a();
}
