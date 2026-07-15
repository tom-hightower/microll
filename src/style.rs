use crate::html;
use crate::structs::{
    BlockBoundary, Display, ElementStyle, HTMLToken, MarkerKind, ParseNode, RenderContext,
    RenderItem,
};

/// Walks the parsed tag tree to handle per-tag styling
pub fn style_tree(root: ParseNode) -> (Vec<RenderItem>, String) {
    let mut out = Vec::new();
    let mut title = String::new();
    walk_children(root.tag, root.children, RenderContext::new(), &mut out, &mut title);
    (out, title)
}

fn walk_children(
    parent_tag: HTMLToken,
    children: Vec<ParseNode>,
    ctx: RenderContext,
    out: &mut Vec<RenderItem>,
    title: &mut String,
) {
    let mut ordinal: u32 = 1;

    for node in children {
        if html::is_hidden_subtree(&node) {
            continue;
        }

        let mut child_ctx = ctx.clone();
        match node.tag {
            HTMLToken::BoldText => child_ctx.bold = true,
            HTMLToken::ItalicText => child_ctx.italics = true,
            HTMLToken::Code | HTMLToken::Preformatted => child_ctx.code = true,
            HTMLToken::HyperLink => {
                child_ctx.link = Some(node.attributes.get("href").cloned().unwrap_or_default());
            }
            HTMLToken::PageTitle => child_ctx.in_title = true,
            HTMLToken::Heading(level) => child_ctx.heading_level = Some(level),
            HTMLToken::UnorderedList => child_ctx.list_kind = Some(MarkerKind::Bullet),
            HTMLToken::OrderedList => child_ctx.list_kind = Some(MarkerKind::Ordinal),
            HTMLToken::ListItem => child_ctx.list_kind = None,
            HTMLToken::LineBreak => {
                out.push(RenderItem::line_break());
                continue;
            }
            _ => {}
        }

        let style = default_style_for(node.tag);
        if style.display == Display::Block {
            let marker = if node.tag == HTMLToken::ListItem {
                match ctx.list_kind {
                    Some(MarkerKind::Bullet) => Some("\u{2022}".to_string()),
                    Some(MarkerKind::Ordinal) => {
                        let m = format!("{ordinal}.");
                        ordinal += 1;
                        Some(m)
                    }
                    None => None,
                }
            } else {
                None
            };
            out.push(RenderItem::block(BlockBoundary {
                spacing_before: style.spacing_before,
                indent: ctx.indent,
                marker,
            }));
        }
        if style.indents {
            child_ctx.indent = ctx.indent + 1;
        }

        if node.tag == HTMLToken::Text
            && !(parent_tag == HTMLToken::Unknown || parent_tag == HTMLToken::Script)
            && !html::is_text_whitespace(&node.text)
        {
            let mut item = RenderItem::new();
            item.text = html::decode_entities(&String::from_utf8_lossy(&node.text));
            item.bold = ctx.bold;
            item.italics = ctx.italics;
            item.code = ctx.code;
            item.heading_level = ctx.heading_level;
            if let Some(url) = &ctx.link {
                item.link = true;
                item.url = url.clone();
            }
            if ctx.in_title {
                item.title = true;
                *title = item.text.clone();
            }
            out.push(item);
        }

        walk_children(node.tag, node.children, child_ctx, out, title);
    }
}

fn default_style_for(tag: HTMLToken) -> ElementStyle {
    match tag {
        HTMLToken::Paragraph => ElementStyle {
            display: Display::Block,
            spacing_before: 1.0,
            indents: false,
        },
        HTMLToken::DivSection => ElementStyle {
            display: Display::Block,
            spacing_before: 0.0,
            indents: false,
        },
        HTMLToken::Heading(_) => ElementStyle {
            display: Display::Block,
            spacing_before: 1.0,
            indents: false,
        },
        HTMLToken::BlockQuote => ElementStyle {
            display: Display::Block,
            spacing_before: 1.0,
            indents: true,
        },
        HTMLToken::UnorderedList | HTMLToken::OrderedList => ElementStyle {
            display: Display::Block,
            spacing_before: 0.5,
            indents: true,
        },
        HTMLToken::ListItem => ElementStyle {
            display: Display::Block,
            spacing_before: 0.15,
            indents: false,
        },
        HTMLToken::Nav
        | HTMLToken::PageHeader
        | HTMLToken::PageFooter
        | HTMLToken::Main
        | HTMLToken::Section
        | HTMLToken::Article
        | HTMLToken::Aside => ElementStyle {
            display: Display::Block,
            spacing_before: 0.5,
            indents: false,
        },
        HTMLToken::Preformatted => ElementStyle {
            display: Display::Block,
            spacing_before: 1.0,
            indents: false,
        },
        _ => ElementStyle {
            display: Display::Inline,
            spacing_before: 0.0,
            indents: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::html::parse_html;

    #[test]
    fn heading_levels_differ() {
        let html = "<html><body><h1>Big</h1><h3>Small</h3></body></html>";
        let (items, _) = parse_html(html);
        let h1 = items.iter().find(|i| i.text == "Big").unwrap();
        let h3 = items.iter().find(|i| i.text == "Small").unwrap();
        assert_eq!(h1.heading_level, Some(1));
        assert_eq!(h3.heading_level, Some(3));
    }

    #[test]
    fn unordered_list_gets_bullet_markers() {
        let html = "<html><body><ul><li>a</li><li>b</li></ul></body></html>";
        let (items, _) = parse_html(html);
        let markers: Vec<String> = items
            .iter()
            .filter_map(|i| i.block.as_ref().and_then(|b| b.marker.clone()))
            .collect();
        assert_eq!(markers, vec!["\u{2022}".to_string(), "\u{2022}".to_string()]);
    }

    #[test]
    fn ordered_list_numbers_items_in_order() {
        let html = "<html><body><ol><li>a</li><li>b</li><li>c</li></ol></body></html>";
        let (items, _) = parse_html(html);
        let markers: Vec<String> = items
            .iter()
            .filter_map(|i| i.block.as_ref().and_then(|b| b.marker.clone()))
            .collect();
        assert_eq!(markers, vec!["1.", "2.", "3."]);
    }

    #[test]
    fn nested_ordered_list_restarts_numbering_and_indents() {
        let html = "<html><body><ol><li>one<ol><li>a</li><li>b</li></ol></li><li>two</li></ol></body></html>";
        let (items, _) = parse_html(html);
        let marked: Vec<(String, u8)> = items
            .iter()
            .filter_map(|i| {
                i.block
                    .as_ref()
                    .and_then(|b| b.marker.clone().map(|m| (m, b.indent)))
            })
            .collect();
        assert_eq!(
            marked,
            vec![
                ("1.".to_string(), 1), // outer li "one"
                ("1.".to_string(), 2), // nested li "a" -- numbering restarted
                ("2.".to_string(), 2), // nested li "b"
                ("2.".to_string(), 1), // outer li "two"
            ]
        );
    }

    #[test]
    fn consecutive_paragraphs_each_get_one_block_boundary() {
        let html = "<html><body><p>one</p><p>two</p></body></html>";
        let (items, _) = parse_html(html);
        let block_count = items.iter().filter(|i| i.block.is_some()).count();
        assert_eq!(block_count, 2);
    }

    #[test]
    fn blockquote_increases_indent_for_nested_block() {
        let html = "<html><body><blockquote><p>quoted</p></blockquote></body></html>";
        let (items, _) = parse_html(html);
        let boundaries: Vec<_> = items.iter().filter_map(|i| i.block.as_ref()).collect();
        assert_eq!(boundaries[0].indent, 0);
        assert_eq!(boundaries[0].marker, None);
        assert_eq!(boundaries[1].indent, 1);
    }

    #[test]
    fn preformatted_sets_code_style() {
        let html = "<html><body><pre>fn main() {}</pre></body></html>";
        let (items, _) = parse_html(html);
        let text = items.iter().find(|i| i.text.contains("fn main")).unwrap();
        assert!(text.code);
    }
}
