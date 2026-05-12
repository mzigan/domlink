use domlink::{Link, Tags};

fn user_card(parent: &Link, name: &str, email: &str) {
    let card = parent
        .div()
        .class("card");

    card.h2().text(name);

    card.p().text(email);
}

fn main() {
    let page = domlink::init(Tags::Div);

    user_card(
        &page,
        "Alice",
        "alice@example.com",
    );

    user_card(
        &page,
        "Bob",
        "bob@example.com",
    );

    println!("{}", page.render_pretty());
}
