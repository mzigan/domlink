use askama::Template;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use maud::{DOCTYPE, html};

use domlink::{SafeHtml, Tags, Tpl, TplArg, init};

#[derive(Clone)]
struct User {
    id_str: String,
    name: String,
    email: String,
    bio: String,
}

fn make_users(n: usize) -> Vec<User> {
    (0..n)
        .map(|i| User {
            id_str: i.to_string(),
            name: format!("User {i}"),
            email: format!("user{i}@example.com"),
            bio: format!("Bio with <unsafe> chars & quotes \" '{i}"),
        })
        .collect()
}

// -----------------------------------------------------------------------------
// Domlink
// -----------------------------------------------------------------------------

fn domlink_build_and_render_compact(title: &str, users: &[User]) -> String {
    let page = init(Tags::Html);

    let head = page.head();
    head.title().text(title);

    let body = page.body();

    body.h1().text(title);

    let table = body.table().class("users-table");
    let tbody = table.tbody();

    for user in users {
        let tr = tbody.tr();

        tr.td().text(&user.id_str);
        tr.td().text(&user.name);
        tr.td().text(&user.email);
        tr.td().text(&user.bio);
    }

    page.render_compact()
}

fn domlink_tpl_render(title: &str, users: &[User]) -> String {
    let row_tpl = Tpl::new("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n");

    let mut rows_buf = String::with_capacity(users.len() * 180);

    for user in users {
        row_tpl.render_into(
            &mut rows_buf,
            &[&user.id_str, &user.name, &user.email, &user.bio],
        );
    }

    let rows = SafeHtml::new_unchecked(rows_buf);

    let page_tpl = Tpl::new(
        "<!DOCTYPE html>
<html>
<head><title>{}</title></head>
<body>
<h1>{}</h1>
<table class=\"users-table\">
<tbody>
{}
</tbody>
</table>
</body>
</html>",
    );

    let mut out = String::with_capacity(rows.as_str().len() + 512);

    page_tpl.render_mixed_into(
        &mut out,
        &[
            TplArg::Text(title),
            TplArg::Text(title),
            TplArg::Html(&rows),
        ],
    );

    out
}

// -----------------------------------------------------------------------------
// Askama
// -----------------------------------------------------------------------------

#[derive(Template)]
#[template(
    source = r#"
<!DOCTYPE html>
<html>
<head><title>{{ title }}</title></head>
<body>
<h1>{{ title }}</h1>
<table class="users-table">
<tbody>
{% for user in users %}
<tr>
<td>{{ user.id_str }}</td>
<td>{{ user.name }}</td>
<td>{{ user.email }}</td>
<td>{{ user.bio }}</td>
</tr>
{% endfor %}
</tbody>
</table>
</body>
</html>
"#,
    ext = "html",
    escape = "html"
)]
struct AskamaPage<'a> {
    title: &'a str,
    users: &'a [User],
}

fn askama_render(title: &str, users: &[User]) -> String {
    AskamaPage { title, users }.render().unwrap()
}

// -----------------------------------------------------------------------------
// Maud
// -----------------------------------------------------------------------------

fn maud_render(title: &str, users: &[User]) -> String {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
            }
            body {
                h1 { (title) }
                table class="users-table" {
                    tbody {
                        @for user in users {
                            tr {
                                td { (user.id_str) }
                                td { (&user.name) }
                                td { (&user.email) }
                                td { (&user.bio) }
                            }
                        }
                    }
                }
            }
        }
    }
    .into_string()
}

// -----------------------------------------------------------------------------
// Benchmarks
// -----------------------------------------------------------------------------

fn bench_renderers(c: &mut Criterion) {
    let mut group = c.benchmark_group("html_renderers");

    for size in [10usize, 100, 1_000, 10_000] {
        let users = make_users(size);
        let title = "Users";

        group.bench_with_input(
            BenchmarkId::new("domlink_build_and_render_compact", size),
            &users,
            |b, users| {
                b.iter(|| {
                    black_box(domlink_build_and_render_compact(
                        black_box(title),
                        black_box(users),
                    ))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("domlink_tpl_render", size),
            &users,
            |b, users| {
                b.iter(|| black_box(domlink_tpl_render(black_box(title), black_box(users))));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("askama_render", size),
            &users,
            |b, users| {
                b.iter(|| black_box(askama_render(black_box(title), black_box(users))));
            },
        );

        group.bench_with_input(BenchmarkId::new("maud_render", size), &users, |b, users| {
            b.iter(|| black_box(maud_render(black_box(title), black_box(users))));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_renderers);
criterion_main!(benches);
