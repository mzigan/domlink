use domlink::{Tags, init};

fn main() {
    let root = init(Tags::Html);

    let head = root.head();
    head.title().text("My Safe Page");
    head.link()
        .raw_attr("rel=\"stylesheet\" href=\"style.css\""); // Сырой атрибут

    let body = root.body();

    body.div()
        .id("app")
        .class("container")
        .class("dark") // Классы склеятся!
        .data("role", "main"); // data-role="main"

    // Безопасная форма (XSS атака будет нейтрализована)
    let form = body.form().class("login-form");
    form.label().attr("for", "username").text("User:"); // attr(name, val) - безопасно
    form.input()
        .id("username")
        .name("user")
        .value("<script>alert('xss')</script>"); // В HTML выведется экранированное значение
    let div = form.div();
    div.button().attr("type", "submit").text("Login");
    div.svg().raw_attr(r#"width="100" height="100""#).raw_html(
        r#"<circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />"#,
    );

    let span = div
        .span()
        .text("SVG above is safe, even with < and > in attributes or content.");
    span.raw_attr(r#"data-info="This is a <test> & should be safe""#)
        .raw_html("Raw HTML inside span, but it's trusted content.");
    let a = div.a();
    a.clone()
        .text("Example Link")
        .attr("href", "https://example.com");
    a.br();

    // Опасная ссылка (XSS атака будет нейтрализована)
    body.a()
        .attr("href", "javascript:alert('xss')")
        .text("Click me!");

    // ОПАСНЫЙ ВЫЗОВ: Добавляем div внутрь img
    let _img = body.img();
    // img.div()
    //     .text("Здесь будет паника");

    // Script (Используем raw_html для JS)
    body.script().raw_html("console.log('Hello, <world>!');");

    // Textarea (Текст будет вставлен без лишних переносов)
    body.textarea().name("comment").text("Default\nText");

    let html = root.render();
    println!("{}", html);
}
