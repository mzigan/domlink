use domlink::{init, Tags};

struct User {
    id: usize,
    name: String,
    email: String,
}

fn main() {
    let users = vec![
        User {
            id: 1,
            name: "Alice".into(),
            email: "alice@example.com".into(),
        },
        User {
            id: 2,
            name: "Bob".into(),
            email: "bob@example.com".into(),
        },
    ];

    let page = init(Tags::Html);

    let body = page.body();

    body.h1().text("Users");

    let table = body.table().class("users");
    let tbody = table.tbody();

    for user in users {
        let tr = tbody.tr();

        tr.td().text(&user.id.to_string());
        tr.td().text(&user.name);
        tr.td().text(&user.email);
    }

    println!("{}", page.render_pretty());
}
