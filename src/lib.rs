//! Fast and safe runtime HTML rendering for Rust.
//!
//! Domlink provides two complementary ways to generate HTML:
//!
//! - a safe DOM builder,
//! - a lightweight runtime template renderer.
//!
//! # Example
//!
//! ```rust
//! use domlink::{init, Tags};
//!
//! let page = init(Tags::Html);
//!
//! page.body()
//!     .div()
//!     .class("container")
//!     .text("Hello");
//!
//! let html = page.render_compact();
//! ```
//!
//! # Runtime templates
//!
//! ```rust
//! use domlink::Tpl;
//!
//! let tpl = Tpl::new("<div>{}</div>");
//! let html = tpl.render(&["Hello"]);
//! ```
//!
//! # Safety
//!
//! Text and attribute values are escaped by default.
//! Raw HTML APIs are explicit and should only be used with trusted input.

mod dom;
mod escape;
mod io_adapter;
mod link;
mod tags;
mod tpl;

pub use io_adapter::IoWriteAdapter;
pub use link::Link;
pub use tags::Tags;
pub use tpl::{SafeHtml, Tpl, TplArg};

use std::{cell::RefCell, rc::Rc};

use crate::dom::Dom;

/// Creates a new DOM tree with the given root tag.
///
/// This is the main entry point for the DOM builder API.
///
/// # Example
///
/// ```rust
/// use domlink::{init, Tags};
///
/// let root = init(Tags::Div)
///     .id("app")
///     .class("container")
///     .text("Hello");
///
/// assert_eq!(
///     root.render_compact(),
///     r#"<div id="app" class="container">Hello</div>"#
/// );
/// ```
pub fn init(tag: Tags) -> Link {
    let dom = Rc::new(RefCell::new(Dom::new()));
    Link::new(dom, None, tag)
}
