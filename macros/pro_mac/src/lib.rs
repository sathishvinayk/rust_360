use proc_macro::TokenStream;

#[proc_macro]
pub fn function_proc(_item: TokenStream) -> TokenStream  {
    "struct A { 
        x: i32
    }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn attribute_proc(attribute: TokenStream, item: TokenStream) -> TokenStream {
    println!("attribute {}", attribute);
    println!("item {}", item);
    item
}

#[proc_macro_derive(JunkTag)]
pub fn derive_proc(_item: TokenStream) -> TokenStream {
    "fn xyz() -> i8 {
        4
    }".parse().unwrap()
}