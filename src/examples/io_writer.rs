use std::fs::File;
use std::io::BufWriter;

use domlink::{
    init,
    IoWriteAdapter,
    Tags,
};

fn main() -> std::fmt::Result {
    let page = init(Tags::Html);

    page.body()
        .h1()
        .text("Streaming Example");

    let file = File::create("example.html").unwrap();

    let writer = BufWriter::new(file);

    let mut adapter = IoWriteAdapter(writer);

    page.render_into(&mut adapter)?;

    Ok(())
}
