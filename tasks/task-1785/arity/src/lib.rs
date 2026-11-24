use proc_macro::TokenStream;
#[proc_macro]
pub fn arity(input: TokenStream) -> TokenStream {
    input.into_iter().count().to_string().parse().unwrap()
}
