use domlink::{init, Tags};

fn main() {
    let page = init(Tags::Html);

    page.head()
        .title()
        .text("Domlink Example");

    let body = page.body();

    body.h1()
        .text("Hello Domlink");

    body.p()
        .text("Fast runtime HTML rendering.");

    println!("{}", page.render_pretty());
}
