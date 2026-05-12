# domlink

[English](#english) | [Русский](#русский)

---

# English

Fast and safe runtime HTML rendering for Rust.

Domlink combines two independent rendering systems:

* Safe DOM builder
* Ultra-light runtime template engine

Domlink focuses on:

* runtime flexibility,
* XSS safety,
* predictable performance,
* low allocations,
* streaming rendering,
* zero proc-macros,
* zero code generation,
* zero compile-time templates,
* zero dependencies.

Unlike Askama or Maud, Domlink templates can be created dynamically during runtime.

---

# Features

## Safe HTML rendering

All text and attribute values are escaped automatically.

```rust
div.text("<script>alert(1)</script>");
```

renders as:

```html
<div>
  &lt;script&gt;alert(1)&lt;/script&gt;
</div>
```

Escaped characters:

* `&`
* `<`
* `>`
* `"`
* `'`

---

## Runtime templates

Templates can be created dynamically during program execution.

```rust
let tpl = Tpl::new("<div>{}</div>");
```

This enables:

* CMS systems,
* database-stored templates,
* dynamic email layouts,
* runtime-generated HTML,
* HTMX fragments,
* SSR without compile-time templates,
* multi-tenant rendering.

---

## Precompiled runtime templates

`Tpl::new()` parses the template only once.

Subsequent renders reuse the prepared structure.

```rust
let tpl = Tpl::new("<tr><td>{}</td></tr>");

for row in rows {
    tpl.render_into(&mut out, &[row]);
}
```

This makes rendering extremely lightweight.

---

## Mixed rendering with SafeHtml

Domlink supports mixed rendering:

* plain text is escaped,
* trusted HTML fragments are inserted without repeated escaping.

```rust
let rows: SafeHtml = build_rows();

page_tpl.render_into(
    &mut out,
    &[
        TplArg::Text(title),
        TplArg::Html(rows.as_str()),
    ]
);
```

This avoids unnecessary escaping passes and improves performance in hot paths.

---

## DOM Builder

```rust
use domlink::{init, Tags};

let page = init(Tags::Html);

page.body()
    .div()
    .class("container")
    .text("Hello");

println!("{}", page.render());
```

---

## Pretty and Compact rendering

### Pretty

Readable formatted HTML:

```rust
page.render_pretty();
```

### Compact

Minified HTML without indentation or newlines:

```rust
page.render_compact();
```

Useful for:

* SSR,
* APIs,
* production HTML,
* bandwidth reduction.

---

## Direct streaming support

Render directly into:

* files,
* sockets,
* HTTP responses,
* buffers.

```rust
use domlink::IoWriteAdapter;

let mut adapter = IoWriteAdapter(writer);

page.render_into(&mut adapter)?;
```

No intermediate page allocation required.

---

## Fragment rendering

`Tags::Any` renders children without a wrapping element.

```rust
let frag = root.any();

frag.span().text("first");
frag.span().text("second");
```

renders as:

```html
<span>first</span>
<span>second</span>
```

---

# Installation

```toml
[dependencies]
domlink = "0.1"
```

---

# Why Domlink?

## Runtime flexibility

Askama and Maud are compile-time systems.

Domlink supports:

* runtime templates,
* dynamic layouts,
* user-generated templates,
* runtime HTML composition.

---

## No macros

No proc-macros.
No generated code.
No template compilation step.

Just Rust.

---

## Predictable performance

Tpl rendering is essentially:

* linear string append,
* minimal branching,
* low allocation overhead.

---

## XSS safety by default

Safe by default.

Unsafe operations are explicit:

```rust
.raw_html(...)
.raw_attr(...)
```

---

# Architecture

Domlink contains two independent rendering systems.

---

## 1. DOM Renderer

Tree-based HTML builder.

Good for:

* structured HTML,
* reusable UI components,
* complex page generation.

---

## 2. Tpl Renderer

Ultra-fast runtime interpolation engine.

Good for:

* hot paths,
* tables,
* repeated fragments,
* SSR loops,
* email rendering,
* runtime templates.

---

# Benchmarks

Example benchmark (10k rows):

| Renderer            | Time    |
| ------------------- | ------- |
| Domlink Tpl         | ~1.38ms |
| Maud                | ~1.49ms |
| Askama              | ~2.37ms |
| Domlink Compact DOM | ~7.5ms  |

Actual performance depends on workload and allocator.

---

# API Safety

| Method                                 | Escapes input |
| -------------------------------------- | ------------- |
| `text(s)`                              | Yes           |
| `attr(name, value)`                    | Yes           |
| `id`, `class`, `name`, `value`, `data` | Yes           |
| `TplArg::Text`                         | Yes           |
| `TplArg::Html`                         | No            |
| `raw_attr(s)`                          | No            |
| `raw_html(s)`                          | No            |

---

# Example

```rust
use domlink::{init, Tags};

fn main() {
    let page = init(Tags::Html);

    page.head()
        .title()
        .text("Users");

    let body = page.body();

    body.h1().text("Users");

    let table = body.table();

    let tr = table.tr();

    tr.td().text("1");
    tr.td().text("John");

    println!("{}", page.render_pretty());
}
```

---

# License

MIT

---

# Русский

Быстрый и безопасный runtime HTML renderer для Rust.

Domlink сочетает две независимые системы рендера:

* безопасный DOM builder,
* сверхлегкий runtime template engine.

Domlink ориентирован на:

* runtime-гибкость,
* XSS-безопасность,
* предсказуемую производительность,
* минимальные аллокации,
* потоковый рендеринг,
* отсутствие proc-macro,
* отсутствие codegen,
* отсутствие compile-time шаблонов,
* отсутствие зависимостей.

В отличие от Askama и Maud, шаблоны Domlink можно создавать динамически во время выполнения программы.

---

# Возможности

## Безопасный HTML rendering

Текст и атрибуты экранируются автоматически.

```rust
div.text("<script>alert(1)</script>");
```

рендерится как:

```html
<div>
  &lt;script&gt;alert(1)&lt;/script&gt;
</div>
```

Экранируются:

* `&`
* `<`
* `>`
* `"`
* `'`

---

## Runtime шаблоны

Шаблоны можно создавать динамически во время выполнения программы.

```rust
let tpl = Tpl::new("<div>{}</div>");
```

Это позволяет делать:

* CMS,
* шаблоны из БД,
* динамические email layout,
* runtime HTML generation,
* HTMX fragments,
* SSR без compile-time шаблонов,
* multi-tenant rendering.

---

## Предкомпилированные runtime шаблоны

`Tpl::new()` разбирает шаблон только один раз.

Последующие вызовы используют уже подготовленную структуру.

```rust
let tpl = Tpl::new("<tr><td>{}</td></tr>");

for row in rows {
    tpl.render_into(&mut out, &[row]);
}
```

Это делает рендер очень легковесным.

---

## Смешанный рендеринг через SafeHtml

Domlink поддерживает смешанный рендеринг:

* обычный текст экранируется,
* доверенные HTML-фрагменты вставляются без повторного escaping.

```rust
let rows: SafeHtml = build_rows();

page_tpl.render_into(
    &mut out,
    &[
        TplArg::Text(title),
        TplArg::Html(rows.as_str()),
    ]
);
```

Это убирает лишние проходы escaping и ускоряет hot path.

---

## DOM Builder

```rust
use domlink::{init, Tags};

let page = init(Tags::Html);

page.body()
    .div()
    .class("container")
    .text("Hello");

println!("{}", page.render());
```

---

## Pretty и Compact rendering

### Pretty

Человекочитаемый HTML:

```rust
page.render_pretty();
```

### Compact

Минифицированный HTML без переносов и отступов:

```rust
page.render_compact();
```

Полезно для:

* SSR,
* API,
* production HTML,
* уменьшения трафика.

---

## Потоковый рендеринг

Можно рендерить напрямую:

* в файл,
* в сокет,
* в HTTP response,
* в буфер.

```rust
use domlink::IoWriteAdapter;

let mut adapter = IoWriteAdapter(writer);

page.render_into(&mut adapter)?;
```

Без промежуточной аллокации всей страницы.

---

## Рендеринг фрагментов

`Tags::Any` рендерит детей без тега-обёртки.

```rust
let frag = root.any();

frag.span().text("first");
frag.span().text("second");
```

рендерится как:

```html
<span>first</span>
<span>second</span>
```

---

# Установка

```toml
[dependencies]
domlink = "0.1"
```

---

# Почему Domlink?

## Runtime гибкость

Askama и Maud — compile-time системы.

Domlink поддерживает:

* runtime templates,
* динамические layout,
* пользовательские шаблоны,
* runtime HTML composition.

---

## Без макросов

Нет proc-macro.
Нет generated code.
Нет compile-time template compilation.

Только обычный Rust.

---

## Предсказуемая производительность

Tpl renderer — это почти:

* linear string append,
* минимальный branching,
* минимальные накладные расходы.

---

## XSS-защита по умолчанию

Безопасность включена по умолчанию.

Опасные операции сделаны явными:

```rust
.raw_html(...)
.raw_attr(...)
```

---

# Архитектура

Domlink содержит две независимые системы рендера.

---

## 1. DOM Renderer

Древовидный HTML builder.

Подходит для:

* сложного HTML,
* UI-компонентов,
* структурированной генерации страниц.

---

## 2. Tpl Renderer

Сверхбыстрый runtime interpolation engine.

Подходит для:

* hot path,
* таблиц,
* повторяющихся фрагментов,
* SSR циклов,
* email rendering,
* runtime шаблонов.

---

# Бенчмарки

Пример benchmark (10k строк):

| Renderer            | Time    |
| ------------------- | ------- |
| Domlink Tpl         | ~1.38ms |
| Maud                | ~1.49ms |
| Askama              | ~2.37ms |
| Domlink Compact DOM | ~7.5ms  |

---

# Безопасность API

| Метод                                  | Экранирует |
| -------------------------------------- | ---------- |
| `text(s)`                              | Да         |
| `attr(name, value)`                    | Да         |
| `id`, `class`, `name`, `value`, `data` | Да         |
| `TplArg::Text`                         | Да         |
| `TplArg::Html`                         | Нет        |
| `raw_attr(s)`                          | Нет        |
| `raw_html(s)`                          | Нет        |

---

# Пример

```rust
use domlink::{init, Tags};

fn main() {
    let page = init(Tags::Html);

    page.head()
        .title()
        .text("Users");

    let body = page.body();

    body.h1().text("Users");

    let table = body.table();

    let tr = table.tr();

    tr.td().text("1");
    tr.td().text("John");

    println!("{}", page.render_pretty());
}
```

---

# License

MIT
