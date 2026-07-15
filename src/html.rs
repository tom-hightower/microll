use crate::structs::HTMLToken;
use crate::structs::ParseNode;
use crate::structs::RenderItem;
use crate::structs::RenderState;
use std::collections::HashMap;
use std::str;

/// Maximum element nesting depth before '<' is treated as literal text.
const MAX_DEPTH: usize = 64;

/// Recursive descent lexer/parser ("larser").
fn larse(input_u8: &[u8], begin: usize, depth: usize, parent: &HTMLToken) -> (Vec<ParseNode>, usize) {
    let mut result = Vec::new();
    let mut i: usize = begin;
    while i < input_u8.len() {
        if is_whitespace(input_u8[i]) {
            i += 1;
            continue;
        }
        if input_u8[i] != b'<' {
            // belongs to caller
            break;
        }
        if has_implied_close(input_u8, i, parent) {
            // this tag implicitly closes the caller's element
            break;
        }
        let lt_pos = i;
        let name_pos = eat_whitespace(input_u8, i + 1);
        if name_pos >= input_u8.len() {
            i = input_u8.len();
            break;
        }
        match input_u8[name_pos] {
            b'/' => {
                // closing tag
                i = lt_pos;
                break;
            }
            b'A'..=b'Z' | b'a'..=b'z' | b'!' => {
                i = larse_element(input_u8, lt_pos, name_pos, depth, &mut result);
            }
            _ => {
                // stray '<' 
                i = lt_pos;
                break;
            }
        }
    }
    (result, i)
}

/// parses one element whose '<' is at `lt_pos` and tag name starts at
/// `name_pos`, pushing any produced nodes onto `result`
/// Returns the index of the first byte after the element.
fn larse_element(
    input_u8: &[u8],
    lt_pos: usize,
    name_pos: usize,
    depth: usize,
    result: &mut Vec<ParseNode>,
) -> usize {
    let len = input_u8.len();
    let mut tag: Vec<u8> = Vec::new();
    let mut buf_pos: usize = name_pos;
    // collect tag text and match it to a known tag
    while buf_pos < len
        && !is_whitespace(input_u8[buf_pos])
        && input_u8[buf_pos] != b'>'
        && input_u8[buf_pos] != b'/'
    {
        tag.push(input_u8[buf_pos]);
        buf_pos += 1;
    }
    let html_tag = match_tag(&tag);
    let mut attributes: HashMap<String, String> = HashMap::new();
    if buf_pos < len
        && input_u8[buf_pos] != b'>'
        && html_tag != HTMLToken::Comment
        && html_tag != HTMLToken::DocType
    {
        buf_pos = parse_attributes(input_u8, buf_pos, &mut attributes);
    }
    if buf_pos >= len {
        // input ended inside the tag
        let node = ParseNode::create(Vec::new(), html_tag, attributes, Vec::new(), lt_pos, len);
        result.push(node);
        return len;
    }
    if input_u8[buf_pos] == b'/' || html_tag == HTMLToken::Void {
        // self-closing/void tag
        let node = ParseNode::create(
            Vec::new(),
            html_tag,
            attributes,
            Vec::new(),
            lt_pos,
            buf_pos + 1,
        );
        result.push(node);
        skip_past_gt(input_u8, buf_pos)
    } else if html_tag == HTMLToken::Comment {
        while buf_pos < len && !is_closing_comment(input_u8, buf_pos) {
            buf_pos += 1;
        }
        (buf_pos + 3).min(len)
    } else if html_tag == HTMLToken::DocType {
        // no closing tag, skip
        skip_past_gt(input_u8, buf_pos)
    } else if html_tag == HTMLToken::LineBreak {
        // no closing tag, cannot have children
        let node = ParseNode::create(
            Vec::new(),
            html_tag,
            attributes,
            Vec::new(),
            lt_pos,
            buf_pos,
        );
        result.push(node);
        skip_past_gt(input_u8, buf_pos)
    } else {
        // collect text contents and nested tags until our closing tag
        buf_pos += 1; // past '>'
        let mut text = Vec::<u8>::new();
        let mut children = Vec::<ParseNode>::new();
        buf_pos = eat_whitespace(input_u8, buf_pos);
        let mut start_text: usize = buf_pos;
        while buf_pos < len
            && !is_closing_tag(input_u8, buf_pos, &tag)
            && !has_implied_close(input_u8, buf_pos, &html_tag)
        {
            if input_u8[buf_pos] != b'<'
                || is_raw_text_element(&html_tag)
                || depth >= MAX_DEPTH
            {
                text.push(input_u8[buf_pos]);
                buf_pos += 1;
                continue;
            }
            if !text.is_empty() {
                let node = ParseNode::create(
                    Vec::new(),
                    HTMLToken::Text,
                    HashMap::new(),
                    std::mem::take(&mut text),
                    start_text,
                    buf_pos.saturating_sub(1),
                );
                children.push(node);
            }
            let (new_childs, new_pos) = larse(input_u8, buf_pos, depth + 1, &html_tag);
            children.extend(new_childs);
            if new_pos > buf_pos {
                buf_pos = new_pos;
            } else {
                // the child made no progress and buf_pos sits on a '<' that is
                // neither our closing tag nor a parseable element
                let after_lt = eat_whitespace(input_u8, buf_pos + 1);
                if after_lt < len && input_u8[after_lt] == b'/' {
                    // Orphan closing tag for some other element — drop it
                    buf_pos = skip_past_gt(input_u8, buf_pos);
                } else {
                    // stray '<' (treat as text)
                    text.push(b'<');
                    buf_pos += 1;
                    start_text = buf_pos - 1;
                    continue;
                }
            }
            start_text = buf_pos;
        }
        if !text.is_empty() {
            let node = ParseNode::create(
                Vec::new(),
                HTMLToken::Text,
                HashMap::new(),
                text,
                start_text,
                buf_pos.saturating_sub(1),
            );
            children.push(node);
        }
        if has_implied_close(input_u8, buf_pos, &html_tag) {
            // implicitly closed element, leave the next tag for our parent
            buf_pos = eat_whitespace(input_u8, buf_pos);
            let node = ParseNode::create(children, html_tag, attributes, Vec::new(), lt_pos, buf_pos);
            result.push(node);
            buf_pos
        } else {
            let end = skip_past_gt(input_u8, buf_pos);
            let node = ParseNode::create(
                children,
                html_tag,
                attributes,
                Vec::new(),
                lt_pos,
                end.saturating_sub(1),
            );
            result.push(node);
            end
        }
    }
}

/// Parses tag attributes starting at `buf_pos` (just after the tag name)
/// Returns the position of the tag terminator: '>' or the '/' of "/>",
/// or the input length if the tag is truncated
fn parse_attributes(
    input_u8: &[u8],
    mut buf_pos: usize,
    attributes: &mut HashMap<String, String>,
) -> usize {
    fn flush(name: &mut Vec<u8>, val: &mut Vec<u8>, attributes: &mut HashMap<String, String>) {
        if !name.is_empty() {
            attributes.insert(
                String::from_utf8_lossy(name).into_owned(),
                decode_entities(&String::from_utf8_lossy(val)),
            );
        }
        name.clear();
        val.clear();
    }

    let mut name: Vec<u8> = Vec::new();
    let mut val: Vec<u8> = Vec::new();
    let mut has_value = false; // saw '=' for the current attribute
    let mut name_done = false; // saw whitespace after the current name
    let mut quote: Option<u8> = None;
    while buf_pos < input_u8.len() {
        let c = input_u8[buf_pos];
        if let Some(q) = quote {
            if c == q {
                flush(&mut name, &mut val, attributes);
                has_value = false;
                quote = None;
            } else {
                val.push(c);
            }
        } else if c == b'>' || (c == b'/' && input_u8.get(buf_pos + 1) == Some(&b'>')) {
            break;
        } else if has_value {
            if val.is_empty() && (c == b'"' || c == b'\'') {
                quote = Some(c);
            } else if is_whitespace(c) {
                if !val.is_empty() {
                    flush(&mut name, &mut val, attributes);
                    has_value = false;
                }
            } else {
                val.push(c);
            }
        } else if c == b'=' {
            has_value = true;
            name_done = false;
        } else if is_whitespace(c) {
            if !name.is_empty() {
                name_done = true;
            }
        } else {
            if name_done {
                // previous attribute had no value (assume boolean attribute)
                flush(&mut name, &mut val, attributes);
                name_done = false;
            }
            name.push(c);
        }
        buf_pos += 1;
    }
    flush(&mut name, &mut val, attributes);
    buf_pos
}

fn match_tag(tag: &[u8]) -> HTMLToken {
    let Ok(tag_str) = str::from_utf8(tag) else {
        return HTMLToken::Unknown;
    };
    if tag_str.starts_with("!--") {
        return HTMLToken::Comment;
    }
    match tag_str.to_uppercase().as_str() {
        "HTML" => HTMLToken::HtmlStart,
        "HEAD" => HTMLToken::Head,
        "A" => HTMLToken::HyperLink,
        "B" | "STRONG" => HTMLToken::BoldText,
        "BODY" => HTMLToken::Body,
        "BR" | "WBR" | "HR" => HTMLToken::LineBreak,
        "CODE" => HTMLToken::Code,
        "PRE" => HTMLToken::Preformatted,
        "DIV" => HTMLToken::DivSection,
        "H1" | "H2" | "H3" | "H4" | "H5" | "H6" => HTMLToken::Heading,
        "I" | "EM" => HTMLToken::ItalicText,
        "LI" => HTMLToken::ListItem,
        "OL" => HTMLToken::OrderedList,
        "P" => HTMLToken::Paragraph,
        "SCRIPT" => HTMLToken::Script,
        "STYLE" => HTMLToken::Style,
        "SPAN" => HTMLToken::Span,
        "TITLE" => HTMLToken::PageTitle,
        "UL" => HTMLToken::UnorderedList,
        "!DOCTYPE" => HTMLToken::DocType,

        // Tables
        "TABLE" => HTMLToken::Table,
        "THEAD" => HTMLToken::TableHead,
        "TBODY" => HTMLToken::TableBody,
        "TFOOT" => HTMLToken::TableFoot,
        "TR" => HTMLToken::TableRow,
        "TD" => HTMLToken::TableCell,
        "TH" => HTMLToken::TableHeaderCell,
        "CAPTION" => HTMLToken::TableCaption,
        "COLGROUP" => HTMLToken::TableColumnGroup,

        // Forms
        "FORM" => HTMLToken::Form,
        "BUTTON" => HTMLToken::Button,
        "SELECT" => HTMLToken::Select,
        "OPTION" => HTMLToken::Option,
        "OPTGROUP" => HTMLToken::OptionGroup,
        "TEXTAREA" => HTMLToken::TextArea,
        "LABEL" => HTMLToken::Label,
        "FIELDSET" => HTMLToken::FieldSet,
        "LEGEND" => HTMLToken::Legend,

        "BLOCKQUOTE" => HTMLToken::BlockQuote,

        // HTML5 semantic sectioning
        "NAV" => HTMLToken::Nav,
        "HEADER" => HTMLToken::PageHeader,
        "FOOTER" => HTMLToken::PageFooter,
        "MAIN" => HTMLToken::Main,
        "SECTION" => HTMLToken::Section,
        "ARTICLE" => HTMLToken::Article,
        "ASIDE" => HTMLToken::Aside,
        "FIGURE" => HTMLToken::Figure,
        "FIGCAPTION" => HTMLToken::FigureCaption,

        // Raw text / special content model elements
        "IFRAME" => HTMLToken::IFrame,
        "NOSCRIPT" => HTMLToken::NoScript,
        "XMP" | "NOEMBED" | "NOFRAMES" => HTMLToken::RawText,
        "TEMPLATE" => HTMLToken::Template,

        // Inline text-level semantics
        "SMALL" => HTMLToken::SmallText,
        "MARK" => HTMLToken::Mark,
        "SUB" => HTMLToken::Subscript,
        "SUP" => HTMLToken::Superscript,
        "DEL" => HTMLToken::Deleted,
        "INS" => HTMLToken::Inserted,
        "U" => HTMLToken::Underline,
        "S" => HTMLToken::Strikethrough,
        "ABBR" => HTMLToken::Abbreviation,
        "CITE" => HTMLToken::Citation,
        "KBD" => HTMLToken::KeyboardInput,
        "SAMP" => HTMLToken::SampleOutput,

        // Definition lists
        "DL" => HTMLToken::DescriptionList,
        "DT" => HTMLToken::DescriptionTerm,
        "DD" => HTMLToken::DescriptionDetails,

        "DETAILS" => HTMLToken::Details,
        "SUMMARY" => HTMLToken::Summary,
        "DIALOG" => HTMLToken::Dialog,

        // Other
        "CANVAS" => HTMLToken::Canvas,
        "SVG" => HTMLToken::Svg,
        "TIME" => HTMLToken::Time,
        "DATA" => HTMLToken::Data,
        "VAR" => HTMLToken::Variable,
        "RUBY" => HTMLToken::Ruby,
        "RT" => HTMLToken::RubyText,
        "RP" => HTMLToken::RubyParenthesis,
        "ADDRESS" => HTMLToken::Address,

        "AREA" | "BASE" | "COL" | "COMMAND" | "EMBED" | "IMG" | "INPUT" | "KEYGEN" | "LINK"
        | "META" | "PARAM" | "SOURCE" | "TRACK" => HTMLToken::Void,
        _ => HTMLToken::Unknown,
    }
}

pub fn parse_html(html: &str) -> (Vec<RenderItem>, String) {
    let mut root = ParseNode::new();
    root.tag = HTMLToken::Root;
    root.start_ind = 0;
    root.end_ind = html.len();
    let (children, _) = larse(html.as_bytes(), 0, 0, &HTMLToken::Root);
    root.children = children;
    let cur_state = RenderState::new();
    build_array(root, Vec::new(), cur_state, String::new())
}

fn build_array(
    node: ParseNode,
    mut ret_vec: Vec<RenderItem>,
    mut cur_state: RenderState,
    title: String,
) -> (Vec<RenderItem>, String) {
    let mut cur_title = title;
    for i in node.children {
        if is_hidden_subtree(&i) {
            continue;
        }
        match i.tag {
            HTMLToken::BoldText => {
                cur_state.bold = i.end_ind;
            }
            HTMLToken::HyperLink => {
                cur_state.link = i.end_ind;
                if let Some(link) = i.attributes.get("href") { cur_state.url = link.clone() }
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
            HTMLToken::Text
                if !(node.tag == HTMLToken::Unknown || node.tag == HTMLToken::Script)
                    && !is_text_whitespace(&i.text)
                => {
                    let mut item = RenderItem::new();
                    item.text = decode_entities(&String::from_utf8_lossy(&i.text));
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
            _ => {}
        }
        let ret = build_array(i, ret_vec, cur_state.clone(), cur_title.clone());
        ret_vec = ret.0;
        cur_title = ret.1;
    }
    (ret_vec, cur_title)
}

/// Decodes common named HTML entities and numeric character references
/// Unrecognized or malformed sequences are passed through unchanged
fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let after = &rest[amp + 1..];
        match after.find(';') {
            Some(semi) if semi <= 32 => {
                if let Some(decoded) = decode_entity_name(&after[..semi]) {
                    out.push(decoded);
                    rest = &after[semi + 1..];
                } else {
                    out.push('&');
                    rest = after;
                }
            }
            _ => {
                out.push('&');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn decode_entity_name(name: &str) -> Option<char> {
    if let Some(num) = name.strip_prefix('#') {
        let code = if let Some(hex) = num.strip_prefix('x').or_else(|| num.strip_prefix('X')) {
            u32::from_str_radix(hex, 16).ok()?
        } else {
            num.parse::<u32>().ok()?
        };
        return char::from_u32(code);
    }
    match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "nbsp" => Some('\u{00A0}'),
        _ => None,
    }
}

fn eat_whitespace(input_u8: &[u8], mut buf_pos: usize) -> usize {
    while buf_pos < input_u8.len() && is_whitespace(input_u8[buf_pos]) {
        buf_pos += 1;
    }
    buf_pos
}

fn is_text_whitespace(text: &[u8]) -> bool {
    text.iter().all(|&c| is_whitespace(c))
}

fn is_whitespace(c: u8) -> bool {
    matches!(c, b'\n' | b' ' | b'\t' | b'\r')
}

/// Advances past the next '>' and returns the index just after it
fn skip_past_gt(input_u8: &[u8], mut buf_pos: usize) -> usize {
    while buf_pos < input_u8.len() && input_u8[buf_pos] != b'>' {
        buf_pos += 1;
    }
    (buf_pos + 1).min(input_u8.len())
}

fn is_closing_tag(input_u8: &[u8], buf_pos: usize, tag: &[u8]) -> bool {
    if buf_pos + 2 >= input_u8.len() {
        return true;
    }
    if input_u8[buf_pos] != b'<' || input_u8[buf_pos + 1] != b'/' {
        return false;
    }
    let name_start = buf_pos + 2;
    match input_u8.get(name_start..name_start + tag.len()) {
        Some(name) => {
            name.eq_ignore_ascii_case(tag)
                && match input_u8.get(name_start + tag.len()) {
                    None | Some(&b'>') => true,
                    Some(&c) => is_whitespace(c),
                }
        }
        None => true,
    }
}

fn is_opening_tag(input_u8: &[u8], buf_pos: usize, tag: &[u8]) -> bool {
    if buf_pos + 1 >= input_u8.len() {
        return true;
    }
    if input_u8[buf_pos] != b'<' {
        return false;
    }
    let name_start = buf_pos + 1;
    match input_u8.get(name_start..name_start + tag.len()) {
        Some(name) => {
            name.eq_ignore_ascii_case(tag)
                && match input_u8.get(name_start + tag.len()) {
                    None | Some(&b'>') | Some(&b'/') => true,
                    Some(&c) => is_whitespace(c),
                }
        }
        None => true,
    }
}

fn is_closing_comment(input_u8: &[u8], buf_pos: usize) -> bool {
    match input_u8.get(buf_pos..buf_pos + 3) {
        Some(s) => s == b"-->",
        None => true,
    }
}

/// nested tags are not parsed for elements whose contents are raw text
fn is_raw_text_element(tag: &HTMLToken) -> bool {
    matches!(
        tag,
        HTMLToken::Script
            | HTMLToken::Style
            | HTMLToken::RawText
            | HTMLToken::TextArea
            | HTMLToken::IFrame
            | HTMLToken::Svg
    )
}

/// True if the tag at `buf_pos` implicitly closes an open `html_tag` element
/// with an optional end tag (like a second <li>, or </ul> while in an <li>).
fn has_implied_close(input_u8: &[u8], buf_pos: usize, html_tag: &HTMLToken) -> bool {
    // Openers that end a <p>: block-level elements
    const P_OPEN: &[&[u8]] = &[
        b"address", b"article", b"aside", b"blockquote", b"details", b"dialog", b"div", b"dl",
        b"fieldset", b"figcaption", b"figure", b"footer", b"form", b"h1", b"h2", b"h3", b"h4",
        b"h5", b"h6", b"header", b"hr", b"main", b"nav", b"ol", b"p", b"pre", b"section",
        b"table", b"ul",
    ];
    const LI_OPEN: &[&[u8]] = &[b"li"];
    const LI_CLOSE: &[&[u8]] = &[b"ul", b"ol"];
    const DT_OPEN: &[&[u8]] = &[b"dt", b"dd"];
    const DT_CLOSE: &[&[u8]] = &[b"dl"];
    const OPTION_OPEN: &[&[u8]] = &[b"option", b"optgroup"];
    const OPTION_CLOSE: &[&[u8]] = &[b"select", b"optgroup", b"datalist"];
    const OPTGROUP_OPEN: &[&[u8]] = &[b"optgroup"];
    const OPTGROUP_CLOSE: &[&[u8]] = &[b"select"];
    const TR_OPEN: &[&[u8]] = &[b"tr"];
    const TR_CLOSE: &[&[u8]] = &[b"table", b"tbody", b"thead", b"tfoot"];
    // A new row also ends the current cell
    const TD_OPEN: &[&[u8]] = &[b"td", b"th", b"tr"];
    const TD_CLOSE: &[&[u8]] = &[b"tr", b"table", b"tbody", b"thead", b"tfoot"];
    const THEAD_OPEN: &[&[u8]] = &[b"tbody", b"tfoot"];
    const TABLE_CLOSE: &[&[u8]] = &[b"table"];
    const CAPTION_OPEN: &[&[u8]] = &[b"tr", b"thead", b"tbody", b"tfoot", b"colgroup"];
    const RT_OPEN: &[&[u8]] = &[b"rt", b"rp"];
    const RUBY_CLOSE: &[&[u8]] = &[b"ruby"];
    const NONE: &[&[u8]] = &[];

    let (openers, closers, any_closer): (&[&[u8]], &[&[u8]], bool) = match html_tag {
        HTMLToken::ListItem => (LI_OPEN, LI_CLOSE, false),
        HTMLToken::DescriptionTerm | HTMLToken::DescriptionDetails => (DT_OPEN, DT_CLOSE, false),
        HTMLToken::Option => (OPTION_OPEN, OPTION_CLOSE, false),
        HTMLToken::OptionGroup => (OPTGROUP_OPEN, OPTGROUP_CLOSE, false),
        HTMLToken::Paragraph => (P_OPEN, NONE, true),
        HTMLToken::TableRow => (TR_OPEN, TR_CLOSE, false),
        HTMLToken::TableCell | HTMLToken::TableHeaderCell => (TD_OPEN, TD_CLOSE, false),
        HTMLToken::TableHead | HTMLToken::TableBody => (THEAD_OPEN, TABLE_CLOSE, false),
        HTMLToken::TableFoot => (NONE, TABLE_CLOSE, false),
        HTMLToken::TableCaption => (CAPTION_OPEN, TABLE_CLOSE, false),
        HTMLToken::RubyText | HTMLToken::RubyParenthesis => (RT_OPEN, RUBY_CLOSE, false),
        _ => return false,
    };
    let buf_pos = eat_whitespace(input_u8, buf_pos);
    if any_closer && is_any_closing_tag(input_u8, buf_pos) {
        return true;
    }
    openers.iter().any(|t| is_opening_tag(input_u8, buf_pos, t))
        || closers.iter().any(|t| is_closing_tag(input_u8, buf_pos, t))
}

/// True if `buf_pos` starts any closing tag, or is too close to EOF.
fn is_any_closing_tag(input_u8: &[u8], buf_pos: usize) -> bool {
    if buf_pos + 1 >= input_u8.len() {
        return true;
    }
    input_u8[buf_pos] == b'<' && input_u8[buf_pos + 1] == b'/'
}

/// Subtrees that produce no visible page content.
fn is_hidden_subtree(node: &ParseNode) -> bool {
    match node.tag {
        HTMLToken::Script
        | HTMLToken::Style
        | HTMLToken::RawText
        | HTMLToken::Template
        | HTMLToken::IFrame
        | HTMLToken::Svg => true,
        // <dialog> is hidden unless it has the `open` attribute
        HTMLToken::Dialog => !node.attributes.contains_key("open"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(html: &str) -> Vec<String> {
        parse_html(html)
            .0
            .into_iter()
            .filter(|item| !item.line_break)
            .map(|item| item.text)
            .collect()
    }

    fn find_item(items: &[RenderItem], text: &str) -> Option<usize> {
        items.iter().position(|i| i.text == text)
    }

    #[test]
    fn parses_simple_page() {
        let html = "<html><head><title>My Page</title></head>\
                    <body><h1>Hello</h1><p>World</p></body></html>";
        let (items, title) = parse_html(html);
        assert_eq!(title, "My Page");
        let heading = &items[find_item(&items, "Hello").expect("heading item")];
        assert!(heading.heading);
        assert!(!heading.bold);
        let body_text = &items[find_item(&items, "World").expect("paragraph item")];
        assert!(!body_text.heading);
    }

    #[test]
    fn applies_inline_formatting() {
        let html = "<html><body><p><b>bold</b> and <i>italic</i> and <code>mono</code></p></body></html>";
        let (items, _) = parse_html(html);
        assert!(items[find_item(&items, "bold").unwrap()].bold);
        assert!(items[find_item(&items, "italic").unwrap()].italics);
        assert!(items[find_item(&items, "mono").unwrap()].code);
        let plain = &items[find_item(&items, "and ").expect("plain text between tags")];
        assert!(!plain.bold && !plain.italics);
    }

    #[test]
    fn link_href_with_equals_and_entity() {
        let html = r#"<html><body><a href="page?a=b&amp;c=d">click</a></body></html>"#;
        let (items, _) = parse_html(html);
        let link = &items[find_item(&items, "click").unwrap()];
        assert!(link.link);
        assert_eq!(link.url, "page?a=b&c=d");
    }

    #[test]
    fn attribute_quoting_variants() {
        let html = concat!(
            "<html><body>",
            "<a href='x y'>single</a>",
            "<a href=plain>unquoted</a>",
            r#"<a href="a">short</a>"#,
            r#"<a data-bool href="z">boolean</a>"#,
            "<a href = \"spaced\">spaced</a>",
            "</body></html>"
        );
        let (items, _) = parse_html(html);
        let url_of = |t: &str| items[find_item(&items, t).unwrap()].url.clone();
        assert_eq!(url_of("single"), "x y");
        assert_eq!(url_of("unquoted"), "plain");
        assert_eq!(url_of("short"), "a");
        assert_eq!(url_of("boolean"), "z");
        assert_eq!(url_of("spaced"), "spaced");
    }

    #[test]
    fn decodes_entities_in_text() {
        let html = "<html><body><p>a &amp; b &lt;c&gt; &#65;&#x42; &bogus; d&nbsp;e</p></body></html>";
        let all = texts(html).join("");
        assert_eq!(all, "a & b <c> AB &bogus; d\u{a0}e");
    }

    #[test]
    fn preserves_non_ascii() {
        let html = "<html><body><p>héllo — ünïcode</p><a href=\"/päge\">x</a></body></html>";
        let (items, _) = parse_html(html);
        assert!(find_item(&items, "héllo — ünïcode").is_some());
        assert_eq!(items[find_item(&items, "x").unwrap()].url, "/päge");
    }

    #[test]
    fn line_break_keeps_following_text() {
        let html = "<html><body><p>a<br>b</p></body></html>";
        let (items, _) = parse_html(html);
        assert!(items.iter().any(|i| i.line_break));
        assert_eq!(texts(html), vec!["a", "b"]);
    }

    #[test]
    fn text_after_nested_element_has_no_stray_bracket() {
        let html = "<html><body><p><b>x</b>y</p></body></html>";
        assert_eq!(texts(html), vec!["x", "y"]);
    }

    #[test]
    fn orphan_closing_tag_is_skipped() {
        let html = "<html><body><p>a</span>b</p></body></html>";
        assert_eq!(texts(html), vec!["a", "b"]);
    }

    #[test]
    fn parses_list_with_unclosed_items() {
        let html = "<html><body><ul><li>one<li>two</li><li>three</ul></body></html>";
        assert_eq!(texts(html), vec!["one", "two", "three"]);
    }

    #[test]
    fn script_and_style_content_not_rendered() {
        let html = "<html><head><style>.x { color: red; }</style></head>\
                    <body><script>var a = 1 < 2;</script><p>ok</p></body></html>";
        assert_eq!(texts(html), vec!["ok"]);
    }

    #[test]
    fn self_closing_and_void_tags() {
        let html = "<html><body><p>a</p><img src=\"pic.png\"/><meta charset=\"utf-8\"><p>b</p></body></html>";
        assert_eq!(texts(html), vec!["a", "b"]);
    }

    #[test]
    fn svg_content_is_skipped() {
        let html = "<html><body><svg viewBox=\"0 0 10 10\"><path d=\"M0 0\">\
                    <circle r=\"4\"/></svg><p>after</p></body></html>";
        assert_eq!(texts(html), vec!["after"]);
    }

    #[test]
    fn textarea_content_is_raw_text() {
        let html = "<html><body><textarea>a <b>b</b></textarea></body></html>";
        assert_eq!(texts(html), vec!["a <b>b</b>"]);
    }

    #[test]
    fn hidden_elements_do_not_render() {
        let html = "<html><body><iframe>fallback</iframe><template>tpl</template>\
                    <dialog>closed</dialog><p>visible</p></body></html>";
        assert_eq!(texts(html), vec!["visible"]);
    }

    #[test]
    fn open_dialog_and_noscript_render() {
        let html = "<html><body><dialog open>hi</dialog>\
                    <noscript>enable js</noscript></body></html>";
        assert_eq!(texts(html), vec!["hi", "enable js"]);
    }

    #[test]
    fn definition_list_implied_close() {
        let html = "<html><body><dl><dt>term<dd>def</dl></body></html>";
        assert_eq!(texts(html), vec!["term", "def"]);
    }

    #[test]
    fn table_with_unclosed_cells_and_rows() {
        let html = "<html><body><table><tr><td>a<td>b<tr><td>c</table><p>end</p></body></html>";
        assert_eq!(texts(html), vec!["a", "b", "c", "end"]);
    }

    #[test]
    fn paragraph_implied_close() {
        let html = "<html><body><div><p>one<p>two</div><p>three</body></html>";
        assert_eq!(texts(html), vec!["one", "two", "three"]);
    }

    #[test]
    fn select_options_render_with_implied_close() {
        let html = "<html><body><select><option>x<option>y</select></body></html>";
        assert_eq!(texts(html), vec!["x", "y"]);
    }

    #[test]
    fn survives_malformed_input() {
        let cases = [
            "",
            "<",
            "< ",
            "</",
            "<3>",
            "<ta",
            "<!DOCTYPE html",
            "<!-- never closed",
            "<html><body><b>x",
            "<a href=\"unterminated",
            "<div",
            "<b>text</blockquote>",
            "<p>a</p>/",
            "<p>1 < 2 but > 0</p>",
            "<p></p>",
            "<p><</p>",
        ];
        for case in cases {
            let _ = parse_html(case); // must not panic or hang
        }
    }

    #[test]
    fn survives_deep_nesting() {
        let mut html = String::from("<html><body>");
        html.push_str(&"<div>".repeat(5000));
        html.push_str("deep");
        html.push_str(&"</div>".repeat(5000));
        html.push_str("</body></html>");
        let _ = parse_html(&html); // must not overflow the stack
    }

    #[test]
    fn parses_bundled_fixtures() {
        let (items, title) = parse_html(include_str!("microll.html"));
        assert!(!items.is_empty());
        assert_eq!(title, "Microll: The Text-Based Web Browser");

        let (items, _) = parse_html(include_str!("test.html"));
        assert!(items.iter().any(|i| i.text == "Test Heading" && i.heading));
        assert!(items.iter().any(|i| i.text == "paragraph" && i.link));
    }
}
