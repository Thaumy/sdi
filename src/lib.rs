#![crate_type = "proc-macro"]

use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use proc_macro::TokenStream;

lazy_static! {
    static ref MAP: Mutex<HashMap<String, String>> =
        Mutex::new(HashMap::new());
}

#[proc_macro]
pub fn provide(attr: TokenStream) -> TokenStream {
    let (token, expr) = {
        let stringify = attr.to_string();
        let mut split = stringify.split("by");

        let token = split
            .next()
            .unwrap()
            .trim()
            .to_owned();
        let expr = split
            .next()
            .unwrap()
            .trim()
            .to_owned();

        (token, expr)
    };
    MAP.lock()
        .unwrap()
        .insert(token, expr);
    TokenStream::new()
}

#[proc_macro]
pub fn inject(attr: TokenStream) -> TokenStream {
    let stringify = attr.to_string();
    MAP.lock()
        .unwrap()
        .get(&stringify)
        .unwrap()
        .parse()
        .unwrap()
}
