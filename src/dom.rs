use std::fmt;

use crate::{Tags, escape::{escape_into_string, write_escaped}};

//----------------------------------------------------------------------------------------
// Dom
//----------------------------------------------------------------------------------------
#[derive(Debug, Default)]
pub struct Dom {
    pub(super) vec: Vec<Element>,
}

impl Dom {
    pub fn new() -> Dom {
        Dom::default()
    }

    pub fn get(&self, index: usize) -> Option<&Element> {
        self.vec.get(index)
    }

    pub(super) fn get_mut(&mut self, index: usize) -> Option<&mut Element> {
        self.vec.get_mut(index)
    }

    pub(super) fn push(&mut self, el: Element) -> usize {
        let index = self.vec.len();
        self.vec.push(el);
        index
    }
}

//----------------------------------------------------------------------------------------
// Element
//----------------------------------------------------------------------------------------
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Element {
    index: usize,
    parent: Option<usize>,
    pub(super) tag: Tags,
    attrs: Vec<(String, String)>,
    raw_attrs: Vec<String>,
    text: String,
    raw_html: String,
    pub(super) childs: Vec<usize>,
}

fn is_valid_attr_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ':' | '.'))
}

impl Element {
    pub fn new(index: usize, parent: Option<usize>, tag: Tags) -> Element {
        Element {
            index,
            parent,
            tag,
            ..Default::default()
        }
    }

    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self {
        assert!(
            is_valid_attr_name(name),
            "invalid attribute name: '{}'",
            name
        );
        let mut escaped_val = String::with_capacity(value.len());
        escape_into_string(&mut escaped_val, value);
        self.attrs.push((name.to_owned(), escaped_val));
        self
    }

    pub fn id(&mut self, value: &str) -> &mut Self {
        self.attr("id", value)
    }

    pub fn name(&mut self, value: &str) -> &mut Self {
        self.attr("name", value)
    }

    pub fn value(&mut self, value: &str) -> &mut Self {
        self.attr("value", value)
    }

    pub fn data(&mut self, key: &str, value: &str) -> &mut Self {
        assert!(!key.is_empty(), "data attribute key cannot be empty");
        assert!(
            is_valid_attr_name(key),
            "invalid data attribute key: '{}'",
            key
        );
        self.attr(&format!("data-{}", key), value)
    }

    // Классы склеиваются, если вызвать метод несколько раз
    pub fn class(&mut self, value: &str) -> &mut Self {
        if let Some(existing) = self.attrs.iter_mut().find(|(k, _)| k == "class") {
            existing.1.push(' ');
            escape_into_string(&mut existing.1, value);
        } else {
            self.attr("class", value);
        }
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text.push_str(text);
        self
    }

    pub fn tpl(&mut self) -> &mut Self {
        self.text("{}")
    }

    // --- Опасные BUILDER'ы ---

    /// Adds a raw, unescaped attribute string to the element.
    ///
    /// # Warning
    ///
    /// This method bypasses the library's built-in XSS protection. The string
    /// will be injected into the HTML as-is. Only use this if you are absolutely
    /// sure the input is safe and properly formatted (e.g., for Alpine.js or HTMX attributes).
    ///
    /// It is highly recommended to use double quotes for attribute values to comply
    /// with HTML standards, e.g., `raw_attr("x-data=\"{ open: false }\"")`.
    pub fn raw_attr(&mut self, attr_str: &str) -> &mut Self {
        self.raw_attrs.push(attr_str.to_owned());
        self
    }

    /// Adds raw, unescaped HTML content inside the element.
    ///
    /// # Warning
    ///
    /// This method does **not** escape the provided HTML. Using it with untrusted
    /// user input will lead to XSS vulnerabilities. Use this only for trusted
    /// markup or JavaScript/CSS code inside `<script>` or `<style>` tags.
    pub fn raw_html(&mut self, html: &str) -> &mut Self {
        self.raw_html.push_str(html);
        self
    }

    // --- RENDER ---

    pub(super) fn render_pretty<W: fmt::Write>(&self, dom: &Dom, depth: usize, out: &mut W) -> fmt::Result {
        if self.tag == Tags::Any {
            for &child_idx in &self.childs {
                if let Some(child) = dom.get(child_idx) {
                    child.render_pretty(dom, depth, out)?;
                }
            }
            return Ok(());
        }

        if self.tag == Tags::Html {
            out.write_str("<!DOCTYPE html>")?;
            out.write_char('\n')?;
        }

        write_indent(out, depth)?;

        out.write_str(self.tag.opening_tag())?;

        for (name, value) in &self.attrs {
            write!(out, " {}=\"{}\"", name, value)?;
        }

        for raw in &self.raw_attrs {
            write!(out, " {}", raw)?;
        }

        out.write_char('>')?;

        if self.tag.is_void() {
            out.write_char('\n')?;
            return Ok(());
        }

        if self.tag == Tags::Textarea {
            write_escaped(out, &self.text)?;
            if let Some(closing) = self.tag.closing_tag() {
                out.write_str(closing)?;
            }
            out.write_char('\n')?;
            return Ok(());
        }

        out.write_char('\n')?;

        if !self.text.is_empty() {
            for line in self.text.lines() {
                write_indent(out, depth + 1)?;
                write_escaped(out, line)?;
                out.write_char('\n')?;
            }
        }

        if !self.raw_html.is_empty() {
            out.write_str(&self.raw_html)?;
            out.write_char('\n')?;
        }

        for &child_idx in &self.childs {
            if let Some(child) = dom.get(child_idx) {
                child.render_pretty(dom, depth + 1, out)?;
            }
        }

        if let Some(closing) = self.tag.closing_tag() {
            write_indent(out, depth)?;

            out.write_str(closing)?;
            out.write_char('\n')?;
        }

        Ok(())
    }

    pub(super) fn render_compact<W: fmt::Write>(&self, dom: &Dom, out: &mut W) -> fmt::Result {
        if self.tag == Tags::Any {
            for &child_idx in &self.childs {
                if let Some(child) = dom.get(child_idx) {
                    child.render_compact(dom, out)?;
                }
            }
            return Ok(());
        }

        if self.tag == Tags::Html {
            out.write_str("<!DOCTYPE html>")?;
        }

        out.write_str(self.tag.opening_tag())?;

        for (name, value) in &self.attrs {
            write!(out, " {}=\"{}\"", name, value)?;
        }

        for raw in &self.raw_attrs {
            write!(out, " {}", raw)?;
        }

        out.write_char('>')?;

        if self.tag.is_void() {
            return Ok(());
        }

        if self.tag == Tags::Textarea {
            write_escaped(out, &self.text)?;
            if let Some(closing) = self.tag.closing_tag() {
                out.write_str(closing)?;
            }
            return Ok(());
        }

        if !self.text.is_empty() {
            write_escaped(out, &self.text)?;
        }

        if !self.raw_html.is_empty() {
            out.write_str(&self.raw_html)?;
        }

        for &child_idx in &self.childs {
            if let Some(child) = dom.get(child_idx) {
                child.render_compact(dom, out)?;
            }
        }

        if let Some(closing) = self.tag.closing_tag() {
            out.write_str(closing)?;
        }

        Ok(())
    }
}

fn write_indent<W: fmt::Write>(out: &mut W, depth: usize) -> fmt::Result {
    for _ in 0..depth * 2 {
        out.write_char(' ')?;
    }
    Ok(())
}
