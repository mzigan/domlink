use domlink::{IoWriteAdapter, Tags, Tpl, init};

#[test]
fn test_xss_in_text() {
    let el = init(Tags::Div).text("<script>alert(1)</script>");
    assert!(!el.render().contains("<script>"));
}

#[test]
fn test_class_merge() {
    let el = init(Tags::Div).class("a").class("b");
    assert!(el.render().contains("class=\"a b\""));
}

#[test]
fn test_void_no_closing_tag() {
    let el = init(Tags::Input);
    assert!(!el.render().contains("</input>"));
}

// Вложенность
#[test]
fn test_nesting() {
    let root = init(Tags::Div);
    root.span().text("hello");
    assert!(root.render().contains("<span>"));
}

// data-атрибут
#[test]
fn test_data_attr() {
    let el = init(Tags::Div).data("foo", "<bar>");
    assert!(el.render().contains("data-foo=\"&lt;bar&gt;\""));
}

// Textarea без лишних переносов
#[test]
fn test_textarea_text() {
    let el = init(Tags::Textarea).text("hello");
    let r = el.render();
    assert!(r.contains("<textarea>hello</textarea>"));
}

// Display совпадает с render
#[test]
fn test_display_eq_render() {
    let el = init(Tags::Div).class("x");
    assert_eq!(el.render(), format!("{}", el));
}

#[test]
fn test_escape_all_special_chars_in_text() {
    let el = init(Tags::Div).text("&<>\"'");
    let r = el.render();

    assert!(r.contains("&amp;&lt;&gt;&quot;&#39;"));
}

#[test]
fn test_escape_attr_value() {
    let el = init(Tags::Div).attr("title", "\" onclick=\"alert(1)");
    let r = el.render();

    assert!(r.contains("title=\"&quot; onclick=&quot;alert(1)\""));
}

#[test]
fn test_invalid_attr_name_panics() {
    let result = std::panic::catch_unwind(|| {
        init(Tags::Div).attr("onclick=\"alert(1)", "x");
    });

    assert!(result.is_err());
}

#[test]
fn test_data_invalid_key_panics() {
    let result = std::panic::catch_unwind(|| {
        init(Tags::Div).data("foo onclick", "bar");
    });

    assert!(result.is_err());
}

#[test]
fn test_raw_html_is_not_escaped() {
    let el = init(Tags::Div).raw_html("<span>raw</span>");
    let r = el.render();

    assert!(r.contains("<span>raw</span>"));
}

#[test]
fn test_raw_attr_is_not_escaped() {
    let el = init(Tags::Div).raw_attr("x-data=\"{ open: false }\"");
    let r = el.render();

    assert!(r.contains("x-data=\"{ open: false }\""));
}

#[test]
fn test_append_to_void_panics() {
    let result = std::panic::catch_unwind(|| {
        let img = init(Tags::Img);
        img.div();
    });

    assert!(result.is_err());
}

#[test]
fn test_tpl_render_raw() {
    let tpl = Tpl::new("<div>{}</div>");
    let r = tpl.render_raw(&["<span>raw</span>"]);

    assert_eq!(r, "<div><span>raw</span></div>");
}

#[test]
fn test_tpl_render_escaped() {
    let tpl = Tpl::new("<div>{}</div>");
    let r = tpl.render(&["<script>alert(1)</script>"]);

    assert_eq!(r, "<div>&lt;script&gt;alert(1)&lt;/script&gt;</div>");
}

#[test]
fn test_tpl_multiple_placeholders() {
    let tpl = Tpl::new("<p>{} - {}</p>");
    let r = tpl.render(&["A&B", "<tag>"]);

    assert_eq!(r, "<p>A&amp;B - &lt;tag&gt;</p>");
}

#[test]
fn test_tpl_missing_values_are_empty() {
    let tpl = Tpl::new("<p>{} {}</p>");
    let r = tpl.render(&["hello"]);

    assert_eq!(r, "<p>hello </p>");
}

#[test]
fn test_tpl_extra_values_are_ignored() {
    let tpl = Tpl::new("<p>{}</p>");
    let r = tpl.render_raw(&["a", "b", "c"]);

    assert_eq!(r, "<p>a</p>");
}

#[test]
fn test_doctype_for_html() {
    let r = init(Tags::Html).render();

    assert!(r.starts_with("<!DOCTYPE html>\n<html>"));
}

#[test]
fn test_script_raw_html() {
    let el = init(Tags::Script).raw_html("console.log('<x>');");
    let r = el.render();

    assert!(r.contains("console.log('<x>');"));
    assert!(!r.contains("&lt;x&gt;"));
}

#[test]
fn test_script_text_is_escaped() {
    let el = init(Tags::Script).text("console.log('<x>');");
    let r = el.render();

    assert!(r.contains("&lt;x&gt;"));
}

#[test]
fn test_nested_render_order() {
    let root = init(Tags::Div);
    root.p().text("first");
    root.span().text("second");

    let r = root.render();

    let p_pos = r.find("<p>").unwrap();
    let span_pos = r.find("<span>").unwrap();

    assert!(p_pos < span_pos);
}

#[test]
fn test_any_fragment_should_not_render_extra_angle_bracket() {
    let root = init(Tags::Any);
    root.div().text("hello");

    let r = root.render();

    assert!(!r.trim_start().starts_with('>'));
    assert!(r.contains("<div>"));
}

#[test]
fn test_tpl_no_placeholders() {
    let tpl = Tpl::new("<p>static</p>");
    assert_eq!(tpl.render(&["ignored"]), "<p>static</p>");
}

#[test]
fn test_tpl_empty_string() {
    let tpl = Tpl::new("");
    assert_eq!(tpl.render(&[]), "");
}

#[test]
fn test_multiple_id_calls_last_wins() {
    let el = init(Tags::Div).id("first").id("second");
    let r = el.render();
    // последний вызов перезаписывает предыдущий — id только один
    assert_eq!(r.matches("id=").count(), 1);
    assert!(r.contains(r#"id="second""#));
    assert!(!r.contains(r#"id="first""#));
}

#[test]
fn test_class_xss() {
    let el = init(Tags::Div).class("a\" onclick=\"alert(1)");
    let r = el.render();
    assert!(r.contains("&quot;")); // кавычка экранирована
    assert!(!r.contains("onclick=\"")); // не стала настоящим атрибутом
}

#[test]
fn test_deep_nesting_renders_correctly() {
    let root = init(Tags::Div);
    let inner = root.div().div().div();
    inner.text("deep");
    let r = root.render();
    assert!(r.contains("deep"));
    // три уровня вложенности — шесть пробелов отступа
    assert!(r.contains("      deep"));
}

#[test]
fn test_any_multiple_children_same_depth() {
    let root = init(Tags::Div);
    let frag = root.any();
    frag.span().text("a");
    frag.span().text("b");
    let r = root.render();
    // оба span на одном уровне отступа
    let lines: Vec<_> = r.lines().filter(|l| l.contains("<span>")).collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].len(), lines[1].len()); // одинаковый отступ
}

#[test]
fn test_display_of_child_not_root() {
    let root = init(Tags::Div);
    let child = root.span().text("hi");
    // рендерим не корень, а дочерний элемент
    let r = child.render();
    assert!(r.contains("<span>"));
    assert!(!r.contains("<div>"));
}

#[test]
fn test_empty_div_renders_open_and_close() {
    let r = init(Tags::Div).render();
    assert!(r.contains("<div>"));
    assert!(r.contains("</div>"));
}

#[test]
fn test_void_element_no_children_in_output() {
    let r = init(Tags::Br).render();
    assert_eq!(r.trim(), "<br>");
}

#[test]
fn test_data_empty_key_panics() {
    let result = std::panic::catch_unwind(|| {
        init(Tags::Div).data("", "value");
    });
    assert!(result.is_err());
}

#[test]
fn test_io_write_adapter() {
    let el = init(Tags::Div).text("hello");
    let mut buf = Vec::new();
    let mut adapter = IoWriteAdapter(&mut buf);
    el.render_into(&mut adapter).unwrap();
    let s = String::from_utf8(buf).unwrap();
    assert!(s.contains("<div>"));
}

#[test]
fn test_tpl_render_into_io_writer() {
    let tpl = Tpl::new("<div>{}</div>");

    let mut buf = Vec::new();
    let mut adapter = IoWriteAdapter(&mut buf);

    tpl.render_into(&mut adapter, &["hello"]).unwrap();

    let s = String::from_utf8(buf).unwrap();
    assert_eq!(s, "<div>hello</div>");
}

#[test]
fn test_tpl_render_mixed_into_io_writer() {
    use domlink::{SafeHtml, TplArg};

    let tpl = Tpl::new("<p>{}{}</p>");
    let safe = SafeHtml::new_unchecked("<b>html</b>".to_string());

    let mut buf = Vec::new();
    let mut adapter = IoWriteAdapter(&mut buf);

    tpl.render_mixed_into(&mut adapter, &[TplArg::Text("<text>"), TplArg::Html(&safe)])
        .unwrap();

    let s = String::from_utf8(buf).unwrap();
    assert_eq!(s, "<p>&lt;text&gt;<b>html</b></p>");
}

#[test]
fn test_tpl_render_raw_into_io_writer() {
    let tpl = Tpl::new("<div>{}</div>");

    let mut buf = Vec::new();
    let mut adapter = IoWriteAdapter(&mut buf);

    tpl.render_raw_into(&mut adapter, &["<b>raw</b>"]).unwrap();

    let s = String::from_utf8(buf).unwrap();
    assert_eq!(s, "<div><b>raw</b></div>");
}

#[test]
fn test_tpl_render_into_returns_result_on_success() {
    let tpl = Tpl::new("<div>{}</div>");
    let mut out = String::new();
    let result = tpl.render_into(&mut out, &["ok"]);
    assert!(result.is_ok());
    assert_eq!(out, "<div>ok</div>");
}

#[test]
fn test_img_uses_double_quotes_for_alt() {
    let el = init(Tags::Img);
    let r = el.render_compact();
    // alt должен использовать двойные кавычки, как все остальные атрибуты
    assert!(r.contains(r#"alt="""#));
    assert!(!r.contains("alt=''"));
}

#[test]
fn test_svg_uses_double_quotes_for_namespaces() {
    let el = init(Tags::Svg);
    let r = el.render_compact();
    // xmlns должен использовать двойные кавычки
    assert!(r.contains(r#"xmlns="http://www.w3.org/2000/svg""#));
    assert!(r.contains(r#"xmlns:xlink="http://www.w3.org/1999/xlink""#));
    // не должно быть одинарных кавычек для атрибутов
    assert!(!r.contains("xmlns='"));
}

#[test]
fn test_attr_style_consistency() {
    // все встроенные и пользовательские атрибуты используют двойные кавычки
    let el = init(Tags::Div)
        .attr("title", "test")
        .class("container")
        .id("app");

    let r = el.render_compact();
    assert!(r.contains(r#"title="test""#));
    assert!(r.contains(r#"class="container""#));
    assert!(r.contains(r#"id="app""#));
}

#[test]
fn test_href_shortcut() {
    let el = init(Tags::A).href("https://example.com").text("Link");
    let r = el.render_compact();
    assert!(r.contains(r#"href="https://example.com""#));
}

#[test]
fn test_src_shortcut() {
    let el = init(Tags::Img).src("image.png");
    let r = el.render_compact();
    assert!(r.contains(r#"src="image.png""#));
}

#[test]
fn test_type_shortcut() {
    let el = init(Tags::Input).type_("text");
    let r = el.render_compact();
    assert!(r.contains(r#"type="text""#));
}

#[test]
fn test_for_shortcut() {
    let el = init(Tags::Label).for_("username").text("Username");
    let r = el.render_compact();
    assert!(r.contains(r#"for="username""#));
}

#[test]
fn test_attr_shortcuts_escape_values() {
    let el = init(Tags::A).href("javascript:alert(\"xss\")");
    let r = el.render_compact();
    assert!(r.contains("javascript:alert(&quot;xss&quot;)"));
    assert!(!r.contains(r#"javascript:alert("xss")"#));
}

#[test]
fn test_pre_preserves_whitespace_in_pretty() {
    let el = init(Tags::Pre).text("line 1\n  line 2\n    line 3");
    let r = el.render_pretty();

    // <pre> должен сохранить переносы и пробелы без добавления отступов
    assert!(r.contains("<pre>line 1\n  line 2\n    line 3</pre>"));
    // не должно быть лишних отступов перед каждой строкой
    assert!(!r.contains("\n    line 2\n      line 3"));
}

#[test]
fn test_pre_compact_preserves_whitespace() {
    let el = init(Tags::Pre).text("  spaced\n  text");
    let r = el.render_compact();
    assert_eq!(r, "<pre>  spaced\n  text</pre>");
}

#[test]
fn test_pre_with_children() {
    let pre = init(Tags::Pre);
    pre.code().text("fn main() {}");
    let r = pre.render_compact();

    assert!(r.contains("<pre>"));
    assert!(r.contains("<code>fn main() {}</code>"));
    assert!(r.contains("</pre>"));
}

#[test]
fn test_code_renders_correctly() {
    let el = init(Tags::Code).text("let x = 1;");
    let r = el.render_compact();
    assert_eq!(r, "<code>let x = 1;</code>");
}

#[test]
fn test_code_escapes_html() {
    let el = init(Tags::Code).text("<b>bold</b>");
    let r = el.render_compact();
    assert_eq!(r, "<code>&lt;b&gt;bold&lt;/b&gt;</code>");
}

#[test]
fn test_pre_escapes_html() {
    let el = init(Tags::Pre).text("<script>alert(1)</script>");
    let r = el.render_compact();
    assert_eq!(r, "<pre>&lt;script&gt;alert(1)&lt;/script&gt;</pre>");
}

#[test]
fn test_tpl_render_mixed_into_mismatched_args() {
    use domlink::{SafeHtml, TplArg};

    // 2 плейсхолдера, 1 аргумент — пропущенный просто игнорируется
    let tpl = Tpl::new("<p>{} {}</p>");
    let mut out = String::new();
    tpl.render_mixed_into(&mut out, &[TplArg::Text("only")])
        .unwrap();

    assert_eq!(out, "<p>only </p>");

    // 2 плейсхолдера, 3 аргумента — лишний игнорируется
    let safe = SafeHtml::new_unchecked("<x/>".to_string());
    let mut out2 = String::new();
    tpl.render_mixed_into(
        &mut out2,
        &[TplArg::Text("a"), TplArg::Text("b"), TplArg::Html(&safe)],
    )
    .unwrap();

    assert_eq!(out2, "<p>a b</p>");
}
