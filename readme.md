# domlink

[English](#english) | [Русский](#русский)

---

## English

Fast and safe runtime HTML rendering for Rust.

Domlink combines two approaches:

- Safe DOM builder
- Ultra-light runtime template engine

The library focuses on:
- runtime flexibility,
- XSS safety,
- minimal overhead,
- predictable rendering performance,
- zero proc-macros,
- zero code generation,
- zero build-time template compilation.

---

# Features

## Safe HTML rendering

All text and attribute values are escaped by default.

```rust
div.text("<script>alert(1)</script>");
````

renders as:

```html
<div>
  &lt;script&gt;alert(1)&lt;/script&gt;
</div>
```

Protected characters:

* `&`
* `<`
* `>`
* `"`
* `'`

---

## Runtime templates

Unlike Askama or Maud, templates can be created dynamically during runtime.

```rust
let tpl = Tpl::new("<div>{}</div>");
```

This enables:

* CMS systems,
* database-stored templates,
* multi-tenant rendering,
* dynamic email layouts,
* runtime-generated HTML,
* HTMX fragments,
* SSR without compile-time templates.

---

## Precompiled runtime templates

`Tpl::new()` parses the template once.

Subsequent renders reuse the prepared structure.

```rust
let tpl = Tpl::new("<tr><td>{}</td></tr>");

for row in rows {
    tpl.render_into(&mut out, &[row]);
}
```

This makes rendering extremely lightweight.

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
let mut adapter = IoWriteAdapter(writer);

page.render_into(&mut adapter)?;
```

No intermediate allocation required.

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

No proc macros.
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

## 1. DOM Renderer

Tree-based HTML builder.

Good for:

* structured HTML,
* complex page generation,
* reusable UI components.

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

| Renderer    | Time    |
| ----------- | ------- |
| Domlink Tpl | ~1.5 ms |
| Maud        | ~1.7 ms |
| Askama      | ~2.8 ms |

Actual performance depends on workload and allocator.

---

# Safety

Safe APIs escape HTML automatically.

Unsafe APIs:

* `raw_html`
* `raw_attr`

should only be used with trusted input.

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

## Русский

Быстрый и безопасный runtime HTML renderer для Rust.

Domlink сочетает два подхода:

- безопасный DOM builder,
- сверхлегкий runtime template engine.

Библиотека ориентирована на:
- runtime-гибкость,
- XSS-безопасность,
- минимальные накладные расходы,
- предсказуемую производительность,
- отсутствие proc-macro,
- отсутствие codegen,
- отсутствие compile-time шаблонов.

---

# Возможности

## Безопасный HTML rendering

Текст и атрибуты экранируются автоматически.

```rust
div.text("<script>alert(1)</script>");
````

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

В отличие от Askama и Maud, шаблоны можно создавать динамически во время выполнения программы.

```rust
let tpl = Tpl::new("<div>{}</div>");
```

Это позволяет делать:

* CMS,
* шаблоны из БД,
* multi-tenant rendering,
* динамические email layout,
* runtime HTML generation,
* HTMX fragments,
* SSR без compile-time шаблонов.

---

## Предкомпилированные runtime шаблоны

`Tpl::new()` разбирает шаблон только один раз.

Дальнейшие вызовы используют уже подготовленную структуру.

```rust
let tpl = Tpl::new("<tr><td>{}</td></tr>");

for row in rows {
    tpl.render_into(&mut out, &[row]);
}
```

Это делает рендер очень легковесным.

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

## Streaming rendering

Можно рендерить напрямую:

* в файл,
* в сокет,
* в HTTP response,
* в буфер.

```rust
let mut adapter = IoWriteAdapter(writer);

page.render_into(&mut adapter)?;
```

Без промежуточной аллокации всей страницы.

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

| Renderer    | Time    |
| ----------- | ------- |
| Domlink Tpl | ~1.5 ms |
| Maud        | ~1.7 ms |
| Askama      | ~2.8 ms |

Результаты зависят от нагрузки и allocator.

---

# Безопасность

Безопасные API автоматически экранируют HTML.

Опасные API:

* `raw_html`
* `raw_attr`

должны использоваться только с доверенным вводом.

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
