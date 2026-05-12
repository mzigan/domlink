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
    // или оба появятся — важно знать поведение
    let el = init(Tags::Div).id("first").id("second");
    let r = el.render();
    // документируем факт: два id в атрибутах
    assert_eq!(r.matches("id=").count(), 2);
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
