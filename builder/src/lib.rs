//use std::fmt::Error;
//use proc_macro2::{TokenStream};
use quote::{format_ident, quote};
use syn;
use syn::{DataStruct, DeriveInput, FieldsNamed};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    // Parse the input tokens into a syntax tree.
    let input: DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let _name = input.ident;

    let _builder_name = format_ident!("{}Builder", _name);

    let _data: FieldsNamed = match input.data {
        syn::Data::Struct(
            DataStruct {
                fields: syn::Fields::Named(named_fields),
                ..
            }
        ) => named_fields,

        //other=> unimplemented!("{:?}", other),
        //other=> panic!("unimplemented"),
        _ => panic!("unimplemented"),
    };

    let _fields = _data.named.iter().filter_map(|field| {
        let _type = &field.ty;
        match &field.ident {
            Some(ident) => Some((ident, _type)),
            _ => None
        }
    });

    let _names = _data.named.iter().filter_map(|field| {
        match field.ident.as_ref() {
            None => None,
            Some(ident) => Some(ident)
        }
    });

    let _initialize_fields = _names.clone().map(|field_name| {
        quote! {
            #field_name: None
        }
    });

    // builder struct fields
    let _builder_struct_fields = _fields.clone().map(|(field_name, field_type)| {
        quote! {
            #field_name: Option<#field_type>
        }
    });

    // Setter methods
    let _setter_methods = _fields.clone().map(|(field_name, field_type)| {
        quote! {
            pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = Some(#field_name);
                self
            }
        }
    });

    let expanded = quote! {
        pub struct #_builder_name {
            #(
                #_builder_struct_fields,
            )*
        }

        impl #_builder_name {
            fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {
                if self.executable.is_none() { return Err(format!("executable is none").into()); }
                if self.args.is_none() { return Err(format!("args is none").into()); }
                if self.env.is_none() { return Err(format!("env is none").into()); }
                if self.current_dir.is_none() { return Err(format!("current_dir is none").into()); }

                let _command = Command {
                    executable: self.executable.as_ref().unwrap().to_owned(),
                    args: self.args.as_ref().unwrap().clone(),
                    env: self.env.as_ref().unwrap().clone(),
                    current_dir: self.current_dir.as_ref().unwrap().clone(),
                };

                return Ok(_command);
            }

            // Setter methods
            #(
                #_setter_methods
            )*
        }

        // The generated impl.
        impl #_name {
            pub fn builder() -> #_builder_name {
                let _cmd_builder = #_builder_name {
                    #(
                        #_initialize_fields,
                    )*
                };

                return _cmd_builder;
            }
        }
    };

    return proc_macro::TokenStream::from(expanded);
}
