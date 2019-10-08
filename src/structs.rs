use imgui::ImString;
use std::collections::HashMap;

pub struct State {
    pub show_app_main_menu_bar: bool,
    pub file_menu: FileMenuState,
    pub url_to_get: ImString,
    pub main_body_array: Vec<RenderItem>,
    pub sub_windows: SubWindowVisibility,
}

impl Default for State {
    fn default() -> Self {
        State {
            show_app_main_menu_bar: true,
            file_menu: Default::default(),
            url_to_get: ImString::new("https://adriann.github.io/rust_parser.html"),
            main_body_array: vec![RenderItem::new()],
            sub_windows: Default::default(),
        }
    }
}

pub struct FileMenuState {
    pub test_enabled: bool,
    pub can_search: bool,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            test_enabled: true,
            can_search: true,
        }
    }
}

pub struct SubWindowVisibility {
    pub go_to_link: bool,
}

impl Default for SubWindowVisibility {
    fn default() -> Self {
        SubWindowVisibility { go_to_link: false }
    }
}

/*
 * HTML Parsing Structs and Enums
*/

#[derive(PartialEq, Debug)]
pub enum HTMLToken {
    ROOT,
    DocType,       // !Doctype
    HyperLink,     //a
    BoldText,      //b or strong
    Body,          //body
    LineBreak,     //br, wbr, or hr
    Code,          //code
    DivSection,    //div
    Head,          //head
    Heading,       //heading
    HtmlStart,     //html
    ItalicText,    //i or em
    ListItem,      //li
    OrderedList,   //ol
    Paragraph,     //p
    Script,        //script
    Span,          //span
    PageTitle,     //title
    UnorderedList, //ul
    Comment,       //<!--***-->
    VOID,          // no closing tag
    Text,          // Text-only
    Unknown,
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
}

pub struct RenderItem {
    pub bold: bool,
    pub heading: bool,
    pub italics: bool,
    pub link: bool,
    pub url: String,
    pub line_break: bool,
    pub title: bool,
    pub code: bool,
    pub text: String,
}

impl RenderItem {
    pub fn new() -> RenderItem {
        RenderItem {
            bold: false,
            heading: false,
            italics: false,
            link: false,
            url: String::new(),
            line_break: false,
            title: false,
            code: false,
            text: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct RenderState {
    pub bold: usize,
    pub heading: usize,
    pub italics: usize,
    pub link: usize,
    pub title: usize,
    pub code: usize,
}

impl RenderState {
    pub fn new() -> RenderState {
        RenderState {
            bold: 0,
            heading: 0,
            italics: 0,
            link: 0,
            title: 0,
            code: 0,
        }
    }
}
