use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, parse_macro_input, Error, Expr, Ident, LitStr, Token, Type};

struct Handler {
    method: Ident,
    path: Option<LitStr>,
    callback: Expr,
}

struct Controller {
    id: Ident,
    base: LitStr,
    handler_type: Type,
    handlers: Vec<Handler>,
}

impl Parse for Controller {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let id: Ident = input.parse()?;
        let _: Token![@] = input
            .parse()
            .map_err(|_| Error::new(id.span(), "Missing '@'."))?;
        let base: LitStr = input.parse()?;
        let _: Token![:] = input
            .parse()
            .map_err(|_| Error::new(base.span(), "Missing ':'."))?;
        let handler_type: Type = input.parse()?;
        let _: Token![;] = input
            .parse()
            .map_err(|_| Error::new(base.span(), "Missing ';'."))?;
        let mut vec = Vec::new();
        while input.peek(Ident) {
            let method: Ident = input.parse()?;
            let path: Option<LitStr> = if input.peek(LitStr) {
                let path: LitStr = input.parse()?;
                Some(path)
            } else {
                None
            };
            let _: Token![=] = input
                .parse()
                .map_err(|_| Error::new(match path.clone() {
                    Some(path) => path.span(),
                    None => method.span()
                }, "Missing '='."))?;
            let callback: Expr = input.parse()?;
            let _: Token![;] = input
                .parse()
                .map_err(|_| Error::new(method.span(), "Missing ';'."))?;
            vec.push(Handler {
                method,
                path,
                callback,
            });
        }
        let handlers = vec;
        Ok(Controller { id, handler_type, base, handlers })
    }
}

impl ToTokens for Controller {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.id;
        let base = &self.base;
        let handler_type = &self.handler_type;
        let handlers = self
            .handlers
            .iter()
            .map(|x| {
                let method = match x.method.to_string().as_str() {
                    "GET" => quote::quote!(get),
                    "POST" => quote::quote!(post),
                    "PUT" => quote::quote!(put),
                    "DELETE" => quote::quote!(delete),
                    _ => quote::quote!(),
                };
                let path = &x.path;
                let callback = &x.callback;
                quote::quote!(((String::from(concat!(#base, #path))), #method(#callback)),)
            })
            .fold(quote::quote!(), |a, b| quote::quote!(#a #b));
        tokens.extend(quote::quote! {
            pub fn #id() -> Vec<#handler_type> {
                vec![#handlers]
            }
        });
    }
}

#[proc_macro]
pub fn controller_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Controller);
    quote::quote! {
        #input
    }
    .into()
}
