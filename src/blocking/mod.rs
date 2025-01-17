//! A blocking Client API.
//!
//! The blocking `Client` will block the current thread to execute, instead
//! of returning futures that need to be executed on a runtime.
//!
//! Conversely, the functionality in `reqwestplus::blocking` must *not* be executed
//! within an async runtime, or it will panic when attempting to block. If
//! calling directly from an async function, consider using an async
//! [`reqwestplus::Client`][crate::Client] instead. If the immediate context is only
//! synchronous, but a transitive caller is async, consider changing that caller
//! to use [`tokio::task::spawn_blocking`] around the calls that need to block.
//!
//! # Optional
//!
//! This requires the optional `blocking` feature to be enabled.
//!
//! # Making a GET request
//!
//! For a single request, you can use the [`get`](get) shortcut method.
//!
//! ```rust
//! # use reqwestplus::{Error, Response};
//!
//! # fn run() -> Result<(), Error> {
//! let body = reqwestplus::blocking::get("https://www.rust-lang.org")?
//!     .text()?;
//!
//! println!("body = {:?}", body);
//! # Ok(())
//! # }
//! ```
//!
//! Additionally, the blocking [`Response`](Response) struct implements Rust's
//! `Read` trait, so many useful standard library and third party crates will
//! have convenience methods that take a `Response` anywhere `T: Read` is
//! acceptable.
//!
//! **NOTE**: If you plan to perform multiple requests, it is best to create a
//! [`Client`](Client) and reuse it, taking advantage of keep-alive connection
//! pooling.
//!
//! # Making POST requests (or setting request bodies)
//!
//! There are several ways you can set the body of a request. The basic one is
//! by using the `body()` method of a [`RequestBuilder`](RequestBuilder). This lets you set the
//! exact raw bytes of what the body should be. It accepts various types,
//! including `String`, `Vec<u8>`, and `File`. If you wish to pass a custom
//! Reader, you can use the `reqwestplus::blocking::Body::new()` constructor.
//!
//! ```rust
//! # use reqwestplus::Error;
//! #
//! # fn run() -> Result<(), Error> {
//! let client = reqwestplus::blocking::Client::new();
//! let res = client.post("http://httpbin.org/post")
//!     .body("the exact body that is sent")
//!     .send()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## And More
//!
//! Most features available to the asynchronous `Client` are also available,
//! on the blocking `Client`, see those docs for more.

mod body;
mod client;
#[cfg(feature = "multipart")]
pub mod multipart;
mod request;
mod response;
mod wait;

pub use self::body::Body;
pub use self::client::{Client, ClientBuilder};
pub use self::request::{Request, RequestBuilder};
pub use self::response::Response;

/// Shortcut method to quickly make a *blocking* `GET` request.
///
/// **NOTE**: This function creates a new internal `Client` on each call,
/// and so should not be used if making many requests. Create a
/// [`Client`](./struct.Client.html) instead.
///
/// # Examples
///
/// ```rust
/// # fn run() -> Result<(), reqwestplus::Error> {
/// let body = reqwestplus::blocking::get("https://www.rust-lang.org")?
///     .text()?;
/// # Ok(())
/// # }
/// # fn main() { }
/// ```
///
/// # Errors
///
/// This function fails if:
///
/// - native TLS backend cannot be initialized
/// - supplied `Url` cannot be parsed
/// - there was an error while sending request
/// - redirect loop was detected
/// - redirect limit was exhausted
pub fn get<T: crate::IntoUrl>(url: T) -> crate::Result<Response> {
    Client::builder().build()?.get(url).send()
}
