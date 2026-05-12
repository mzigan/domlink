use domlink::{init, Tags};

fn main() {
    let page = init(Tags::Div)
        .class("container")
        .text("Hello");

    println!("{}", page.render_compact());
}
