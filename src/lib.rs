use std::{
    cell::RefCell,
    fmt::{self},
    rc::Rc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Pretty,
    Compact,
}

//----------------------------------------------------------------------------------------
// ESCAPE (XSS Protection)
//----------------------------------------------------------------------------------------
// здесь unwrap нельзя использовать, нужно всегда обрабатывать ошибку
fn write_escaped<W: fmt::Write>(out: &mut W, s: &str) -> fmt::Result {
    let bytes = s.as_bytes();
    let mut last = 0;

    for (i, &b) in bytes.iter().enumerate() {
        let escaped = match b {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => continue,
        };

        if last < i {
            out.write_str(&s[last..i])?;
        }

        out.write_str(escaped)?;
        last = i + 1;
    }

    if last < s.len() {
        out.write_str(&s[last..])?;
    }

    Ok(())
}

// для String write_str никогда не возвращает Err, это гарантировано стандартной библиотекой
// поэтому здесь смело используеи unwrap
fn escape_into_string(out: &mut String, s: &str) {
    write_escaped(out, s).unwrap()
}
//----------------------------------------------------------------------------------------
// TAGS
//----------------------------------------------------------------------------------------
#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Tags {
    Html,
    Head,
    Body,
    Svg,
    Path,
    #[default]
    Div,
    Table,
    Thead,
    Tbody,
    Tr,
    Td,
    Form,
    Iframe,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Ol,
    Ul,
    Li,
    Br,
    Span,
    Img,
    A,
    Button,
    Input,
    Textarea,
    Select,
    Opt, // Переименован из Option
    Meta,
    Label,
    Title,
    Link,
    Script,
    Style,
    Any,
}

impl Tags {
    fn opening_tag(&self) -> &'static str {
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

    fn closing_tag(&self) -> Option<&'static str> {
        match self {
            // Void-элементы и фрагменты не имеют закрывающего тега
            Tags::Meta | Tags::Link | Tags::Img | Tags::Br | Tags::Input | Tags::Any => None,

            // Остальные теги явно перечислены
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

    fn is_void(&self) -> bool {
        matches!(
            self,
            Tags::Meta | Tags::Link | Tags::Img | Tags::Br | Tags::Input
        )
    }
}

//----------------------------------------------------------------------------------------
// Dom
//----------------------------------------------------------------------------------------
#[derive(Debug, Default)]
pub struct Dom {
    vec: Vec<Element>,
}

impl Dom {
    pub fn new() -> Dom {
        Dom::default()
    }

    pub fn get(&self, index: usize) -> Option<&Element> {
        self.vec.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Element> {
        self.vec.get_mut(index)
    }

    fn push(&mut self, el: Element) -> usize {
        let index = self.vec.len();
        self.vec.push(el);
        index
    }
}

//----------------------------------------------------------------------------------------
// Tpl
//----------------------------------------------------------------------------------------
pub struct Tpl {
    subs: Vec<String>,
    // static_len: usize,
}

#[allow(dead_code)]
impl Tpl {
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

    pub fn render_into(&self, out: &mut String, text: &[&str]) {
        for (n, s) in self.subs.iter().enumerate() {
            out.push_str(s);

            if n < text.len() && n < self.subs.len() - 1 {
                escape_into_string(out, text[n]);
            }
        }
    }

    pub fn render_raw_into(&self, out: &mut String, text: &[&str]) {
        for (n, s) in self.subs.iter().enumerate() {
            out.push_str(s);

            if n < text.len() && n < self.subs.len() - 1 {
                out.push_str(text[n]);
            }
        }
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
    tag: Tags,
    attrs: Vec<(String, String)>,
    raw_attrs: Vec<String>,
    text: String,
    raw_html: String,
    childs: Vec<usize>,
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

    fn render_pretty<W: fmt::Write>(&self, dom: &Dom, depth: usize, out: &mut W) -> fmt::Result {
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

    fn render_compact<W: fmt::Write>(&self, dom: &Dom, out: &mut W) -> fmt::Result {
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

// Макрос для шорткатов тегов
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

//----------------------------------------------------------------------------------------
// INIT
//----------------------------------------------------------------------------------------
pub fn init(tag: Tags) -> Link {
    let dom = Rc::new(RefCell::new(Dom::new()));
    Link::new(dom, None, tag) // Родителя нет (None)
}

/// Adapts [`std::io::Write`] to [`std::fmt::Write`], allowing [`Link::render_into`]
/// to write directly to files, sockets, or any other I/O sink without buffering
/// the entire page in memory.
///
/// # Example
/// ```rust
/// use std::io::BufWriter;
/// use domlink::{init, Tags, IoWriteAdapter};
///
/// let page = init(Tags::Div).text("hello");
/// let mut buf = BufWriter::new(Vec::new());
/// let mut adapter = IoWriteAdapter(buf);
/// page.render_into(&mut adapter).unwrap();
/// ```
/// Полезно для серверного рендера прямо в HTTP-ответ без буферизации всей страницы в памяти.
/// let file = std::fs::File::create("out.html").unwrap();
/// let mut adapter = IoWriteAdapter(std::io::BufWriter::new(file));
/// el.render(&dom, 0, &mut adapter).unwrap();
#[allow(dead_code)]
pub struct IoWriteAdapter<W: std::io::Write>(pub W);

impl<W: std::io::Write> fmt::Write for IoWriteAdapter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}
