#[macro_use]
extern crate proc_macro_hack;

extern crate proc_macro;
extern crate proc_macro2; // XXX

proc_macro_expr_impl! {
    /// Add one to an expression.
    pub fn add_one_impl(input: &str) -> String {
        format!("1 + {}", input)
    }
}

proc_macro_item_impl! {
    /// A function that always returns 2.
    pub fn two_fn_impl(input: &str) -> String {
        format!("fn {}() -> u8 {{ 2 }}", input)
    }
}

proc_macro2_item_impl! {
    /// A function that always returns 2.
    pub fn id_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        input
    }
}
