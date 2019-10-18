use crate::structs::HTMLToken;
use crate::structs::ParseNode;
use crate::structs::RenderItem;
use crate::structs::RenderState;
use std::collections::HashMap;
use std::str;

fn larse(input_u8: &Vec<u8>, begin: usize) -> Result<Vec<ParseNode>, String> {
    let mut result = Vec::new();
    let mut i: usize = begin;
    'outer: while i < input_u8.len() {
        match input_u8[i] as char {
            '<' => {
                i += 1;
                i = eat_whitespace(input_u8, i, Some(true));
                match input_u8[i] as char {
                    'A'..='Z' | 'a'..='z' | '!' => {
                        let mut tag: Vec<u8> = Vec::<u8>::new();
                        let mut buf_pos: usize = i;
                        let start: usize = i - 1;
                        while !is_whitespace(input_u8[buf_pos])
                            && (input_u8[buf_pos] != '>' as u8)
                            && (input_u8[buf_pos] != '/' as u8)
                        {
                            tag.push(input_u8[buf_pos]);
                            buf_pos += 1;
                        }
                        let html_tag = match_tag(tag.clone());
                        let mut attributes: HashMap<String, String> = HashMap::new();
                        if (input_u8[buf_pos] != '>' as u8)
                            && (html_tag != HTMLToken::Unknown)
                            && (html_tag != HTMLToken::Comment)
                            && (html_tag != HTMLToken::DocType)
                        {
                            // Find tag attributes
                            let mut attr_name: String = string!("");
                            let mut attr_val: String = string!("");
                            let mut is_name: bool = true;
                            buf_pos += 1;
                            while not_close_of_open(input_u8, buf_pos) {
                                match input_u8[buf_pos] as char {
                                    '=' => is_name = !is_name,
                                    ' ' | '\t' | '\r' | '\n' => {
                                        if is_quote(input_u8, buf_pos - 1) {
                                            attributes.insert(attr_name, attr_val);
                                            attr_name = string!("");
                                            attr_val = string!("");
                                            is_name = true;
                                        } else {
                                            if is_name {
                                                if attr_name != "".to_string() {
                                                    if attr_val.len() > 3 {
                                                        attributes.insert(
                                                            attr_name,
                                                            attr_val[1..(attr_val.len() - 1)]
                                                                .to_string(),
                                                        );
                                                    } else {
                                                        attributes.insert(attr_name, attr_val);
                                                    }
                                                    attr_name = string!("");
                                                    attr_val = string!("");
                                                }
                                            } else {
                                                attr_val.push(input_u8[buf_pos] as char);
                                            }
                                        }
                                    }
                                    _ => {
                                        if is_name {
                                            attr_name.push(input_u8[buf_pos] as char);
                                        } else {
                                            attr_val.push(input_u8[buf_pos] as char);
                                        }
                                    }
                                }
                                buf_pos += 1;
                            }
                            if attr_name != "".to_string() {
                                if attr_val.len() > 3 {
                                    attributes.insert(
                                        attr_name,
                                        attr_val[1..(attr_val.len() - 1)].to_string(),
                                    );
                                } else {
                                    attributes.insert(attr_name, attr_val);
                                }
                            }
                        }
                        if buf_pos < input_u8.len() {
                            if input_u8[buf_pos] == '/' as u8 || html_tag == HTMLToken::VOID {
                                // self closing tag
                                let node = ParseNode::create(
                                    Vec::new(),
                                    html_tag,
                                    attributes,
                                    Vec::new(),
                                    start,
                                    buf_pos + 1,
                                );
                                result.push(node);
                                i = buf_pos + 2;
                            } else if html_tag == HTMLToken::Comment {
                                while !is_closing_comment(input_u8, buf_pos) {
                                    buf_pos += 1;
                                }
                                i = buf_pos + 3;
                            } else if html_tag == HTMLToken::DocType {
                                // No closing tag, skip
                                while !(input_u8[buf_pos] == '>' as u8) {
                                    buf_pos += 1
                                }
                                i = buf_pos + 1
                            } else if html_tag == HTMLToken::LineBreak {
                                // No closing tag, cannot have children
                                let node = ParseNode::create(
                                    Vec::new(),
                                    html_tag,
                                    attributes,
                                    Vec::new(),
                                    start,
                                    buf_pos,
                                );
                                result.push(node);
                                i = buf_pos + 1;
                            } else {
                                //close this bad boi and look for text contents/nested tags
                                buf_pos += 1;
                                let mut text = Vec::<u8>::new();
                                let mut children = Vec::<ParseNode>::new();
                                buf_pos = eat_whitespace(input_u8, buf_pos, Some(true));
                                let mut start_text: usize = buf_pos;
                                while !is_closing_tag(input_u8, buf_pos, &tag) {
                                    if input_u8[buf_pos] != '<' as u8
                                        || html_tag == HTMLToken::Script
                                    {
                                        text.push(input_u8[buf_pos]);
                                    } else {
                                        if text != Vec::<u8>::new() {
                                            let node = ParseNode::create(
                                                Vec::new(),
                                                HTMLToken::Text,
                                                HashMap::new(),
                                                text,
                                                start_text,
                                                buf_pos - 1,
                                            );
                                            children.push(node);
                                            text = Vec::<u8>::new();
                                        }
                                        match larse(&input_u8, buf_pos) {
                                            Ok(new_childs) => {
                                                
                                                for child in new_childs {
                                                    children.push(child);
                                                }
                                            }
                                            Err(e) => println!("{}", e),
                                        }
                                        if children.len() != 0 {
                                            buf_pos = children[children.len() - 1].end_ind;
                                            start_text = buf_pos;
                                        }
                                    }
                                    buf_pos += 1;
                                }
                                if text != "".as_bytes().to_vec() {
                                    let node = ParseNode::create(
                                        Vec::new(),
                                        HTMLToken::Text,
                                        HashMap::new(),
                                        text,
                                        start_text,
                                        buf_pos - 1,
                                    );
                                    children.push(node);
                                }
                                while input_u8[buf_pos] != '>' as u8 {
                                    buf_pos += 1
                                }
                                let node = ParseNode::create(
                                    children,
                                    html_tag,
                                    attributes,
                                    Vec::new(),
                                    start,
                                    buf_pos,
                                );
                                result.push(node);
                                i = buf_pos
                            }
                        } else {
                            // end of file
                            let node = ParseNode::create(
                                Vec::new(),
                                html_tag,
                                HashMap::new(),
                                Vec::new(),
                                start,
                                input_u8.len(),
                            );
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
            '\n' | ' ' | '\t' | '\r' => {}
            _ => {
                break 'outer;
            }
        }
        i += 1;
    }
    Ok(result)
}

fn match_tag(tag: Vec<u8>) -> HTMLToken {
    let tag_str = str::from_utf8(&tag).unwrap_or_else(|e| {
        panic!("Invalid UTF-8 sequence: {}", e);
    });
    match tag_str.to_uppercase().as_str() {
        "HTML" => return HTMLToken::HtmlStart,
        "HEAD" => return HTMLToken::Head,
        "A" => return HTMLToken::HyperLink,
        "B" | "STRONG" => return HTMLToken::BoldText,
        "BODY" => return HTMLToken::Body,
        "BR" | "WBR" | "HR" => return HTMLToken::LineBreak,
        "CODE" => return HTMLToken::Code,
        "DIV" => return HTMLToken::DivSection,
        "H1" | "H2" | "H3" | "H4" | "H5" | "H6" => return HTMLToken::Heading,
        "I" | "EM" => return HTMLToken::ItalicText,
        "LI" => return HTMLToken::ListItem,
        "OL" => return HTMLToken::OrderedList,
        "P" => return HTMLToken::Paragraph,
        "SCRIPT" => return HTMLToken::Script,
        "SPAN" => return HTMLToken::Span,
        "TITLE" => return HTMLToken::PageTitle,
        "UL" => return HTMLToken::UnorderedList,
        "!--" => return HTMLToken::Comment,
        "!DOCTYPE" => return HTMLToken::DocType,
        "AREA" | "BASE" | "COL" | "COMMAND" | "EMBED" | "IMG" | "INPUT" | "KEYGEN" | "LINK"
        | "META" | "PARAM" | "SOURCE" | "TRACK" | "PATH" => return HTMLToken::VOID,
        _ => return HTMLToken::Unknown,
    }
}

pub fn parse_html(html: &String) -> (Vec<RenderItem>, String) {
    let mut root = ParseNode::new();
    root.tag = HTMLToken::ROOT;
    root.start_ind = 0;
    root.end_ind = html.len();
    let html_vec = html.as_bytes().to_vec();
    let result;
    match larse(&html_vec, 0) {
        Ok(children) => {
            result = children;
            for node in result {
                root.children.push(node);
            }
        }
        Err(e) => println!("{}", e),
    }
    let cur_state = RenderState::new();
    return build_array(root, Vec::new(), cur_state, string!(""));
}

fn build_array(
    node: ParseNode,
    mut ret_vec: Vec<RenderItem>,
    mut cur_state: RenderState,
    title: String,
) -> (Vec<RenderItem>, String) {
    let mut cur_title = title;
    for i in node.children {
        match i.tag {
            HTMLToken::BoldText => {
                cur_state.bold = i.end_ind;
            }
            HTMLToken::HyperLink => {
                cur_state.link = i.end_ind;
                match i.attributes.get("href") {
                    Some(link) => cur_state.url = link.clone(),
                    None => {}
                }
            }
            HTMLToken::LineBreak => {
                let mut break_item = RenderItem::new();
                break_item.line_break = true;
                ret_vec.push(break_item);
            }
            HTMLToken::Code => {
                cur_state.code = i.end_ind;
            }
            HTMLToken::ItalicText => {
                cur_state.italics = i.end_ind;
            }
            HTMLToken::PageTitle => {
                cur_state.title = i.end_ind;
            }
            HTMLToken::Heading => {
                cur_state.heading = i.end_ind;
            }
            HTMLToken::Text => {
                if !(node.tag == HTMLToken::Unknown || node.tag == HTMLToken::Script)
                    && !is_text_whitespace(&i.text)
                {
                    let mut item = RenderItem::new();
                    let text = String::from_utf8(i.text.clone());
                    if text.is_ok() {
                        item.text = text.unwrap_or_else(|e| {
                            println!("{}", e);
                            string!("")
                        });
                    }
                    if i.end_ind < cur_state.bold {
                        item.bold = true;
                    }
                    if i.end_ind < cur_state.link {
                        item.link = true;
                        item.url = cur_state.url.clone();
                    }
                    if i.end_ind < cur_state.code {
                        item.code = true;
                    }
                    if i.end_ind < cur_state.italics {
                        item.italics = true;
                    }
                    if i.end_ind < cur_state.title {
                        item.title = true;
                        cur_title = item.text.clone();
                    }
                    if i.end_ind < cur_state.heading {
                        item.heading = true;
                    }
                    ret_vec.push(item);
                }
            }
            _ => {}
        }
        let ret = build_array(i, ret_vec, cur_state.clone(), cur_title.clone());
        ret_vec = ret.0;
        cur_title = ret.1;
    }
    return (ret_vec, cur_title);
}

fn eat_whitespace(input_u8: &Vec<u8>, mut buf_pos: usize, incl_space: Option<bool>) -> usize {
    match incl_space {
        Some(true) => {
            while is_whitespace(input_u8[buf_pos]) {
                buf_pos += 1;
            }
        }
        _ => {
            while is_newline_or_tab(input_u8[buf_pos]) {
                buf_pos += 1;
            }
        }
    }
    buf_pos
}

fn is_text_whitespace(text: &Vec<u8>) -> bool {
    (text.len() > 0) && (text.len() < 2) && is_whitespace(text[0])
}

fn is_whitespace(c: u8) -> bool {
    match c as char {
        '\n' | ' ' | '\t' | '\r' => true,
        _ => false,
    }
}

fn is_newline_or_tab(c: u8) -> bool {
    match c as char {
        '\n' | '\r' | '\t' => true,
        _ => false,
    }
}

fn is_closing_tag(input_u8: &Vec<u8>, buf_pos: usize, tag: &Vec<u8>) -> bool {
    input_u8[buf_pos] == '<' as u8
        && input_u8[buf_pos + 1] == '/' as u8
        && (input_u8[buf_pos + 2..buf_pos + 2 + tag.len()].to_ascii_uppercase()
            == tag.to_ascii_uppercase())
}

fn is_closing_comment(input_u8: &Vec<u8>, buf_pos: usize) -> bool {
    (input_u8[buf_pos] == '-' as u8
        && input_u8[buf_pos + 1] == '-' as u8
        && input_u8[buf_pos + 2] == '>' as u8)
}

fn is_quote(input_u8: &Vec<u8>, buf_pos: usize) -> bool {
    input_u8[buf_pos] == '\"' as u8 || input_u8[buf_pos] == '\'' as u8
}

fn not_close_of_open(input_u8: &Vec<u8>, buf_pos: usize) -> bool {
    buf_pos < input_u8.len()
        && (input_u8[buf_pos] != '>' as u8
            && !(input_u8[buf_pos] == '/' as u8 && input_u8[buf_pos + 1] == '>' as u8))
}
