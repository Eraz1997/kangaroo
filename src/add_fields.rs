extern crate proc_macro;
use darling::{FromMeta, ast::NestedMeta};
use quote::quote;
use syn::{Ident, Meta, parse::ParseStream};

// Struct to define the macro's configuration for adding fields and
// optionally getters and setters
#[derive(FromMeta, Clone, Debug)]
struct AddFields {
    #[darling(multiple, rename = "fields")]
    add_fields: Vec<NewFieldDef>,
    #[darling(default)]
    getter: Option<IdentList>,
    #[darling(default)]
    setter: Option<IdentList>,
}

// Struct definition for new fields that will be added dynamically
#[derive(FromMeta, Clone, Debug)]
struct NewFieldDef {
    name: syn::Ident,
    ty: syn::Type,
}

// A struct to hold the parsed identifiers of getters and setters
#[derive(Debug, Clone)]
struct IdentList(Vec<Ident>);

// Implement `FromMeta` for `IdentList` to allow extraction
// from meta attributes
impl darling::FromMeta for IdentList {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        if let syn::Meta::List(meta_list) = item {
            let tokens: proc_macro2::TokenStream = meta_list.clone().tokens;
            let list: IdentList = syn::parse2::<IdentList>(tokens).expect("Parse IdentList");
            Ok(list)
        } else {
            Err(darling::Error::custom("Expected a list of identifiers"))
        }
    }
}

// Implementation of Parse trait for IdentList to enable custom parsing from token streams
impl syn::parse::Parse for IdentList {
    /// Parses input from a parse stream to construct an IdentList
    /// Panics if the input is empty or incorrect.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::<Ident>::new();

        if input.is_empty() {
            panic!("At least one field must be specified");
        }

        // Step 1: parse the "field" argument
        let field = input.parse::<syn::Ident>().expect("search field");
        if field.to_string().as_str() != "field" {
            panic!("wrong field name");
        }

        // Step 2: parse Token "="
        input.parse::<syn::Token![=]>()?;

        // Step 3: parse the field entries for setters and getters, separated by Token ','
        while !input.is_empty() {
            let field_entry: syn::Ident = if let Ok(field_entry) = input.parse::<syn::Ident>() {
                // parse identifier
                field_entry
            } else if let Ok(field_entry) = input.parse::<syn::LitStr>() {
                // parse String
                syn::Ident::new(field_entry.value().as_str(), proc_macro2::Span::call_site())
            } else {
                panic!("field_entry must be either a string literal or an identifier!");
            };

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
            entries.push(field_entry);
        }
        // Step 4: Return the filled IdentList
        Ok(IdentList(entries))
    }
}

// Main function to implement the macro logic
pub fn add_fields_impl(
    attributes: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    // Step 1: Parse input struct and handle parse errors
    let input: syn::ItemStruct = match syn::parse2::<syn::ItemStruct>(input) {
        Ok(is) => is,
        Err(e) => {
            return darling::Error::from(e).write_errors();
        }
    };

    // Step 2: Parse attributes using darling and handle errors
    // see https://github.com/TedDriggs/darling
    let attr_args: Vec<NestedMeta> = match NestedMeta::parse_meta_list(attributes) {
        Ok(v) => v,
        Err(e) => {
            return darling::Error::from(e).write_errors();
        }
    };
    let args: AddFields = match AddFields::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors();
        }
    };

    // Step 3: Retrieve the name of the structure
    let struct_name: &proc_macro2::Ident = &input.ident;
    let mut new_struct: syn::ItemStruct = input.clone();

    // Check if the struct has named fields
    match new_struct.fields {
        syn::Fields::Named(ref mut existing_fields) => {
            // This is a struct with named fields, proceed with macro logic
            for new_field_def in args.add_fields {
                let field_name: syn::Ident = new_field_def.name;
                let field_type: syn::Type = new_field_def.ty;
                let new_field: syn::Field = syn::parse_quote! {
                    #field_name : #field_type
                };
                existing_fields.named.push(new_field);
            }
        }
        syn::Fields::Unnamed(_) => {
            // This is a tuple struct, return an error
            return quote! {
                compile_error!("This macro only supports structs with named fields.");
            };
        }
        syn::Fields::Unit => {
            // This is a unit struct, return an error
            return quote! {
                compile_error!("This macro does not support unit structs.");
            };
        }
    };

    // Step 4: Process getter and setter logic
    let (getters, getter_errors) = process_getter_setter_fields(&args.getter, &new_struct, true);
    let (setters, setter_errors) = process_getter_setter_fields(&args.setter, &new_struct, false);

    // Step 5: Collect and process any errors encountered during field processing
    let all_errors: Vec<proc_macro2::TokenStream> = [getter_errors, setter_errors].concat();
    if !all_errors.is_empty() {
        return quote! { #(#all_errors)* };
    }

    // Step 6: Generate the modified struct and any additional trait implementations
    quote! {
        #new_struct
        impl #struct_name {
            #(#getters)*
            #(#setters)*
        }
    }
}

/// Processes specified fields for generating getters and setters based on the provided configuration.
///
/// This function takes a list of field identifiers from `IdentList` and the structure to modify, and determines
/// whether to create getters or setters based on the `is_getter` flag. It handles each field by checking the
/// structure's field definitions, generating the appropriate getter or setter methods, or returning errors if the fields
/// are not found.
///
/// # Parameters:
/// * `list_option`: Optional reference to an `IdentList` containing identifiers of fields to process.
///   This list defines which fields will have getters or setters generated.
/// * `new_struct`: Reference to the `syn::ItemStruct` that describes the entire structure being processed.
///   This is used to retrieve type information for each field.
/// * `is_getter`: Boolean flag indicating the type of methods to generate:
///    - If `true`, the function generates getter methods that return a reference to the field.
///    - If `false`, the function generates setter methods that take a value, set the field, and return a mutable reference to the struct.
///
/// # Returns:
/// A tuple containing two vectors of `proc_macro2::TokenStream`:
/// * The first vector contains the generated getter or setter methods for the fields. These are wrapped in `Ok` results.
/// * The second vector contains error token streams for any fields that could not be processed, typically because they are not found in the structure.
///   These errors are wrapped in `Err` results and generate compile-time error messages.
///
/// # Example Usage:
/// The function is typically called within a macro's implementation to dynamically add getters and setters
/// to a struct based on macro attributes. For instance:
///
/// ```ignore
/// let (getters, getter_errors) = process_getter_setter_fields(&some_config.getter, &parsed_struct, true);
/// let (setters, setter_errors) = process_getter_setter_fields(&some_config.setter, &parsed_struct, false);
/// ```
fn process_getter_setter_fields(
    list_option: &Option<IdentList>,
    new_struct: &syn::ItemStruct,
    is_getter: bool,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    // Step 1: Conditional Handling of Identifier List
    if let Some(setter_list) = list_option {
        // Step 2: Iterating Over Fields
        let results: Vec<Result<proc_macro2::TokenStream, proc_macro2::TokenStream>> = setter_list
            .0
            .iter()
            .map(|id| {
                let field_type = new_struct
                    .fields
                    .iter()
                    .find(|&field| field.ident.as_ref() == Some(id))
                    .map(|field| &field.ty);

                match field_type {
                    Some(ft) => {
                        // Step 3: Getter/Setter Generation
                        #[allow(clippy::bool_comparison)]
                        if is_getter == true {
                            Ok(quote! {
                                pub fn #id(&self) -> &#ft {
                                    &self.#id
                                }
                            })
                        } else {
                            let setter_name = syn::Ident::new(
                                &format!("set_{}", id),
                                proc_macro2::Span::call_site(),
                            );
                            Ok(quote! {
                                pub fn #setter_name(&mut self, value: #ft) -> &mut Self {
                                    self.#id=value;
                                    self
                                }
                            })
                        }
                    }

                    // Step 4: Error Handling
                    None => Err(quote! {
                        compile_error!("Setter Field not found in struct:", #id);
                    }),
                }
            })
            .collect();

        // Step 5: Split results into success and error token streams
        let (ok, err): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
        (
            ok.into_iter().filter_map(Result::ok).collect(),
            err.into_iter().filter_map(Result::err).collect(),
        )
    } else {
        (vec![], vec![])
    }
}
