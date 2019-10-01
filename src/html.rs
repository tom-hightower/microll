use scraper::Html;
use std::collections::HashMap;
use std::str;

#[derive(PartialEq, Debug)]
enum HTMLToken {
    ROOT,
    DocType,       // !Doctype
    HyperLink,     //a
    BoldText,      //b or strong
    Body,          //body
    LineBreak,     //br, wbr, or hr
    DivSection,    //div
    Head,          //head
    Heading,       //heading
    HtmlStart,     //html
    ItalicText,    //i or em
    ListItem,      //li
    OrderedList,   //ol
    Paragraph,     //p
    Span,          //span
    PageTitle,     //title
    UnorderedList, //ul
    Comment,       //<!--***-->
    VOID,          // no closing tag
    Unknown,
}

struct ParseNode {
    children: Vec<ParseNode>,
    tag: HTMLToken,
    attributes: HashMap<String, String>,
    text: Vec<u8>,
    start_ind: usize,
    end_ind: usize,
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

fn larse(input_u8: Vec<u8>, begin: usize) -> Result<Vec<ParseNode>, String> {
    let mut result = Vec::new();
    let mut i = begin;
    'outer: while i < input_u8.len() {
        match input_u8[i] as char {
            '<' => {
                match input_u8[i + 1] as char {
                    'A'..='Z' | 'a'..='z' | '!' => {
                        let mut tag: Vec<u8> = Vec::<u8>::new();
                        let mut x: usize = i + 1;
                        let start: usize = i;
                        while (input_u8[x] != ' ' as u8)
                            && (input_u8[x] != '>' as u8)
                            && (input_u8[x] != '/' as u8)
                        {
                            tag.push(input_u8[x]);
                            x += 1;
                        }
                        let html_tag = match_tag(tag.clone());
                        let mut attributes: HashMap<String, String> = HashMap::new();
                        if (input_u8[x] != '>' as u8)
                            && (html_tag != HTMLToken::Unknown)
                            && (html_tag != HTMLToken::Comment)
                            && (html_tag != HTMLToken::DocType)
                        {
                            // Find tag attributes
                            let mut attr_name: String = String::new();
                            let mut attr_val: String = String::new();
                            let mut is_name: bool = true;
                            x += 1;
                            while x < input_u8.len()
                                && (input_u8[x] as char != '>'
                                    && !(input_u8[x] as char == '/'
                                        && input_u8[x + 1] as char == '>'))
                            {
                                match input_u8[x] as char {
                                    '=' => is_name = !is_name,
                                    ' ' => {
                                        if input_u8[x - 1] as char == '\"'
                                            || input_u8[x - 1] as char == '\''
                                        {
                                            attributes.insert(attr_name, attr_val);
                                            attr_name = String::from("");
                                            attr_val = String::from("");
                                            is_name = true;
                                        } else {
                                            if is_name {
                                                if attr_name != "".to_string() {
                                                    attributes.insert(attr_name, attr_val);
                                                    attr_name = String::from("");
                                                    attr_val = String::from("");
                                                }
                                            } else {
                                                attr_val.push(input_u8[x] as char);
                                            }
                                        }
                                    }
                                    _ => {
                                        if is_name {
                                            attr_name.push(input_u8[x] as char);
                                        } else {
                                            attr_val.push(input_u8[x] as char);
                                        }
                                    }
                                }
                                x += 1;
                            }
                            if attr_name != "".to_string() {
                                attributes.insert(attr_name, attr_val);
                            }
                        }
                        if x < input_u8.len() {
                            if input_u8[x] as char == '/' {
                                // self closing tag
                                let mut node = ParseNode::new();
                                node.start_ind = start;
                                node.end_ind = x + 1;
                                node.tag = html_tag;
                                node.attributes = attributes;
                                result.push(node);
                                i = x + 2;
                            } else if html_tag == HTMLToken::Comment {
                                while !(input_u8[x] as char == '-'
                                    && input_u8[x + 1] as char == '-'
                                    && input_u8[x + 2] as char == '>')
                                {
                                    x += 1;
                                }
                                i = x + 3;
                            } else if html_tag == HTMLToken::DocType || html_tag == HTMLToken::VOID
                            {
                                // No closing tag, skip
                                i = x + 1
                            } else if html_tag == HTMLToken::LineBreak {
                                // No closing tag, cannot have children
                                let mut node = ParseNode::new();
                                node.start_ind = start;
                                node.end_ind = x;
                                node.tag = html_tag;
                                node.attributes = attributes;
                                result.push(node);
                                i = x + 1;
                            } else {
                                //close this bad boi and look for text contents/nested tags
                                x += 1;
                                let mut text = Vec::<u8>::new();
                                let mut children = Vec::<ParseNode>::new();
                                while !(input_u8[x] as char == '<'
                                    && input_u8[x + 1] as char == '/'
                                    && (input_u8[x + 2..x + 2 + tag.len()].to_ascii_uppercase()
                                        == tag.to_ascii_uppercase()))
                                {
                                    if input_u8[x] as char != '<' {
                                        text.push(input_u8[x]);
                                    } else {
                                        let new_children = larse(input_u8.clone(), x).unwrap();
                                        for child in new_children {
                                            children.push(child);
                                        }
                                        if children.len() != 0 {
                                            x = children[children.len() - 1].end_ind;
                                        }
                                    }
                                    x += 1;
                                }
                                let mut node = ParseNode::new();
                                node.start_ind = start;
                                while input_u8[x] as char != '>' {
                                    x += 1
                                }
                                node.end_ind = x;
                                node.tag = html_tag;
                                node.attributes = attributes;
                                node.children = children;
                                node.text = text;
                                result.push(node);
                                i = x
                            }
                        } else {
                            // end of file
                            let mut node = ParseNode::new();
                            node.start_ind = start;
                            node.end_ind = input_u8.len();
                            node.tag = html_tag;
                            result.push(node);
                            break 'outer;
                        }
                    }
                    '/' => {
                        //reached the end of nested segment probably
                        break 'outer;
                    }
                    '\n' | ' ' | '\t' | '\r' => {}
                    _ => {
                        // shouldn't be here
                        break 'outer;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    Ok(result)
}

fn match_tag(tag: Vec<u8>) -> HTMLToken {
    let tag_str = match str::from_utf8(&tag) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    match tag_str.to_uppercase().as_str() {
        "HTML" => return HTMLToken::HtmlStart,
        "HEAD" => return HTMLToken::Head,
        "A" => return HTMLToken::HyperLink,
        "B" | "STRONG" => return HTMLToken::BoldText,
        "BODY" => return HTMLToken::Body,
        "BR" | "WBR" | "HR" => return HTMLToken::LineBreak,
        "DIV" => return HTMLToken::DivSection,
        "H1" | "H2" | "H3" | "H4" | "H5" | "H6" => return HTMLToken::Heading,
        "I" | "EM" => return HTMLToken::ItalicText,
        "LI" => return HTMLToken::ListItem,
        "OL" => return HTMLToken::OrderedList,
        "P" => return HTMLToken::Paragraph,
        "SPAN" => return HTMLToken::Span,
        "TITLE" => return HTMLToken::PageTitle,
        "UL" => return HTMLToken::UnorderedList,
        "!--" => return HTMLToken::Comment,
        "!DOCTYPE" => return HTMLToken::DocType,
        "AREA" | "BASE" | "COL" | "COMMAND" | "EMBED" | "IMG" | "INPUT" | "KEYGEN" | "LINK"
        | "META" | "PARAM" | "SOURCE" | "TRACK" => return HTMLToken::VOID,
        _ => return HTMLToken::Unknown,
    }
}

pub fn parse_html(html: &String) -> scraper::Html {
    let mut root = ParseNode::new();
    root.tag = HTMLToken::ROOT;
    root.start_ind = 0;
    root.end_ind = html.len();
    let html_vec = html.as_bytes().to_vec();

    let result = larse(html_vec, 0).unwrap();
    for node in result {
        root.children.push(node);
    }
    print_nodes(root);
    let document = Html::parse_document(html);
    return document;
}

fn print_nodes(node: ParseNode) {
    for i in node.children {
        println!("{:?}", i.tag);
        print_nodes(i);
    }
}

pub fn traverse_document(document: scraper::Html) -> Vec<String> {
    let mut elements: Vec<String> = Vec::new();
    for el in document.root_element().text() {
        elements.push(String::from(el));
    }

    return elements;
}

/*
pub fn get_elements(document: scraper::Html, key: &str) -> Vec<String> {
    let selector = Selector::parse(key).unwrap();
    let mut elements = Vec::new();
    for item in document.select(&selector) {
        elements.push(item.text().collect::<String>());
    }
    return elements;
}
*/
