use domlink::{init, Tags};

fn main() {
    let button = init(Tags::Button)
        .attr("hx-post", "/clicked")
        .attr("hx-target", "#result")
        .text("Click");

    println!("{}", button.render_compact());
}
