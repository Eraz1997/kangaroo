pub use config::KangarooConfig;
pub use kangaroo_macros::kangarooise;
pub use router::KangarooRouterExtension;
pub use traits::IntoKangarooError;

mod config;
mod router;
mod traits;

pub mod exports {
    pub mod axum {
        pub use axum::Extension;

        pub mod body {
            pub use axum::body::Body;
        }

        pub mod http {
            pub use axum::http::StatusCode;
        }

        pub mod response {
            pub use axum::response::Response;
        }
    }

    pub mod regex {
        pub use regex::Regex;
    }

    pub mod serde_json {
        pub use serde_json::to_string;
    }

    pub mod tokio {
        pub mod fs {
            pub use tokio::fs::read_to_string;
        }
    }
}
