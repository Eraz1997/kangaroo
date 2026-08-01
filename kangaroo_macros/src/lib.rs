use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, parse_quote};

use crate::{
    models::FunctionItem,
    parsers::{parse_arguments, parse_function},
};

mod models;
mod parsers;

#[proc_macro_attribute]
pub fn kangarooise(attributes: TokenStream, item: TokenStream) -> TokenStream {
    kangarooise2(attributes.into(), item.into()).into()
}

fn kangarooise2(
    attributes: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let arguments = match parse_arguments(attributes) {
        Ok(arguments) => arguments,
        Err(error) => return error.write_errors(),
    };

    let FunctionItem {
        attrs,
        vis,
        mut inputs,
        ident,
        asyncness,
        generics,
        where_clause,
        is_result_return_type,
        return_type,
        block,
    } = match parse_function(item) {
        Ok(function) => function,
        Err(error) => return error.write_errors(),
    };

    let additional_input: FnArg = parse_quote! {
        kangaroo::exports::axum::Extension(config): kangaroo::exports::axum::Extension<kangaroo::KangarooConfig>
    };
    inputs.insert(0, additional_input);

    let result_handling_body = if is_result_return_type {
        quote! {
            let (status_code, kangaroo_data) = match kangaroo_data {
                Ok(kangaroo_data) => {
                    let kangaroo_data = kangaroo::exports::serde_json::to_string(&kangaroo_data);
                    (kangaroo::exports::axum::http::StatusCode::OK, kangaroo_data)
                },
                Err(error) => {
                    let (status_code, error_payload) = error.into_kangaroo_error();
                    let kangaroo_data = kangaroo::exports::serde_json::to_string(&error_payload);
                    (status_code, kangaroo_data)
                }
            };
        }
    } else {
        quote! {
            let status_code = kangaroo::exports::axum::http::StatusCode::OK;
            let kangaroo_data = kangaroo::exports::serde_json::to_string(&kangaroo_data);
        }
    };

    let custom_file_matches_body: Vec<proc_macro2::TokenStream> = arguments.custom_files.into_iter().map(|(status_code_ident, file)| quote! {
        kangaroo::exports::axum::http::StatusCode::#status_code_ident => config.get_full_path_from_relative(#file),
    }).collect();

    let default_html_path = match arguments.default_file {
        Some(default_file) => quote!(config.get_full_path_from_relative(#default_file)),
        None => quote!(config.get_default_path()),
    };

    quote! {
        #(#attrs)*
        #vis #asyncness fn #ident #generics (#inputs) -> kangaroo::exports::axum::response::Response #where_clause {
            let get_kangaroo_data = async move || {
                #block
            };

            let kangaroo_data: #return_type = get_kangaroo_data().await;

            #result_handling_body

            let json_data = match kangaroo_data {
                Ok(json_data) => json_data,
                Err(_) => return kangaroo::exports::axum::response::Response::builder()
                    .status(kangaroo::exports::axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "text/plain")
                    .body(kangaroo::exports::axum::body::Body::from("JSON serialization error"))
                    .unwrap(),
            };

            let injected_script = format!("<script type=\"application/json\" id=\"kangaroo-data\">{}</script>", json_data);

            let html_file_path = match status_code {
                #(#custom_file_matches_body)*
                _ => config.get_full_path_from_status_code(status_code).unwrap_or(#default_html_path),
            };

            let mut html_document = match kangaroo::exports::tokio::fs::read_to_string(html_file_path).await {
                Ok(html_document) => html_document,
                Err(_) => return kangaroo::exports::axum::response::Response::builder()
                    .status(kangaroo::exports::axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "text/plain")
                    .body(kangaroo::exports::axum::body::Body::from("error reading document"))
                    .unwrap(),
            };

            let re = match kangaroo::exports::regex::Regex::new(r"(?i)<head\b[^>]*>") {
                Ok(re) => re,
                Err(_) => return kangaroo::exports::axum::response::Response::builder()
                    .status(kangaroo::exports::axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "text/plain")
                    .body(kangaroo::exports::axum::body::Body::from("regex compilation error"))
                    .unwrap(),
            };

            if let Some(matching_group) = re.find(&html_document) {
                html_document.insert_str(matching_group.end(), &injected_script);
            } else {
                return kangaroo::exports::axum::response::Response::builder()
                    .status(kangaroo::exports::axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "text/plain")
                    .body(kangaroo::exports::axum::body::Body::from("invalid html document"))
                    .unwrap();
            }

            kangaroo::exports::axum::response::Response::builder()
                .status(status_code)
                .header("Content-Type", "text/html")
                .body(kangaroo::exports::axum::body::Body::from(html_document))
                .unwrap()
        }
    }
}
