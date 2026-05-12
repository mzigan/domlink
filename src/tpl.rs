//! Runtime HTML template engine.
//!
//! This module provides a lightweight runtime HTML templating system.
//!
//! Unlike compile-time template engines such as Askama or Maud,
//! templates here can be created dynamically during program execution.
//!
//! The engine is optimized for:
//!
//! - runtime-generated templates,
//! - SSR hot paths,
//! - repeated fragment rendering,
//! - dynamic CMS layouts,
//! - database-stored templates,
//! - low allocation overhead.
//!
//! # Escaping
//!
//! By default, inserted text is HTML-escaped automatically.
//!
//! ```rust
//! use domlink::Tpl;
//!
//! let tpl = Tpl::new("<div>{}</div>");
//!
//! let html = tpl.render(&["<script>"]);
//!
//! assert_eq!(html, "<div>&lt;script&gt;</div>");
//! ```
//!
//! # Trusted HTML
//!
//! Trusted pre-rendered HTML fragments can be inserted without
//! additional escaping via [`SafeHtml`] and [`TplArg::Html`].
//!
//! ```rust
//! use domlink::{SafeHtml, Tpl, TplArg};
//!
//! let rows = SafeHtml::new_unchecked(
//!     "<tr><td>1</td></tr>".to_string()
//! );
//!
//! let tpl = Tpl::new("<table>{}</table>");
//!
//! let html = tpl.render_mixed(&[
//!     TplArg::Html(&rows)
//! ]);
//! ```

use crate::escape::escape_into_string;

/// Trusted HTML fragment.
///
/// `SafeHtml` represents HTML that is already escaped or generated
/// by trusted code and therefore may be inserted without additional
/// escaping.
///
/// This type is useful for:
///
/// - pre-rendered DOM fragments,
/// - nested templates,
/// - SSR rendering,
/// - avoiding repeated escaping passes,
/// - caching rendered HTML.
///
/// # Warning
///
/// `SafeHtml` does not validate or sanitize HTML.
///
/// You must guarantee that the content is trusted.
///
/// # Example
///
/// ```rust
/// use domlink::SafeHtml;
///
/// let html = SafeHtml::new_unchecked(
///     "<div>safe</div>".to_string()
/// );
///
/// assert_eq!(html.as_str(), "<div>safe</div>");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeHtml(String);

impl SafeHtml {
    /// Creates a trusted HTML fragment without escaping or validation.
    ///
    /// # Warning
    ///
    /// The provided HTML will be inserted into templates as-is.
    ///
    /// Never use untrusted user input here.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::SafeHtml;
    ///
    /// let html = SafeHtml::new_unchecked(
    ///     "<span>Hello</span>".to_string()
    /// );
    /// ```
    pub fn new_unchecked(html: String) -> Self {
        Self(html)
    }

    /// Consumes the fragment and returns the inner string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::SafeHtml;
    ///
    /// let html = SafeHtml::new_unchecked("<b>x</b>".to_string());
    ///
    /// let s = html.into_string();
    /// ```
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns the inner HTML string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::SafeHtml;
    ///
    /// let html = SafeHtml::new_unchecked("<b>x</b>".to_string());
    ///
    /// assert_eq!(html.as_str(), "<b>x</b>");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SafeHtml {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<SafeHtml> for String {
    fn from(value: SafeHtml) -> Self {
        value.0
    }
}

/// Template rendering argument.
///
/// Controls whether inserted content should be escaped.
///
/// # Variants
///
/// - [`TplArg::Text`] — escaped text,
/// - [`TplArg::Html`] — trusted raw HTML.
///
/// # Example
///
/// ```rust
/// use domlink::{Tpl, TplArg};
///
/// let tpl = Tpl::new("<div>{}</div>");
///
/// let html = tpl.render_mixed(&[
///     TplArg::Text("<unsafe>")
/// ]);
///
/// assert_eq!(
///     html,
///     "<div>&lt;unsafe&gt;</div>"
/// );
/// ```
pub enum TplArg<'a> {
    /// Escaped text.
    ///
    /// HTML special characters are escaped automatically.
    Text(&'a str),

    /// Trusted raw HTML.
    ///
    /// Inserted without escaping.
    ///
    /// # Warning
    ///
    /// Do not pass untrusted input.
    Html(&'a SafeHtml),
}

/// Runtime HTML template.
///
/// `Tpl` parses a template once and reuses the parsed structure
/// for repeated rendering.
///
/// Placeholders are represented by `{}`.
///
/// # Example
///
/// ```rust
/// use domlink::Tpl;
///
/// let tpl = Tpl::new("<li>{}</li>");
///
/// let html = tpl.render(&["Alice"]);
///
/// assert_eq!(html, "<li>Alice</li>");
/// ```
///
/// # Reuse
///
/// Templates are designed to be reused:
///
/// ```rust
/// use domlink::Tpl;
///
/// let tpl = Tpl::new("<li>{}</li>");
///
/// let mut out = String::new();
///
/// tpl.render_into(&mut out, &["A"]);
/// tpl.render_into(&mut out, &["B"]);
/// ```
pub struct Tpl {
    subs: Vec<String>,
}

#[allow(dead_code)]
impl Tpl {
    /// Parses a template string.
    ///
    /// The template is split into static fragments around `{}` placeholders.
    ///
    /// Parsing occurs only once and the prepared structure can then
    /// be reused efficiently.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::Tpl;
    ///
    /// let tpl = Tpl::new("<div>{}</div>");
    /// ```
    pub fn new(input: &str) -> Tpl {
        let mut subs = Vec::new();
        let mut last_end = 0;

        for (start, _) in input.match_indices("{}") {
            subs.push(input[last_end..start].to_owned());
            last_end = start + 2;
        }

        subs.push(input[last_end..].to_owned());

        Tpl { subs }
    }

    /// Renders escaped text and returns trusted HTML.
    ///
    /// The resulting HTML is considered safe because all inserted
    /// values are escaped automatically.
    ///
    /// Useful for composing larger templates without repeated escaping.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::Tpl;
    ///
    /// let tpl = Tpl::new("<td>{}</td>");
    ///
    /// let html = tpl.render_safe_html(&["Alice"]);
    /// ```
    pub fn render_safe_html(&self, text: &[&str]) -> SafeHtml {
        let mut out = String::new();
        self.render_into(&mut out, text);
        SafeHtml(out)
    }

    /// Renders escaped text directly into an existing buffer.
    ///
    /// Useful for minimizing allocations in hot rendering paths.
    pub fn render_safe_html_into(&self, out: &mut String, text: &[&str]) {
        self.render_into(out, text);
    }

    /// Renders mixed escaped text and trusted HTML into a new string.
    ///
    /// `TplArg::Text` values are escaped.
    ///
    /// `TplArg::Html` fragments are inserted directly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::{SafeHtml, Tpl, TplArg};
    ///
    /// let rows = SafeHtml::new_unchecked(
    ///     "<tr><td>1</td></tr>".to_string()
    /// );
    ///
    /// let tpl = Tpl::new("<table>{}</table>");
    ///
    /// let html = tpl.render_mixed(&[
    ///     TplArg::Html(&rows)
    /// ]);
    /// ```
    pub fn render_mixed(&self, args: &[TplArg<'_>]) -> String {
        let mut out = String::new();
        self.render_mixed_into(&mut out, args);
        out
    }

    /// Renders mixed escaped/raw arguments into an existing buffer.
    ///
    /// This is the most flexible rendering API.
    pub fn render_mixed_into(&self, out: &mut String, args: &[TplArg<'_>]) {
        for (n, s) in self.subs.iter().enumerate() {
            out.push_str(s);

            if n < args.len() && n < self.subs.len() - 1 {
                match args[n] {
                    TplArg::Text(text) => escape_into_string(out, text),
                    TplArg::Html(html) => out.push_str(html.as_str()),
                }
            }
        }
    }

    /// Renders mixed content and returns trusted HTML.
    ///
    /// Useful for composing nested runtime templates efficiently.
    pub fn render_mixed_safe_html(&self, args: &[TplArg<'_>]) -> SafeHtml {
        let mut out = String::new();
        self.render_mixed_into(&mut out, args);
        SafeHtml(out)
    }

    /// Renders escaped text arguments into a new string.
    ///
    /// All inserted values are HTML-escaped automatically.
    ///
    /// # Example
    ///
    /// ```rust
    /// use domlink::Tpl;
    ///
    /// let tpl = Tpl::new("<div>{}</div>");
    ///
    /// let html = tpl.render(&["<unsafe>"]);
    ///
    /// assert_eq!(
    ///     html,
    ///     "<div>&lt;unsafe&gt;</div>"
    /// );
    /// ```
    pub fn render(&self, text: &[&str]) -> String {
        let len = self.subs.iter().map(|s| s.len()).sum::<usize>()
            + text.iter().map(|s| s.len()).sum::<usize>();

        let mut res = String::with_capacity(len);

        for (n, s) in self.subs.iter().enumerate() {
            res.push_str(s);

            if n < text.len() && n < self.subs.len() - 1 {
                escape_into_string(&mut res, text[n]);
            }
        }

        res
    }

    /// Renders raw unescaped text into a new string.
    ///
    /// # Warning
    ///
    /// This method bypasses HTML escaping.
    ///
    /// Never pass untrusted user input here.
    ///
    /// Prefer [`Tpl::render`] whenever possible.
    pub fn render_raw(&self, text: &[&str]) -> String {
        let len = self.subs.iter().map(|s| s.len()).sum::<usize>()
            + text.iter().map(|s| s.len()).sum::<usize>();

        let mut res = String::with_capacity(len);

        for (n, s) in self.subs.iter().enumerate() {
            res.push_str(s);

            if n < text.len() && n < self.subs.len() - 1 {
                res.push_str(text[n]);
            }
        }

        res
    }

    /// Renders escaped text into an existing buffer.
    ///
    /// Avoids allocating a new string for each render.
    pub fn render_into(&self, out: &mut String, text: &[&str]) {
        for (n, s) in self.subs.iter().enumerate() {
            out.push_str(s);

            if n < text.len() && n < self.subs.len() - 1 {
                escape_into_string(out, text[n]);
            }
        }
    }

    /// Renders raw unescaped text into an existing buffer.
    ///
    /// # Warning
    ///
    /// This method bypasses HTML escaping.
    ///
    /// Never pass untrusted user input here.
    pub fn render_raw_into(&self, out: &mut String, text: &[&str]) {
        for (n, s) in self.subs.iter().enumerate() {
            out.push_str(s);

            if n < text.len() && n < self.subs.len() - 1 {
                out.push_str(text[n]);
            }
        }
    }
}
