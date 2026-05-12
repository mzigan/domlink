use domlink::{Tags, init};

fn main() {
    let root = init(Tags::Html);
    let body = root.body();

    body.div().tpl();
    body.div().tpl();

    let t = root.template();    

    println!("{}", t.render(&["John & Doe", "<script>"]));

    println!("{}", t.render_escaped(&["John & Doe", "<script>"]));
}