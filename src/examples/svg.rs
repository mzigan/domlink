use domlink::{init, Tags};

fn main() {
    let svg = init(Tags::Svg)
        .attr("width", "100")
        .attr("height", "100");

    svg.path()
        .attr("d", "M10 10 H 90 V 90 H 10 Z")
        .attr("stroke", "black")
        .attr("fill", "transparent");

    println!("{}", svg.render_pretty());
}
