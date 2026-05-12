use domlink::Tpl;

fn main() {
    let tpl = Tpl::new("<div>{}</div>");

    let html = tpl.render_raw(&[
        "<b>Hello</b>"
    ]);

    println!("{html}");
}
