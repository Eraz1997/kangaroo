use darling::ast::NestedMeta;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, Ident, ItemFn, Lit, Meta, ReturnType, Type, parse2};

use crate::{FunctionItem, models::Arguments};

pub fn parse_arguments(attributes: proc_macro2::TokenStream) -> Result<Arguments, darling::Error> {
    let attribute_arguments: Vec<NestedMeta> = match NestedMeta::parse_meta_list(attributes) {
        Ok(arguments) => arguments,
        Err(error) => {
            return Err(darling::Error::from(error));
        }
    };

    let mut default_file: Option<String> = None;
    let mut custom_files: Vec<(Ident, String)> = vec![];

    for meta in &attribute_arguments {
        let NestedMeta::Meta(Meta::NameValue(nested_value)) = meta else {
            continue;
        };

        let ident = nested_value.path.get_ident().unwrap().to_string();
        let value = match &nested_value.value {
            Expr::Lit(expression) => match &expression.lit {
                Lit::Str(string) => string.value(),
                _ => return Err(darling::Error::custom("expected string literal")),
            },
            _ => return Err(darling::Error::custom("expected string literal")),
        };

        if ident == "file" {
            default_file = Some(value);
        } else {
            let status_code_ident = Ident::new(&ident.to_uppercase(), Span::call_site());
            custom_files.push((status_code_ident, value));
        }
    }

    Ok(Arguments {
        default_file,
        custom_files,
    })
}

pub fn parse_function(item: proc_macro2::TokenStream) -> Result<FunctionItem, darling::Error> {
    let function: ItemFn = match parse2::<ItemFn>(item) {
        Ok(function) => function,
        Err(error) => {
            return Err(darling::Error::from(error));
        }
    };

    let attrs = function.attrs.clone();
    let vis = function.vis.clone();
    let sig = function.sig.clone();
    let inputs = sig.inputs;
    let ident = sig.ident.clone();
    let asyncness = sig.asyncness;
    let generics = sig.generics.clone();
    let where_clause = sig.generics.where_clause.clone();
    let return_type_definition = match &sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, return_type) => Some(return_type),
    };
    let is_result_return_type =
        if let Some(Type::Path(type_path)) = return_type_definition.map(Box::as_ref) {
            type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "Result")
        } else {
            false
        };
    let return_type = match return_type_definition {
        Some(return_type) => quote!(#return_type),
        None => quote!(()),
    };
    let block = function.block.clone();

    Ok(FunctionItem {
        attrs,
        vis,
        inputs,
        ident,
        asyncness,
        generics,
        where_clause,
        is_result_return_type,
        return_type,
        block,
    })
}
