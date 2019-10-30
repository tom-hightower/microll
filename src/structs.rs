use std::collections::HashMap;

pub struct State {
    pub show_app_main_menu_bar: bool,
    pub file_menu: FileMenuState,
    pub url_to_get: String,
    pub root_url: String,
    pub main_body_array: Vec<RenderItem>,
    pub sub_windows: SubWindowVisibility,
    pub window_title: String,
    pub history: HashMap<String, WebpageFinder>,
    pub preloaded_pages: HashMap<String, String>,
}

impl<'a> Default for State {
    fn default() -> Self {
        State {
            show_app_main_menu_bar: true,
            file_menu: Default::default(),
            url_to_get: string!("https://adriann.github.io/rust_parser.html"),
            root_url: string!(""),
            main_body_array: vec![RenderItem::new()],
            sub_windows: Default::default(),
            window_title: String::from("Microll"),
            history: hashmap![string!("Microll: The Text-Based Web Browser") => WebpageFinder::create(WebpageType::Preload, string!("microll"))],
            preloaded_pages: hashmap![string!("microll") => string!("microll.html")],
        }
    }
}

pub struct FileMenuState {
    pub test_enabled: bool,
    pub can_search: bool,
    pub file_to_get: String,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            test_enabled: true,
            can_search: true,
            file_to_get: String::from("E:\\Programming\\microll\\src\\test.html"),
        }
    }
}

pub struct SubWindowVisibility {
    pub go_to_link: bool,
}

impl Default for SubWindowVisibility {
    fn default() -> Self {
        SubWindowVisibility { go_to_link: true }
    }
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
    pub url: String,
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
            url: String::new(),
            title: 0,
            code: 0,
        }
    }
}
