use crate::escape::escape_into_string;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeHtml(String);

impl SafeHtml {
    pub fn new_unchecked(html: String) -> Self {
        Self(html)
    }

    pub fn into_string(self) -> String {
        self.0
    }

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

pub enum TplArg<'a> {
    Text(&'a str),
    Html(&'a SafeHtml),
}

pub struct Tpl {
    subs: Vec<String>,
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

    pub fn render_safe_html(&self, text: &[&str]) -> SafeHtml {
        let mut out = String::new();
        self.render_into(&mut out, text);
        SafeHtml(out)
    }

    pub fn render_safe_html_into(&self, out: &mut String, text: &[&str]) {
        self.render_into(out, text);
    }

    pub fn render_mixed(&self, args: &[TplArg<'_>]) -> String {
        let mut out = String::new();
        self.render_mixed_into(&mut out, args);
        out
    }

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

    pub fn render_mixed_safe_html(&self, args: &[TplArg<'_>]) -> SafeHtml {
        let mut out = String::new();
        self.render_mixed_into(&mut out, args);
        SafeHtml(out)
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
