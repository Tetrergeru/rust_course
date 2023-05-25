use proc_macro::TokenStream;
use syn::{
    parse::{self, Parse},
    parse_macro_input,
};

use quote::quote;

struct Item {
    lit: syn::LitStr,
}

impl Parse for Item {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let lit: syn::LitStr = input.parse()?;

        Ok(Self { lit })
    }
}

#[proc_macro]
pub fn my_println(input: TokenStream) -> TokenStream {
    let input: Item = parse_macro_input!(input as Item);

    let value = input.lit.value();
    let mut strings = vec![];

    let mut start = 0;

    for (idx, ch) in value.chars().enumerate() {
        if ch == '*' {
            strings.push(&value[start..idx]);
            start = idx + 1;
        }
    }

    strings.push(&value[start..]);

    let strings: Vec<_> = strings
        .into_iter()
        .map(|it| syn::LitStr::new(it, input.lit.span()))
        .collect();

    let res = quote! {
        {
            #( print!( #strings ); );* ;
            println!();
        }
    };

    res.into()
}
