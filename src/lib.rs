/*!
Kangaroos carry puppies in their pouches. `kangaroo` carries server-side data in static web applications, served by Axum.

This Rust crate allows you to serve static web applications and inject server-side data through Axum, making it available to the client as JSON objects. Kangaroo aims to avoid having heavy server runtimes serving apps through full SSR, but still be able to manipulate response status codes and reduce network round trips.

## Usage 🪿

### Basics 🍍

To easily get started, add the following in your existing Axum app as a bare minimum setup:

```ignore
// Imports 👇
use kangaroo_axum::{IntoKangarooError, KangarooConfig, KangarooRouterExtension, kangarooise};

// ...

// App extension 👇
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_home))
        // ...
        .with_kangaroo(KangarooConfig::new("public/"));
    //...
}

// ...

// Define your serialisable data model 👇
#[derive(Serialize)]
struct HomeData {
    pub current_username: Option<String>,
}

// ...

// Kangarooise endpoint 👇
#[kangarooise]
async fn get_home() -> HomeData {
    // ...
}
```

And in your frontend JavaScript code, add the following to retrieve the server-side injected data:

```js
const data = JSON.parse(document.getElementById("kangaroo-data").innerText);
```

Requests to `/` will return the content of `index.html` with an injected JSON script with ID `kangaroo-data` containing your server-side data.

### Error Handling 👎

Kangaroo can gracefully handle errors raised by your endpoints:

```ignore
#[kangarooise]
async fn get_home() -> Result<HomeData, Error> {
    // ...
}
```

You just need to implement the `IntoKangarooError` to make it handle gracefully. The only member must return status code and a payload to inject as `kangaroo-data` JSON script.

```ignore
impl IntoKangarooError for Error {
    fn into_kangaroo_error(self) -> (StatusCode, impl Serialize) {
        // ...
    }
}
```

The frontend can read the serialised error payload from the `kangaroo-data` JSON script as usual.

### Custom Paths 👟

Kangaroo supports some layers of customisation to pick the correct HTML document based on route and status code. The default document path is `index.html`, relative to the folder path passed to the `KangarooConfig` object.

To set another default:

```ignore
KangarooConfig::new("public/")
    .with_default_document("default.html")
```

You can set specific document paths for each HTTP response status code coming from your `Error` objects:

```ignore
use axum::http::StatusCode;

KangarooConfig::new("public/")
    .with_document_for_status(StatusCode::NOT_FOUND, "404.html")
    .with_document_for_status(StatusCode::INTERNAL_SERVER_ERROR, "5xx.html")
```

You can also set overrides for each route:

```ignore
#[kangarooise(
    file = "home/index.html"
    not_found = "home/404.html"
    internal_server_error = "home/5xx.html"
)]
async fn get_home() -> Result<HomeData, Error> {
    // ...
}
```

`file` is the default path override for the route, the other arguments must be valid lowercased variants of the members of `axum::http::StatusCode`.

The precedence order for the document to load is the following, from top to bottom priority:

1. Route-specific override for the returning status code
    ```ignore
    #[kangarooise(not_found = "404.html")]
    ```
2. Status code-specific default path
    ```ignore
    KangarooConfig::new("public/")
        .with_document_for_status(StatusCode::NOT_FOUND, "404.html")
    ```
3. Route-specific override for the default document
    ```ignore
    #[kangarooise(file = "home.html")]
    ```
4. Custom default document
    ```ignore
    KangarooConfig::new("public/")
        .with_default_document("home.html")
    ```
5. `index.html`
*/

pub use config::KangarooConfig;
pub use kangaroo_axum_macros::kangarooise;
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
