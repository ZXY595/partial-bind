/// A proc-macro to bind some arguments of a function to a closure.
/// Which is known as partial function application.
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{ExprCall, ExprPath, Path, PathSegment};

/// Binds some arguments of a function to a closure.
/// The arguments to be bound are denoted by `_` (underscore) in the function signature.
///
/// # Example:
/// ```rust
/// fn foo(a: i32, b: i32, c: i32, d: i32) { a + b + c + d }
/// let bar = bind(foo(1, _, 3, _));
/// assert_eq!(bar(2, 4), foo(1, 2, 3, 4));
///
/// let baz = bind(foo(_, 2, _, 4));
/// assert_eq!(baz(1, 3), foo(1, 2, 3, 4));
/// ```
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let mut call: ExprCall = syn::parse2(input).unwrap();

    // Token![_] => "_n"
    // Expr => Expr,
    let placeholders = call
        .args
        .pairs_mut()
        .filter_map(|pair| {
            let value = pair.into_value();
            matches!(value, syn::Expr::Infer(_)).then_some(value)
        })
        .enumerate()
        .map(|(i, underscore)| {
            // Ident => Expr
            let ident = format!("__{}", i);
            let ident = Ident::new(&ident, Span::call_site());
            let segment = PathSegment::from(ident);
            let path = Path::from(segment);
            let expr_path = ExprPath {
                attrs: Vec::new(),
                qself: None,
                path,
            };
            *underscore = expr_path.clone().into();
            expr_path
        })
        .collect::<Vec<_>>();

    quote! {
        |#(#placeholders),*| { #call }
    }
    .into()
}
