use domlink::Tpl;

fn main() {
    let tpl = Tpl::new(
        "<div class=\"user\">{}</div>"
    );

    let html = tpl.render(&[
        "<script>alert(1)</script>"
    ]);

    println!("{html}");
}
