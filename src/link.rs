use std::{cell::RefCell, fmt, rc::Rc};

use crate::{Tags, dom::{Dom, Element}, tpl::Tpl};

//----------------------------------------------------------------------------------------
// Link
//----------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Link {
    dom: Rc<RefCell<Dom>>,
    index: usize,
}

#[allow(dead_code)]
impl Link {

    pub fn new(dom: Rc<RefCell<Dom>>, parent: Option<usize>, tag: Tags) -> Link {
        let index = {
            let mut d = dom.borrow_mut();
            let idx = d.vec.len();
            let el = Element::new(idx, parent, tag);
            d.push(el);
            idx
        };

        // Регистрируем ребенка у родителя
        if let Some(p_idx) = parent {
            let mut d = dom.borrow_mut();
            // Убрали p_idx < index.
            // Если индекс родителя некорректен, get_mut просто вернет None,
            // и ребенок не добавится в список — это безопасно.
            if let Some(parent_el) = d.get_mut(p_idx) {
                parent_el.childs.push(index);
            }
        }

        Link { dom, index }
    }

    /// Panics if the current element is a void element (e.g. `<input>`, `<br>`, `<img>`),
    /// which cannot have children by definition.
    pub fn append(&self, tag: Tags) -> Link {
        {
            let d = self.dom.borrow();
            if d.get(self.index).is_some_and(|el| el.tag.is_void()) {
                panic!("Cannot append child <{:?}> to void element", tag);
            }
        }

        Link::new(self.dom.clone(), Some(self.index), tag)
    }

    // --- БЕЗОПАСНЫЕ МЕТОДЫ ---

    pub fn attr(self, name: &str, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.attr(name, value);
        }
        self
    }

    pub fn id(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.id(value);
        }
        self
    }

    pub fn class(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.class(value);
        }
        self
    }

    pub fn name(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.name(value);
        }
        self
    }

    pub fn value(self, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.value(value);
        }
        self
    }

    pub fn data(self, key: &str, value: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.data(key, value);
        }
        self
    }

    pub fn text(self, text: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.text(text);
        }
        self
    }

    pub fn tpl(self) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.tpl();
        }
        self
    }

    // --- ОПАСНЫЕ МЕТОДЫ ---

    pub fn raw_attr(self, attr_str: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.raw_attr(attr_str);
        }
        self
    }

    pub fn raw_html(self, html: &str) -> Self {
        if let Some(el) = self.dom.borrow_mut().get_mut(self.index) {
            el.raw_html(html);
        }
        self
    }

    // --- RENDER ---

    pub fn render_pretty_into<W: fmt::Write>(&self, out: &mut W) -> fmt::Result {
        let d = self.dom.borrow();
        if let Some(el) = d.get(self.index) {
            el.render_pretty(&d, 0, out)?;
        }
        Ok(())
    }

    pub fn render_compact_into<W: fmt::Write>(&self, out: &mut W) -> fmt::Result {
        let d = self.dom.borrow();
        if let Some(el) = d.get(self.index) {
            el.render_compact(&d, out)?;
        }
        Ok(())
    }

    pub fn render_pretty(&self) -> String {
        let mut out = String::new();
        self.render_pretty_into(&mut out).unwrap();
        out
    }

    pub fn render_compact(&self) -> String {
        let mut out = String::new();
        self.render_compact_into(&mut out).unwrap();
        out
    }

    pub fn render_into<W: fmt::Write>(&self, out: &mut W) -> fmt::Result {
        self.render_pretty_into(out)
    }

    // unwrap здесь безопасен по той же причине что и в escape_into_string
    // String: fmt::Write никогда не возвращает Err.
    pub fn render(&self) -> String {
        self.render_pretty()
    }

    pub fn template(&self) -> Tpl {
        Tpl::new(&self.render())
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render_pretty_into(f)
    }
}
