# domlink

[English](#english) | [Русский](#русский)

---

## English

A lightweight, safe HTML builder for Rust. Construct HTML trees programmatically using a fluent builder API, with automatic XSS protection and direct rendering to any `fmt::Write` sink.

### Features

- **Fluent builder API** — construct HTML trees with chained method calls
- **XSS protection by default** — all text content and attribute values are escaped automatically
- **Explicit unsafe escape hatches** — `raw_html` and `raw_attr` are clearly named and documented
- **Zero intermediate allocations** — renders directly to `String`, `fmt::Formatter`, or any `io::Write` via `IoWriteAdapter`
- **Fragment support** — `Tags::Any` renders children without a wrapping element
- **Template engine** — `Tpl` for pre-built templates with `{}` placeholders
- **No dependencies** — only the Rust standard library

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
domlink = "0.1"
```

### Quick Start

```rust
use domlink::{init, Tags};

let root = init(Tags::Html);

let head = root.head();
head.title().text("My Page");
head.link().raw_attr(r#"rel="stylesheet" href="style.css""#);

let body = root.body();
body.div()
    .id("app")
    .class("container")
    .class("dark")
    .data("role", "main");

let form = body.form().class("login-form");
form.label().attr("for", "username").text("Username:");
form.input().id("username").name("user");

body.script().raw_html("console.log('ready');");

println!("{}", root);
```

Output:

```html
<!DOCTYPE html>
<html>
  <head>
    <title>
      My Page
    </title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <div id="app" class="container dark" data-role="main">
    </div>
    <form class="login-form">
      <label for="username">
        Username:
      </label>
      <input id="username" name="user">
    </form>
    <script>
    console.log('ready');
    </script>
  </body>
</html>
```

### API Overview

#### Creating elements

```rust
let root = init(Tags::Div);         // root element
let child = root.div();             // append a child
let named = root.div().id("main");  // with id
```

All tag shortcuts are available directly on `Link`: `div`, `span`, `p`, `a`, `ul`, `li`, `table`, `tr`, `td`, `form`, `input`, `button`, `img`, `script`, `style`, and more.

#### Attributes

```rust
el.id("my-id")                  // id="my-id"
  .class("foo").class("bar")    // class="foo bar"  (merged)
  .attr("type", "text")         // any safe attribute
  .data("key", "value")         // data-key="value"
  .name("field")
  .value("default");
```

All values are HTML-escaped automatically.

#### Text content

```rust
el.text("Hello, <world>");   // escaped: Hello, &lt;world&gt;
```

#### Unsafe methods

Use only with trusted input:

```rust
el.raw_attr(r#"x-data="{ open: false }""#);  // Alpine.js / HTMX
el.raw_html("<script>alert(1)</script>");      // unescaped HTML
```

#### Rendering

```rust
// To String
let html: String = root.render();

// Via Display
println!("{}", root);
let s = format!("{}", root);

// To any fmt::Write
root.render_into(&mut writer)?;

// To any io::Write (file, socket, HTTP response)
use domlink::IoWriteAdapter;
let mut adapter = IoWriteAdapter(std::io::BufWriter::new(file));
root.render_into(&mut adapter)?;
```

#### Fragments

`Tags::Any` renders its children without a wrapping tag, useful for returning multiple sibling elements:

```rust
let frag = root.any();
frag.span().text("first");
frag.span().text("second");
// renders two <span> elements at the same depth, no wrapper
```

#### Templates

`Tpl` pre-renders a structure once and fills `{}` placeholders repeatedly:

```rust
let row = init(Tags::Tr);
row.td().tpl();
row.td().tpl();
let mut tpl = row.template();

for (name, email) in &users {
    // render_escaped escapes values before inserting
    println!("{}", tpl.render_escaped(&[name, email]));
}
```

Use `render` when values are already safe, `render_escaped` when they come from user input.

### Safety

| Method | Escapes input |
|---|---|
| `text(s)` | Yes |
| `attr(name, value)` | Yes (value) |
| `id`, `class`, `name`, `value`, `data` | Yes |
| `raw_attr(s)` | **No** |
| `raw_html(s)` | **No** |
| `Tpl::render` | **No** |
| `Tpl::render_escaped` | Yes |

Attribute names are validated — passing an invalid name (e.g. containing spaces or quotes) will panic.

Appending children to void elements (`input`, `img`, `br`, `meta`, `link`) will panic.

### License

MIT

---

## Русский

Лёгкая и безопасная библиотека для построения HTML на Rust. Позволяет конструировать HTML-деревья программно через fluent builder API с автоматической защитой от XSS и прямой записью в любой `fmt::Write`-приёмник.

### Возможности

- **Fluent builder API** — построение HTML-деревьев через цепочки вызовов методов
- **Защита от XSS по умолчанию** — весь текст и значения атрибутов экранируются автоматически
- **Явные небезопасные методы** — `raw_html` и `raw_attr` явно названы и задокументированы
- **Без лишних аллокаций** — рендер напрямую в `String`, `fmt::Formatter` или любой `io::Write` через `IoWriteAdapter`
- **Поддержка фрагментов** — `Tags::Any` рендерит детей без обёртки
- **Шаблонизатор** — `Tpl` для предварительно собранных шаблонов с плейсхолдерами `{}`
- **Без зависимостей** — только стандартная библиотека Rust

### Установка

Добавьте в `Cargo.toml`:

```toml
[dependencies]
domlink = "0.1"
```

### Быстрый старт

```rust
use domlink::{init, Tags};

let root = init(Tags::Html);

let head = root.head();
head.title().text("Моя страница");
head.link().raw_attr(r#"rel="stylesheet" href="style.css""#);

let body = root.body();
body.div()
    .id("app")
    .class("container")
    .class("dark")
    .data("role", "main");

let form = body.form().class("login-form");
form.label().attr("for", "username").text("Пользователь:");
form.input().id("username").name("user");

body.script().raw_html("console.log('готово');");

println!("{}", root);
```

### Обзор API

#### Создание элементов

```rust
let root = init(Tags::Div);         // корневой элемент
let child = root.div();             // добавить дочерний элемент
let named = root.div().id("main");  // с атрибутом id
```

Все теги доступны как методы на `Link`: `div`, `span`, `p`, `a`, `ul`, `li`, `table`, `tr`, `td`, `form`, `input`, `button`, `img`, `script`, `style` и другие.

#### Атрибуты

```rust
el.id("my-id")                  // id="my-id"
  .class("foo").class("bar")    // class="foo bar"  (склеиваются)
  .attr("type", "text")         // любой безопасный атрибут
  .data("key", "value")         // data-key="value"
  .name("field")
  .value("default");
```

Все значения экранируются автоматически.

#### Текстовое содержимое

```rust
el.text("Привет, <мир>");   // экранируется: Привет, &lt;мир&gt;
```

#### Небезопасные методы

Использовать только с доверенными данными:

```rust
el.raw_attr(r#"x-data="{ open: false }""#);  // Alpine.js / HTMX
el.raw_html("<script>alert(1)</script>");      // HTML без экранирования
```

#### Рендер

```rust
// В String
let html: String = root.render();

// Через Display
println!("{}", root);
let s = format!("{}", root);

// В любой fmt::Write
root.render_into(&mut writer)?;

// В любой io::Write (файл, сокет, HTTP-ответ)
use domlink::IoWriteAdapter;
let mut adapter = IoWriteAdapter(std::io::BufWriter::new(file));
root.render_into(&mut adapter)?;
```

#### Фрагменты

`Tags::Any` рендерит своих детей без тега-обёртки:

```rust
let frag = root.any();
frag.span().text("первый");
frag.span().text("второй");
// рендерит два <span> на одном уровне без обёртки
```

#### Шаблоны

`Tpl` собирает структуру один раз и многократно заполняет плейсхолдеры `{}`:

```rust
let row = init(Tags::Tr);
row.td().tpl();
row.td().tpl();
let mut tpl = row.template();

for (name, email) in &users {
    // render_escaped экранирует значения перед вставкой
    println!("{}", tpl.render_escaped(&[name, email]));
}
```

Используйте `render` если данные уже безопасны, `render_escaped` — если приходят от пользователя.

### Безопасность

| Метод | Экранирует |
|---|---|
| `text(s)` | Да |
| `attr(name, value)` | Да (значение) |
| `id`, `class`, `name`, `value`, `data` | Да |
| `raw_attr(s)` | **Нет** |
| `raw_html(s)` | **Нет** |
| `Tpl::render` | **Нет** |
| `Tpl::render_escaped` | Да |

Имена атрибутов валидируются — передача некорректного имени (например, содержащего пробелы или кавычки) вызовет панику.

Добавление дочерних элементов к void-элементам (`input`, `img`, `br`, `meta`, `link`) вызовет панику.

### Лицензия

MIT
