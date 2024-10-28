use darling::{ast, FromDeriveInput, FromField, FromVariant};

use quote::quote;
use syn::*;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(alua), supports(struct_named, enum_unit))]
struct ClassAnnotationArgs {
    data: ast::Data<ClassAnnotationVariantArgs, ClassAnnotationFieldArgs>,

    #[darling(default)]
    fields: Vec<LitStr>,
    #[cfg(feature = "userdata")]
    #[darling(default, multiple)]
    method: Vec<Ident>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(alua))]
struct ClassAnnotationVariantArgs {
    ident: syn::Ident,

    #[darling(default)]
    skip: bool,
    as_lua: Option<LitStr>,
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

    #[cfg_attr(not(feature = "userdata"), allow(unused))]
    #[darling(default)]
    get: bool,
    #[cfg_attr(not(feature = "userdata"), allow(unused))]
    #[darling(default)]
    set: bool,
}

#[cfg(feature = "userdata")]
#[proc_macro_derive(UserData, attributes(alua))]
pub fn userdata(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let args = match ClassAnnotationArgs::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let name = input.ident;
    let args_struct = args.data.as_ref().take_struct().unwrap();
    let arg_fields = args_struct.fields;
    let fields = arg_fields.iter().filter(|x| !x.skip);
    let methods = args.method;

    let getters = fields.clone().filter(|x| x.get).map(|x| &x.ident);
    let getter_methods = fields.clone().filter(|x| x.get).map(|x| {
        let ident = &x.ident;
        let expected_type = x.as_lua.as_ref().map(|x| x.value());
        let expected_type = expected_type.as_ref().map(|x| x.as_str());
        match expected_type {
            Some("string") => quote!(this.#ident.to_string()),
            Some("string?") => quote!(this.#ident.map(|x| x.to_string())),
            _ => quote!(this.#ident.clone()),
        }
    });

    let setters = fields.clone().filter(|x| x.set).map(|x| &x.ident);

    let expanded = quote! {
        impl ::mlua::UserData for #name {
            fn add_fields<F: ::mlua::UserDataFields<Self>>(fields: &mut F) {
                #(
                    fields.add_field_method_get(stringify!(#getters), |_, this| Ok(#getter_methods));
                )*
                #(
                    fields.add_field_method_set(stringify!(#setters), |_, this, value| {
                        this.#setters = value;
                        Ok(())
                    });
                )*
            }
            fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
                #(
                    methods.add_method_mut(stringify!(#methods), #methods);
                )*
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(ClassAnnotation, attributes(alua))]
pub fn class_annotation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let args = match ClassAnnotationArgs::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let name = input.ident;
    match args.data.as_ref() {
        ast::Data::Struct(arg_fields) => {
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
            let manual_fields = args.fields;

            // Build the output, possibly using quasi-quotation
            let expanded = quote! {
                impl ::alua::TypeAnnotation for #name {
                    fn lua_type() -> ::std::borrow::Cow<'static, str> {
                        ::std::borrow::Cow::Borrowed(stringify!(#name))
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
                            let _ = writeln!(out, "--- @field {}", #manual_fields);
                        )*
                        out
                    }
                }
            };

            expanded.into()
        }
        ast::Data::Enum(variants) => {
            let mut variant_names = variants.iter().filter(|x| !x.skip).map(|x| {
                if let Some(as_lua) = &x.as_lua {
                    quote!(#as_lua)
                } else {
                    let ident = &x.ident;
                    quote!(#ident)
                }
            });
            let first_variant = variant_names.next().unwrap();
            let expanded = quote! {
                impl ::alua::TypeAnnotation for #name {
                    fn lua_type() -> ::std::borrow::Cow<'static, str> {
                        ::std::borrow::Cow::Borrowed(stringify!(#name))
                    }
                }
                impl ::alua::ClassAnnotation for #name {
                    fn class_annotation() -> String {
                        use ::std::fmt::Write;
                        let mut out = String::new();
                        let _ = write!(out, "--- @alias {} \"{}\"", stringify!(#name), stringify!(#first_variant));
                        #(
                            let _ = write!(out, " | \"{}\"", stringify!(#variant_names));
                        )*
                        let _ = writeln!(out);
                        out
                    }
                }
            };

            expanded.into()
        }
    }
}
