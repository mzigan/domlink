//! HTML tag definitions used by domlink.
//!
//! [`Tags`] defines all built-in HTML and SVG tags supported by the DOM builder.
//!
//! Most users do not interact with this enum directly except when creating a
//! root element through [`crate::init`].
//!
//! # Examples
//!
//! ```rust
//! use domlink::{init, Tags};
//!
//! let page = init(Tags::Html);
//! ```
//!
//! Child elements are usually created through shortcut methods instead of using
//! [`Tags`] manually:
//!
//! ```rust
//! use domlink::{init, Tags};
//!
//! let root = init(Tags::Div);
//!
//! root.span().text("hello");
//! root.table().tbody().tr().td().text("cell");
//! ```

use crate::link::Link;

/// Supported HTML and SVG tags.
///
/// This enum is used internally by the renderer and externally when creating
/// root elements with [`crate::init`].
///
/// Most tags have corresponding shortcut methods on [`crate::Link`]:
///
/// ```rust
/// use domlink::{init, Tags};
///
/// let page = init(Tags::Html);
///
/// page.body()
///     .div()
///     .class("container")
///     .text("Hello");
/// ```
///
/// # Fragment Rendering
///
/// [`Tags::Any`] is a special fragment tag that renders only its children
/// without generating a wrapping HTML element.
///
/// # Void Elements
///
/// Some tags are void elements and cannot contain children:
///
/// - `Meta`
/// - `Link`
/// - `Img`
/// - `Br`
/// - `Input`
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Tags {
    /// `<html>`
    Html,

    /// `<head>`
    Head,

    /// `<body>`
    Body,

    /// `<svg>`
    Svg,

    /// `<path>`
    Path,

    /// `<div>`
    #[default]
    Div,

    /// `<table>`
    Table,

    /// `<thead>`
    Thead,

    /// `<tbody>`
    Tbody,

    /// `<tr>`
    Tr,

    /// `<td>`
    Td,

    /// `<form>`
    Form,

    /// `<iframe>`
    Iframe,

    /// `<p>`
    P,

    /// `<h1>`
    H1,

    /// `<h2>`
    H2,

    /// `<h3>`
    H3,

    /// `<h4>`
    H4,

    /// `<h5>`
    H5,

    /// `<h6>`
    H6,

    /// `<ol>`
    Ol,

    /// `<ul>`
    Ul,

    /// `<li>`
    Li,

    /// `<br>`
    ///
    /// Void element.
    Br,

    /// `<span>`
    Span,

    /// `<img>`
    ///
    /// Void element.
    Img,

    /// `<a>`
    A,

    /// `<button>`
    Button,

    /// `<input>`
    ///
    /// Void element.
    Input,

    /// `<textarea>`
    Textarea,

    /// `<select>`
    Select,

    /// `<option>`
    Opt,

    /// `<meta>`
    ///
    /// Void element.
    Meta,

    /// `<label>`
    Label,

    /// `<title>`
    Title,

    /// `<link>`
    ///
    /// Void element.
    Link,

    /// `<script>`
    Script,

    /// `<style>`
    Style,

    /// Fragment node without wrapping tag.
    ///
    /// `Any` renders only its children and does not produce an HTML element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let frag = init(Tags::Any);
    ///
    /// frag.span().text("A");
    /// frag.span().text("B");
    ///
    /// let html = frag.render_compact();
    ///
    /// assert_eq!(html, "<span>A</span><span>B</span>");
    /// ```
    Any,
}

impl Tags {
    /// Returns the opening HTML tag representation.
    ///
    /// This method is used internally by the renderer.
    ///
    /// For example:
    ///
    /// - `Tags::Div` → `"<div"`
    /// - `Tags::Span` → `"<span"`
    ///
    /// Some tags include built-in attributes:
    ///
    /// - `Svg` includes XML namespaces
    /// - `Img` includes an empty `alt` attribute
    pub(crate) fn opening_tag(&self) -> &'static str {
        match self {
            Tags::Html => "<html",
            Tags::Head => "<head",
            Tags::Meta => "<meta",
            Tags::Link => "<link",
            Tags::Body => "<body",
            Tags::Svg => {
                "<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'"
            }
            Tags::Path => "<path",
            Tags::Div => "<div",
            Tags::Form => "<form",
            Tags::Table => "<table",
            Tags::Tbody => "<tbody",
            Tags::Thead => "<thead",
            Tags::Tr => "<tr",
            Tags::Td => "<td",
            Tags::Iframe => "<iframe",
            Tags::Ol => "<ol",
            Tags::Ul => "<ul",
            Tags::Li => "<li",
            Tags::Span => "<span",
            Tags::Img => "<img alt=''",
            Tags::Br => "<br",
            Tags::A => "<a",
            Tags::P => "<p",
            Tags::H1 => "<h1",
            Tags::H2 => "<h2",
            Tags::H3 => "<h3",
            Tags::H4 => "<h4",
            Tags::H5 => "<h5",
            Tags::H6 => "<h6",
            Tags::Label => "<label",
            Tags::Button => "<button",
            Tags::Input => "<input",
            Tags::Textarea => "<textarea",
            Tags::Select => "<select",
            Tags::Opt => "<option",
            Tags::Title => "<title",
            Tags::Script => "<script",
            Tags::Style => "<style",
            Tags::Any => "",
        }
    }

    /// Returns the closing HTML tag representation.
    ///
    /// Void elements and [`Tags::Any`] return `None`.
    ///
    /// This method is used internally by the renderer.
    pub(crate) fn closing_tag(&self) -> Option<&'static str> {
        match self {
            Tags::Meta | Tags::Link | Tags::Img | Tags::Br | Tags::Input | Tags::Any => None,

            Tags::Html => Some("</html>"),
            Tags::Head => Some("</head>"),
            Tags::Body => Some("</body>"),
            Tags::Svg => Some("</svg>"),
            Tags::Path => Some("</path>"),
            Tags::Div => Some("</div>"),
            Tags::Form => Some("</form>"),
            Tags::Table => Some("</table>"),
            Tags::Tbody => Some("</tbody>"),
            Tags::Thead => Some("</thead>"),
            Tags::Tr => Some("</tr>"),
            Tags::Td => Some("</td>"),
            Tags::Iframe => Some("</iframe>"),
            Tags::Ol => Some("</ol>"),
            Tags::Ul => Some("</ul>"),
            Tags::Li => Some("</li>"),
            Tags::Span => Some("</span>"),
            Tags::A => Some("</a>"),
            Tags::P => Some("</p>"),
            Tags::H1 => Some("</h1>"),
            Tags::H2 => Some("</h2>"),
            Tags::H3 => Some("</h3>"),
            Tags::H4 => Some("</h4>"),
            Tags::H5 => Some("</h5>"),
            Tags::H6 => Some("</h6>"),
            Tags::Label => Some("</label>"),
            Tags::Button => Some("</button>"),
            Tags::Textarea => Some("</textarea>"),
            Tags::Select => Some("</select>"),
            Tags::Opt => Some("</option>"),
            Tags::Title => Some("</title>"),
            Tags::Script => Some("</script>"),
            Tags::Style => Some("</style>"),
        }
    }

    /// Returns `true` if the tag is a void element.
    ///
    /// Void elements cannot contain children and do not have closing tags.
    ///
    /// # Void Elements
    ///
    /// - `<meta>`
    /// - `<link>`
    /// - `<img>`
    /// - `<br>`
    /// - `<input>`
    ///
    /// This method is used internally by the renderer and builder validation.
    pub(crate) fn is_void(&self) -> bool {
        matches!(
            self,
            Tags::Meta | Tags::Link | Tags::Img | Tags::Br | Tags::Input
        )
    }
}

macro_rules! impl_tag_shortcuts {
    ($($method:ident => $tag:expr),* $(,)?) => {
        #[allow(dead_code)]
        impl Link {
            $(
                pub fn $method(&self) -> Link {
                    self.append($tag)
                }
            )*
        }
    };
}

impl_tag_shortcuts! {
    html => Tags::Html,
    head => Tags::Head,
    title => Tags::Title,
    meta => Tags::Meta,
    link => Tags::Link,
    body => Tags::Body,
    div => Tags::Div,
    h1 => Tags::H1,
    h2 => Tags::H2,
    h3 => Tags::H3,
    h4 => Tags::H4,
    h5 => Tags::H5,
    h6 => Tags::H6,
    p => Tags::P,
    span => Tags::Span,
    a => Tags::A,
    button => Tags::Button,
    form => Tags::Form,
    input => Tags::Input,
    textarea => Tags::Textarea,
    select => Tags::Select,
    opt => Tags::Opt,
    table => Tags::Table,
    thead => Tags::Thead,
    tbody => Tags::Tbody,
    tr => Tags::Tr,
    td => Tags::Td,
    ul => Tags::Ul,
    ol => Tags::Ol,
    li => Tags::Li,
    img => Tags::Img,
    svg => Tags::Svg,
    path => Tags::Path,
    script => Tags::Script,
    style => Tags::Style,
    br => Tags::Br,
    label => Tags::Label,
    iframe => Tags::Iframe,
    any => Tags::Any,
}
