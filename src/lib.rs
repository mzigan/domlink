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

pub fn init(tag: Tags) -> Link {
    let dom = Rc::new(RefCell::new(Dom::new()));
    Link::new(dom, None, tag)
}
