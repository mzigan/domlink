use domlink::{init, Tags};

fn main() {
    let app = init(Tags::Div)
        .raw_attr(r#"x-data="{ open: false }""#);

    app.button()
        .raw_attr(r#"@click="open = !open""#)
        .text("Toggle");

    app.div()
        .raw_attr(r#"x-show="open""#)
        .text("Visible");

    println!("{}", app.render_pretty());
}
