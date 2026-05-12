use std::fmt;

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
/// Useful for server-side rendering directly into an HTTP response
/// without buffering the entire page in memory.
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
