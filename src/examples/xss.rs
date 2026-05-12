use domlink::{init, Tags};

fn main() {
    let page = init(Tags::Div)
        .text("<script>alert(1)</script>");

    println!("{}", page.render_compact());
}
