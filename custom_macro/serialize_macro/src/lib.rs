use proc_macro::TokenStream;

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialize_number_struct(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialize_number_struct(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
