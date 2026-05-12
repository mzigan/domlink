use crate::link::Link;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Tags {
    Html,
    Head,
    Body,
    Svg,
    Path,
    #[default]
    Div,
    Table,
    Thead,
    Tbody,
    Tr,
    Td,
    Form,
    Iframe,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Ol,
    Ul,
    Li,
    Br,
    Span,
    Img,
    A,
    Button,
    Input,
    Textarea,
    Select,
    Opt, // Переименован из Option
    Meta,
    Label,
    Title,
    Link,
    Script,
    Style,
    Any,
}

impl Tags {
    pub(super) fn opening_tag(&self) -> &'static str {
        match self {
            Tags::Html => "<html",
            Tags::Head => "<head",
            Tags::Meta => "<meta",
            Tags::Link => "<link",
            Tags::Body => "<body",
            Tags::Svg => {
                "<svg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'"
            }
            Tags::Path => "<path",
            Tags::Div => "<div",
            Tags::Form => "<form",
            Tags::Table => "<table",
            Tags::Tbody => "<tbody",
            Tags::Thead => "<thead",
            Tags::Tr => "<tr",
            Tags::Td => "<td",
            Tags::Iframe => "<iframe",
            Tags::Ol => "<ol",
            Tags::Ul => "<ul",
            Tags::Li => "<li",
            Tags::Span => "<span",
            Tags::Img => "<img alt=''",
            Tags::Br => "<br",
            Tags::A => "<a",
            Tags::P => "<p",
            Tags::H1 => "<h1",
            Tags::H2 => "<h2",
            Tags::H3 => "<h3",
            Tags::H4 => "<h4",
            Tags::H5 => "<h5",
            Tags::H6 => "<h6",
            Tags::Label => "<label",
            Tags::Button => "<button",
            Tags::Input => "<input",
            Tags::Textarea => "<textarea",
            Tags::Select => "<select",
            Tags::Opt => "<option",
            Tags::Title => "<title",
            Tags::Script => "<script",
            Tags::Style => "<style",
            Tags::Any => "",
        }
    }

    pub(super) fn closing_tag(&self) -> Option<&'static str> {
        match self {
            // Void-элементы и фрагменты не имеют закрывающего тега
            Tags::Meta | Tags::Link | Tags::Img | Tags::Br | Tags::Input | Tags::Any => None,

            // Остальные теги явно перечислены
            Tags::Html => Some("</html>"),
            Tags::Head => Some("</head>"),
            Tags::Body => Some("</body>"),
            Tags::Svg => Some("</svg>"),
            Tags::Path => Some("</path>"),
            Tags::Div => Some("</div>"),
            Tags::Form => Some("</form>"),
            Tags::Table => Some("</table>"),
            Tags::Tbody => Some("</tbody>"),
            Tags::Thead => Some("</thead>"),
            Tags::Tr => Some("</tr>"),
            Tags::Td => Some("</td>"),
            Tags::Iframe => Some("</iframe>"),
            Tags::Ol => Some("</ol>"),
            Tags::Ul => Some("</ul>"),
            Tags::Li => Some("</li>"),
            Tags::Span => Some("</span>"),
            Tags::A => Some("</a>"),
            Tags::P => Some("</p>"),
            Tags::H1 => Some("</h1>"),
            Tags::H2 => Some("</h2>"),
            Tags::H3 => Some("</h3>"),
            Tags::H4 => Some("</h4>"),
            Tags::H5 => Some("</h5>"),
            Tags::H6 => Some("</h6>"),
            Tags::Label => Some("</label>"),
            Tags::Button => Some("</button>"),
            Tags::Textarea => Some("</textarea>"),
            Tags::Select => Some("</select>"),
            Tags::Opt => Some("</option>"),
            Tags::Title => Some("</title>"),
            Tags::Script => Some("</script>"),
            Tags::Style => Some("</style>"),
        }
    }

    pub(super) fn is_void(&self) -> bool {
        matches!(
            self,
            Tags::Meta | Tags::Link | Tags::Img | Tags::Br | Tags::Input
        )
    }
}

macro_rules! impl_tag_shortcuts {
    ($($method:ident => $tag:expr),* $(,)?) => {
        #[allow(dead_code)]
        impl Link {
            $(
                pub fn $method(&self) -> Link {
                    self.append($tag)
                }
            )*
        }
    };
}

impl_tag_shortcuts! {
    html => Tags::Html,
    head => Tags::Head,
    title => Tags::Title,
    meta => Tags::Meta,
    link => Tags::Link,
    body => Tags::Body,
    div => Tags::Div,
    h1 => Tags::H1,
    h2 => Tags::H2,
    h3 => Tags::H3,
    h4 => Tags::H4,
    h5 => Tags::H5,
    h6 => Tags::H6,
    p => Tags::P,
    span => Tags::Span,
    a => Tags::A,
    button => Tags::Button,
    form => Tags::Form,
    input => Tags::Input,
    textarea => Tags::Textarea,
    select => Tags::Select,
    opt => Tags::Opt,
    table => Tags::Table,
    thead => Tags::Thead,
    tbody => Tags::Tbody,
    tr => Tags::Tr,
    td => Tags::Td,
    ul => Tags::Ul,
    ol => Tags::Ol,
    li => Tags::Li,
    img => Tags::Img,
    svg => Tags::Svg,
    path => Tags::Path,
    script => Tags::Script,
    style => Tags::Style,
    br => Tags::Br,
    label => Tags::Label,
    iframe => Tags::Iframe,
    any => Tags::Any,
}
