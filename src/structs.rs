use std::collections::HashMap;

pub struct State {
    pub file_menu: FileMenuState,
    pub url_to_get: String,
    pub root_url: String,
    pub main_body_array: Vec<RenderItem>,
    pub sub_windows: SubWindowVisibility,
    pub window_title: String,
    pub history: HashMap<String, WebpageFinder>,
    pub preloaded_pages: HashMap<String, String>,
    pub current_page: Option<WebpageFinder>,
    pub back_stack: Vec<WebpageFinder>,
    pub current_raw_html: String,
    pub search: SearchState,
}

impl Default for State {
    fn default() -> Self {
        State {
            file_menu: Default::default(),
            url_to_get: String::from("https://adriann.github.io/rust_parser.html"),
            root_url: String::new(),
            main_body_array: vec![RenderItem::new()],
            sub_windows: Default::default(),
            window_title: String::from("Microll"),
            history: HashMap::from([(
                String::from("Microll: The Text-Based Web Browser"),
                WebpageFinder::create(WebpageType::Preload, String::from("microll")),
            )]),
            preloaded_pages: HashMap::from([(
                String::from("microll"),
                String::from(include_str!("microll.html")),
            )]),
            current_page: None,
            back_stack: Vec::new(),
            current_raw_html: String::new(),
            search: SearchState::default(),
        }
    }
}

pub struct FileMenuState {
    pub can_search: bool,
    pub file_to_get: String,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            can_search: true,
            file_to_get: String::from("src/test.html"),
        }
    }
}

#[derive(Default)]
pub struct SubWindowVisibility {
    pub go_to_link: bool,
    pub search: bool,
    pub document_info: bool,
    pub help: bool,
    pub show_raw_html: bool,
}

#[derive(Default)]
pub struct SearchState {
    pub query: String,
    pub matches: Vec<usize>,
    pub current: usize,
}


#[derive(PartialEq, Debug, Clone)]
pub enum WebpageType {
    Link,    // http://mypage.com
    File,    // C:/User/username/Documents/test.html
    Preload, // microll
}

#[derive(Clone)]
pub struct WebpageFinder {
    pub web_type: WebpageType,
    pub location: String,
}

impl WebpageFinder {
    pub fn create(web_type: WebpageType, location: String) -> Self {
        WebpageFinder { web_type, location }
    }
}

/*
 * HTML Parsing Structs and Enums
*/

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum HTMLToken {
    Root,
    DocType,       // !Doctype
    HyperLink,     //a
    BoldText,      //b or strong
    Body,          //body
    LineBreak,     //br, wbr, or hr
    Code,          //code
    Preformatted,  //pre
    DivSection,    //div
    Head,          //head
    HtmlStart,     //html
    ItalicText,    //i or em
    ListItem,      //li
    OrderedList,   //ol
    Paragraph,     //p
    Script,        //script
    Style,         //style
    Span,          //span
    PageTitle,     //title
    UnorderedList, //ul
    Comment,       //<!--***-->
    Void,          // no closing tag
    Text,          // Text-only
    Unknown,
    Heading(u8),   //h1-h6, level 1-6

    // Tables
    Table,             //table
    TableHead,         //thead
    TableBody,         //tbody
    TableFoot,         //tfoot
    TableRow,          //tr
    TableCell,         //td
    TableHeaderCell,   //th
    TableCaption,      //caption
    TableColumnGroup,  //colgroup

    // Forms
    Form,          //form
    Button,        //button
    Select,        //select
    Option,        //option
    OptionGroup,   //optgroup
    TextArea,      //textarea
    Label,         //label
    FieldSet,      //fieldset
    Legend,        //legend

    BlockQuote,    //blockquote

    // HTML5 semantic sectioning
    Nav,           //nav
    PageHeader,    //header
    PageFooter,    //footer
    Main,          //main
    Section,       //section
    Article,       //article
    Aside,         //aside
    Figure,        //figure
    FigureCaption, //figcaption

    // Raw text
    IFrame,        //iframe
    NoScript,      //noscript
    RawText,       //xmp, noembed, or noframes
    Template,      //template

    // Inline text-level semantics
    SmallText,     //small
    Mark,          //mark
    Subscript,     //sub
    Superscript,   //sup
    Deleted,       //del
    Inserted,      //ins
    Underline,     //u
    Strikethrough, //s
    Abbreviation,  //abbr
    Citation,      //cite
    KeyboardInput, //kbd
    SampleOutput,  //samp

    // Definition lists
    DescriptionList,    //dl
    DescriptionTerm,    //dt
    DescriptionDetails, //dd

    Details, //details
    Summary, //summary
    Dialog,  //dialog

    // other?
    Canvas,          //canvas
    Svg,             //svg
    Time,            //time
    Data,            //data
    Variable,        //var
    Ruby,            //ruby
    RubyText,        //rt
    RubyParenthesis, //rp
    Address,         //address
}

pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub tag: HTMLToken,
    pub attributes: HashMap<String, String>,
    pub text: Vec<u8>,
    pub start_ind: usize,
    pub end_ind: usize,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            tag: HTMLToken::Unknown,
            attributes: HashMap::new(),
            text: Vec::new(),
            start_ind: 0,
            end_ind: 0,
        }
    }
    pub fn create(
        children: Vec<ParseNode>,
        tag: HTMLToken,
        attributes: HashMap<String, String>,
        text: Vec<u8>,
        start_ind: usize,
        end_ind: usize,
    ) -> ParseNode {
        ParseNode {
            children,
            tag,
            attributes,
            text,
            start_ind,
            end_ind,
        }
    }
}

pub struct RenderItem {
    pub bold: bool,
    pub heading_level: Option<u8>,
    pub italics: bool,
    pub link: bool,
    pub url: String,
    pub line_break: bool,
    pub title: bool,
    pub code: bool,
    pub text: String,
    pub block: Option<BlockBoundary>,
}

impl RenderItem {
    pub fn new() -> RenderItem {
        RenderItem {
            bold: false,
            heading_level: None,
            italics: false,
            link: false,
            url: String::new(),
            line_break: false,
            title: false,
            code: false,
            text: String::new(),
            block: None,
        }
    }

    pub fn line_break() -> RenderItem {
        let mut item = RenderItem::new();
        item.line_break = true;
        item
    }

    pub fn block(boundary: BlockBoundary) -> RenderItem {
        let mut item = RenderItem::new();
        item.block = Some(boundary);
        item
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElementStyle {
    pub display: Display,
    pub spacing_before: f32,
    pub indents: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Display {
    Inline,
    Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockBoundary {
    pub spacing_before: f32,
    pub indent: u8,
    pub marker: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MarkerKind {
    Bullet,
    Ordinal,
}

#[derive(Clone)]
pub struct RenderContext {
    pub bold: bool,
    pub italics: bool,
    pub code: bool,
    pub link: Option<String>,
    pub heading_level: Option<u8>,
    pub in_title: bool,
    pub indent: u8,
    pub list_kind: Option<MarkerKind>,
}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {
            bold: false,
            italics: false,
            code: false,
            link: None,
            heading_level: None,
            in_title: false,
            indent: 0,
            list_kind: None,
        }
    }
}
