use std::{collections::HashMap, path::MAIN_SEPARATOR_STR};

use axum::http::StatusCode;

#[derive(Clone)]
pub struct KangarooConfig {
    static_files_folder_path: String,
    default_document_path: String,
    custom_document_paths: HashMap<StatusCode, String>,
}

impl KangarooConfig {
    pub fn new(static_files_folder_path: &str) -> Self {
        Self {
            static_files_folder_path: static_files_folder_path.to_string(),
            default_document_path: "index.html".to_string(),
            custom_document_paths: HashMap::new(),
        }
    }

    pub fn with_default_document(mut self, relative_path: &str) -> Self {
        self.default_document_path = relative_path.to_string();
        self
    }

    pub fn with_document_for_status(
        mut self,
        status_code: StatusCode,
        relative_path: &str,
    ) -> Self {
        self.custom_document_paths
            .insert(status_code, relative_path.to_string());
        self
    }

    pub fn get_full_path_from_status_code(&self, status_code: StatusCode) -> Option<String> {
        self.custom_document_paths
            .get(&status_code)
            .map(|relative_path| {
                format!(
                    "{}{}{}",
                    self.static_files_folder_path, MAIN_SEPARATOR_STR, relative_path
                )
            })
    }

    pub fn get_default_path(&self) -> String {
        format!(
            "{}{}{}",
            self.static_files_folder_path, MAIN_SEPARATOR_STR, self.default_document_path
        )
    }

    pub fn get_full_path_from_relative(&self, relative_path: &str) -> String {
        format!(
            "{}{}{}",
            self.static_files_folder_path, MAIN_SEPARATOR_STR, relative_path
        )
    }

    pub fn get_folder_path(&self) -> String {
        self.static_files_folder_path.clone()
    }
}
