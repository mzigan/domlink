//! Public DOM handle used to build and render HTML trees.
//!
//! `Link` is the main user-facing builder type in domlink. It acts as a handle
//! to an element stored inside the internal DOM arena.
//!
//! Most methods consume and return `Self`, which allows ergonomic chaining:
//!
//! ```rust
//! use domlink::{init, Tags};
//!
//! let page = init(Tags::Div)
//!     .id("app")
//!     .class("container")
//!     .text("Hello");
//! ```
//!
//! Child elements are created through tag shortcut methods such as `.div()`,
//! `.span()`, `.table()`, `.tr()`, `.td()`, and others.

use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    Tags,
    dom::{Dom, Element},
    tpl::Tpl,
};

/// A handle to an element inside a domlink DOM tree.
///
/// `Link` is the primary builder object used by the public API. It does not
/// store the element directly. Instead, it points to an element inside the
/// internal [`Dom`] arena.
///
/// Cloning a `Link` is cheap: it clones the internal `Rc` handle and keeps the
/// same element index.
///
/// # Examples
///
/// ```rust
/// use domlink::{init, Tags};
///
/// let root = init(Tags::Html);
///
/// root.body()
///     .div()
///     .id("app")
///     .class("container")
///     .text("Hello");
///
/// let html = root.render_compact();
/// assert!(html.contains("Hello"));
/// ```
#[derive(Debug, Clone)]
pub struct Link {
    dom: Rc<RefCell<Dom>>,
    index: usize,
}

#[allow(dead_code)]
impl Link {
    /// Creates a new element inside the internal DOM arena.
    ///
    /// This method is primarily used internally by domlink. Most users should
    /// start with [`crate::init`] and then create child elements through tag
    /// shortcut methods such as `.div()`, `.span()`, `.table()`, and so on.
    ///
    /// If `parent` is provided and points to an existing element, the new element
    /// is registered as a child of that parent.
    pub fn new(dom: Rc<RefCell<Dom>>, parent: Option<usize>, tag: Tags) -> Link {
        let index = {
            let mut d = dom.borrow_mut();
            let idx = d.vec.len();
            let el = Element::new(idx, parent, tag);
            d.push(el);
            idx
        };

        if let Some(p_idx) = parent {
            let mut d = dom.borrow_mut();
            if let Some(parent_el) = d.get_mut(p_idx) {
                parent_el.childs.push(index);
            }
        }

        Link { dom, index }
    }

    /// Appends a child element with the given tag and returns a [`Link`] to it.
    ///
    /// Usually this method is used indirectly through generated tag shortcut
    /// methods:
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let root = init(Tags::Div);
    /// root.append(Tags::Span).text("Hello");
    ///
    /// // Equivalent:
    /// root.span().text("Hello");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the current element is a void element, such as `<input>`,
    /// `<br>`, `<img>`, `<meta>`, or `<link>`.
    ///
    /// Void elements cannot have children by definition.
    pub fn append(&self, tag: Tags) -> Link {
        {
            let d = self.dom.borrow();
            if d.get(self.index).is_some_and(|el| el.tag.is_void()) {
                panic!("Cannot append child <{:?}> to void element", tag);
            }
        }

        Link::new(self.dom.clone(), Some(self.index), tag)
    }

    // --- SAFE METHODS ---

    /// Adds a safe HTML attribute.
    ///
    /// The attribute value is HTML-escaped automatically.
    ///
    /// # Panics
    ///
    /// Panics if the attribute name is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let el = init(Tags::Input)
    ///     .attr("type", "text")
    ///     .attr("placeholder", "<name>");
    ///
    /// let html = el.render_compact();
    /// assert!(html.contains("placeholder=\"&lt;name&gt;\""));
    /// ```
    pub fn attr(self, name: &str, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.attr(name, value);
        }
        self
    }

    /// Sets the `id` attribute.
    ///
    /// The value is HTML-escaped automatically.
    pub fn id(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.id(value);
        }
        self
    }

    /// Adds a CSS class.
    ///
    /// Calling this method multiple times merges classes into a single
    /// `class` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let el = init(Tags::Div)
    ///     .class("container")
    ///     .class("dark");
    ///
    /// assert!(el.render_compact().contains("class=\"container dark\""));
    /// ```
    pub fn class(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.class(value);
        }
        self
    }

    /// Sets the `name` attribute.
    ///
    /// The value is HTML-escaped automatically.
    pub fn name(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.name(value);
        }
        self
    }

    /// Sets the `value` attribute.
    ///
    /// The value is HTML-escaped automatically.
    pub fn value(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.value(value);
        }
        self
    }

    /// Adds a `data-*` attribute.
    ///
    /// The key is validated and the value is HTML-escaped automatically.
    ///
    /// # Panics
    ///
    /// Panics if the key is empty or contains invalid attribute-name characters.
    pub fn data(self, key: &str, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.data(key, value);
        }
        self
    }

    /// Adds escaped text content to the current element.
    ///
    /// Text is escaped during rendering, so user input is safe by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let el = init(Tags::Div).text("<script>alert(1)</script>");
    /// let html = el.render_compact();
    ///
    /// assert!(!html.contains("<script>"));
    /// assert!(html.contains("&lt;script&gt;"));
    /// ```
    pub fn text(self, text: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.text(text);
        }
        self
    }

    /// Adds a `{}` placeholder to the current element.
    ///
    /// This is useful when building a DOM tree first and converting it into a
    /// [`Tpl`] with [`Link::template`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let row = init(Tags::Tr);
    /// row.td().tpl();
    /// row.td().tpl();
    ///
    /// let tpl = row.template();
    /// let html = tpl.render(&["Alice", "alice@example.com"]);
    /// ```
    pub fn tpl(self) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.tpl();
        }
        self
    }

    // --- UNSAFE METHODS ---

    /// Adds a raw, unescaped attribute string.
    ///
    /// This method bypasses attribute escaping and validation.
    ///
    /// Use it only for trusted attributes that cannot be conveniently expressed
    /// through [`Link::attr`], for example Alpine.js or HTMX attributes.
    ///
    /// # Warning
    ///
    /// Do not pass untrusted user input to this method. It can create XSS
    /// vulnerabilities or invalid HTML.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let el = init(Tags::Div)
    ///     .raw_attr(r#"x-data="{ open: false }""#);
    ///
    /// assert!(el.render_compact().contains(r#"x-data="{ open: false }""#));
    /// ```
    pub fn raw_attr(self, attr_str: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.raw_attr(attr_str);
        }
        self
    }

    /// Adds raw, unescaped HTML inside the current element.
    ///
    /// This method bypasses domlink's XSS protection.
    ///
    /// Use it only for trusted HTML, JavaScript, or CSS.
    ///
    /// # Warning
    ///
    /// Do not pass user input to this method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use domlink::{init, Tags};
    ///
    /// let script = init(Tags::Script)
    ///     .raw_html("console.log('ready');");
    ///
    /// assert!(script.render_compact().contains("console.log('ready');"));
    /// ```
    pub fn raw_html(self, html: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.raw_html(html);
        }
        self
    }

    // --- RENDER ---

    /// Renders the current element and its children as formatted HTML.
    ///
    /// The result is written into any [`fmt::Write`] sink.
    ///
    /// Pretty rendering includes indentation and newlines.
    pub fn render_pretty_into<W: fmt::Write>(&self, out: &mut W) -> fmt::Result {
        let d = self.dom.borrow();
        if let Some(el) = d.get(self.index) {
            el.render_pretty(&d, 0, out)?;
        }
        Ok(())
    }

    /// Renders the current element and its children as compact HTML.
    ///
    /// The result is written into any [`fmt::Write`] sink.
    ///
    /// Compact rendering omits indentation and unnecessary newlines.
    pub fn render_compact_into<W: fmt::Write>(&self, out: &mut W) -> fmt::Result {
        let d = self.dom.borrow();
        if let Some(el) = d.get(self.index) {
            el.render_compact(&d, out)?;
        }
        Ok(())
    }

    /// Renders the current element and its children as formatted HTML.
    ///
    /// Returns a newly allocated [`String`].
    pub fn render_pretty(&self) -> String {
        let mut out = String::new();
        self.render_pretty_into(&mut out).unwrap();
        out
    }

    /// Renders the current element and its children as compact HTML.
    ///
    /// Returns a newly allocated [`String`].
    pub fn render_compact(&self) -> String {
        let mut out = String::new();
        self.render_compact_into(&mut out).unwrap();
        out
    }

    /// Renders the current element into a [`fmt::Write`] sink.
    ///
    /// This is equivalent to [`Link::render_pretty_into`].
    pub fn render_into<W: fmt::Write>(&self, out: &mut W) -> fmt::Result {
        self.render_pretty_into(out)
    }

    /// Renders the current element as formatted HTML.
    ///
    /// This is equivalent to [`Link::render_pretty`].
    pub fn render(&self) -> String {
        self.render_pretty()
    }

    /// Converts the rendered HTML into a [`Tpl`].
    ///
    /// This is useful for building a structure once through the DOM builder and
    /// then rendering it many times with different values.
    ///
    /// Placeholders can be created with [`Link::tpl`].
    pub fn template(&self) -> Tpl {
        Tpl::new(&self.render())
    }
}

impl fmt::Display for Link {
    /// Formats this link as pretty-rendered HTML.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_pretty_into(f)
    }
}
