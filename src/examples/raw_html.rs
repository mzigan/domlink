use domlink::{init, Tags};

fn main() {
    let page = init(Tags::Div)
        .raw_html("<b>Hello</b>");

    println!("{}", page.render_compact());
}
