use domlink::{SafeHtml, Tpl, TplArg};

fn main() {
    let row_tpl = Tpl::new(
        "<tr><td>{}</td></tr>"
    );

    let mut rows = String::new();

    for i in 1..=3 {
        row_tpl.render_into(
            &mut rows,
            &[&i.to_string()],
        );
    }

    let rows = SafeHtml::new_unchecked(rows);

    let page_tpl = Tpl::new(
        "<table>{}</table>"
    );

    let html = page_tpl.render_mixed(&[
        TplArg::Html(&rows)
    ]);

    println!("{html}");
}
