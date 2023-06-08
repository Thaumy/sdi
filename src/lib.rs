#![crate_type = "proc-macro"]

use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use proc_macro::TokenStream;

lazy_static! {
    static ref MAP: Mutex<HashMap<String, String>> =
        Mutex::new(HashMap::new());
}

/// Register an statically resolved service expression indexed by key.
///
/// ## Example
///
/// ```rust
/// use sdi::{inject, provide};
///
/// #[derive(Debug, PartialEq)]
/// struct A;
///
/// impl A {
///     pub fn new() -> A { A }
///     provide!(A <- A::new());
/// }
///
/// assert_eq!(A::new(), inject!(A))
/// ```
/// Provide by inject is also ok.
/// ```rust
/// use sdi::{inject, provide};
///
/// #[derive(Debug, PartialEq)]
/// struct A;
/// provide!(A <- A);
///
/// #[derive(Debug, PartialEq)]
/// struct B(A);
///
/// impl B {
///     pub fn new(a:A) -> B { B(a) }
///     provide!(B <- B::new(inject!(A)));
/// }
///
/// assert_eq!(B::new(A), inject!(B))
/// ```
#[proc_macro]
pub fn provide(attr: TokenStream) -> TokenStream {
    let (key, expr) = {
        let stringify = attr.to_string();
        let split: Vec<String> = stringify
            .split(" <- ")
            .map(|str| str.to_owned())
            .collect();

        let err_msg = "Invalid provider syntax, consider: provide!(<key> <- <expr>)";
        if split.len() != 2 {
            panic!("{}", err_msg);
        }

        let key = split
            .get(0)
            .expect(err_msg)
            .to_owned();
        let expr = split
            .get(1)
            .expect(err_msg)
            .to_owned();

        (key, expr)
    };

    MAP.lock()
        .unwrap()
        .insert(key, expr);

    TokenStream::new()
}

/// Get an statically resolved service expression by key
///
/// ## Example
///
/// ```rust
/// use sdi::{inject, provide};
///
/// #[derive(Debug, PartialEq)]
/// struct A;
///
/// impl A {
///     pub fn new() -> A { A }
///     provide!(A <- A::new());
/// }
///
/// assert_eq!(A::new(), inject!(A))
/// ```
#[proc_macro]
pub fn inject(attr: TokenStream) -> TokenStream {
    let key = attr.to_string();
    MAP.lock()
        .unwrap()
        .get(&key)
        .expect(&format!("Unknown inject key: `{}`", key))
        .parse()
        .unwrap()
}
