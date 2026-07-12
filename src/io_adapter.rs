use std::fmt;

/// Adapts [`std::io::Write`] to [`std::fmt::Write`], allowing domlink
/// render methods to write directly to files, sockets, or any other I/O sink
/// without buffering the entire page in memory.
///
/// # Example
///
/// Render into an in-memory buffer:
///
/// ```rust
/// use std::io::BufWriter;
/// use domlink::{init, Tags, IoWriteAdapter};
///
/// let page = init(Tags::Div).text("hello");
/// let mut buf = BufWriter::new(Vec::new());
/// let mut adapter = IoWriteAdapter(buf);
/// page.render_into(&mut adapter).unwrap();
/// ```
///
/// Render directly into a file, useful for server-side rendering
/// without buffering the entire page in memory:
///
/// ```no_run
/// use std::fs::File;
/// use domlink::{init, Tags, IoWriteAdapter};
///
/// let page = init(Tags::Div).text("hello");
/// let file = File::create("out.html").unwrap();
/// let mut adapter = IoWriteAdapter(std::io::BufWriter::new(file));
/// page.render_into(&mut adapter).unwrap();
/// ```
#[allow(dead_code)]
pub struct IoWriteAdapter<W: std::io::Write>(pub W);

impl<W: std::io::Write> fmt::Write for IoWriteAdapter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}
