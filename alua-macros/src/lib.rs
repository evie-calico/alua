use darling::{ast, FromDeriveInput, FromField};
use quote::quote;
use syn::*;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(alua), supports(struct_named))]
struct ClassAnnotationArgs {
    data: ast::Data<(), ClassAnnotationFieldArgs>,

    #[darling(default)]
    functions: Vec<LitStr>,
}

#[derive(Debug, FromField)]
#[darling(attributes(alua), forward_attrs(doc))]
struct ClassAnnotationFieldArgs {
    ident: Option<syn::Ident>,
    attrs: Vec<syn::Attribute>,
    ty: syn::Type,

    #[darling(default)]
    skip: bool,
    as_lua: Option<LitStr>,
}

#[proc_macro_derive(ClassAnnotation, attributes(alua))]
pub fn environment(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let args = match ClassAnnotationArgs::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let name = input.ident;
    let arg_fields = args.data.as_ref().take_struct().unwrap().fields;
    let fields = arg_fields.iter().filter(|x| !x.skip);
    let identifiers = fields.clone().map(|x| &x.ident);
    let types = fields.clone().map(|x| {
        if let Some(as_lua) = &x.as_lua {
            quote!(#as_lua)
        } else {
            let ty = &x.ty;
            quote!(<#ty as ::alua::TypeAnnotation>::lua_type())
        }
    });
    let docs = fields.clone().map(|x| {
        let mut documentation = String::new();
        for i in &x.attrs {
            if let Meta::NameValue(i) = &i.meta {
                if let Some(ident) = i.path.get_ident() {
                    if ident.to_string() == "doc" {
                        if let Expr::Lit(ExprLit {
                            attrs: _,
                            lit: Lit::Str(s),
                        }) = &i.value
                        {
                            documentation += &s.value();
                        }
                    }
                }
            }
        }
        documentation
    });
    let methods = args.functions;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl ::alua::TypeAnnotation for #name {
            fn lua_type() -> String {
                stringify!(#name).into()
            }
        }
        impl ::alua::ClassAnnotation for #name {
            fn class_annotation() -> String {
                use ::std::fmt::Write;
                let mut out = String::new();
                let _ = writeln!(out,"--- @class {}", stringify!(#name));
                #(
                    let _ = writeln!(out,"--- @field {} {} -{}", stringify!(#identifiers), #types, #docs);
                )*
                #(
                    let _ = writeln!(out, "--- @method {}", #methods);
                )*
                out
            }
        }
    };

    // Hand the output tokens back to the compiler
    expanded.into()
}
