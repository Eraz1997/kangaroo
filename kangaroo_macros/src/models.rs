use syn::{
    Attribute, Block, FnArg, Generics, Ident, Visibility, WhereClause,
    punctuated::Punctuated,
    token::{Async, Comma},
};

pub struct Arguments {
    pub default_file: Option<String>,
    pub custom_files: Vec<(Ident, String)>,
}

pub struct FunctionItem {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub inputs: Punctuated<FnArg, Comma>,
    pub ident: Ident,
    pub asyncness: Option<Async>,
    pub generics: Generics,
    pub where_clause: Option<WhereClause>,
    pub is_result_return_type: bool,
    pub return_type: proc_macro2::TokenStream,
    pub block: Box<Block>,
}
