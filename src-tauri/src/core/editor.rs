use pulldown_cmark::{html::push_html, Event, Options, Parser, Tag, TagEnd};
use serde::Serialize;

// todo: test data
const TEST_MARKDOWN: &str = r#"
# a~~b~~
## b
### c1
123
- a
- b
  1. c
  1. d
### c2
456

```rust
println!("test");
```
"#;

#[derive(Serialize)]
pub struct ParsedMarkdown {
    heading_level: Option<usize>,
    heading_text: Option<String>,
    html: Option<String>,
}

// trait EventExt {
//     // fn index(&self) -> usize;
//     fn to_string(&self) -> String;
// }

// impl<'a> EventExt for Event<'a> {
//     // fn index(&self) -> usize {
//     //     match self {
//     //         Event::Start(_) => 0,
//     //         Event::End(_) => 1,
//     //         Event::Text(_) => 2,
//     //         Event::Code(_) => 3,
//     //         Event::InlineMath(_) => 4,
//     //         Event::DisplayMath(_) => 5,
//     //         Event::Html(_) => 6,
//     //         Event::InlineHtml(_) => 7,
//     //         Event::FootnoteReference(_) => 8,
//     //         Event::SoftBreak => 9,
//     //         Event::HardBreak => 10,
//     //         Event::Rule => 11,
//     //         Event::TaskListMarker(_) => 12,
//     //     }
//     // }

//     fn to_string(&self) -> String {
//         match self {
//             Event::Text(s)
//             | Event::Code(s)
//             | Event::InlineMath(s)
//             | Event::DisplayMath(s)
//             | Event::Html(s)
//             | Event::InlineHtml(s)
//             | Event::FootnoteReference(s) => s.to_string(),
//             _ => String::new(),
//         }
//     }
// }

// trait TagExt {
//     fn name(&self) -> String;
// }

// impl<'a> TagExt for Tag<'a> {
//     fn name(&self) -> String {
//         match self {
//             Tag::Paragraph => "p".to_owned(),
//             Tag::Heading { level, .. } => level.to_string(),
//             Tag::BlockQuote { .. } => "blockquote".to_owned(),
//             Tag::CodeBlock { .. } => "code".to_owned(),
//             Tag::List(opt) => {
//                 // todo correct
//                 if opt.is_some() && opt.unwrap() == 0 {
//                     "ol".to_owned()
//                 } else {
//                     "ul".to_owned()
//                 }
//             }
//             Tag::Item => "li".to_owned(),
//             Tag::DefinitionList => "dl".to_owned(),
//             Tag::DefinitionListTitle => "dt".to_owned(),
//             Tag::DefinitionListDefinition => "dd".to_owned(),
//             Tag::Table(_) => "table".to_owned(),
//             Tag::TableHead => "thead".to_owned(),
//             Tag::TableRow => "tr".to_owned(),
//             Tag::TableCell => "td".to_owned(),
//             Tag::Emphasis => "em".to_owned(),
//             Tag::Strong => "strong".to_owned(),
//             Tag::Strikethrough => "strikethrough".to_owned(),
//             Tag::Link { .. } => "a".to_owned(),
//             Tag::Image { .. } => "img".to_owned(),
//             Tag::HtmlBlock | Tag::FootnoteDefinition { .. } | Tag::MetadataBlock { .. } => {
//                 String::new()
//             }
//         }
//     }
// }

// impl TagExt for TagEnd {
//     fn name(&self) -> String {
//         match self {
//             TagEnd::Paragraph => "p".to_owned(),
//             TagEnd::Heading(level) => level.to_string(),
//             TagEnd::BlockQuote { .. } => "blockquote".to_owned(),
//             TagEnd::CodeBlock { .. } => "code".to_owned(),
//             TagEnd::List(opt) => {
//                 // todo correct
//                 if *opt {
//                     "ol".to_owned()
//                 } else {
//                     "ul".to_owned()
//                 }
//             }
//             TagEnd::Item => "li".to_owned(),
//             TagEnd::DefinitionList => "dl".to_owned(),
//             TagEnd::DefinitionListTitle => "dt".to_owned(),
//             TagEnd::DefinitionListDefinition => "dd".to_owned(),
//             TagEnd::Table => "table".to_owned(),
//             TagEnd::TableHead => "thead".to_owned(),
//             TagEnd::TableRow => "tr".to_owned(),
//             TagEnd::TableCell => "td".to_owned(),
//             TagEnd::Emphasis => "em".to_owned(),
//             TagEnd::Strong => "strong".to_owned(),
//             TagEnd::Strikethrough => "strikethrough".to_owned(),
//             TagEnd::Link { .. } => "a".to_owned(),
//             TagEnd::Image { .. } => "img".to_owned(),
//             TagEnd::HtmlBlock | TagEnd::FootnoteDefinition | TagEnd::MetadataBlock(_) => {
//                 String::new()
//             }
//         }
//     }
// }

#[tauri::command]
pub fn ready() -> String {
    TEST_MARKDOWN.to_owned()
}

#[tauri::command]
pub fn parse(markdown_text: &str) -> Vec<ParsedMarkdown> {
    let mut ret: Vec<ParsedMarkdown> = vec![];

    let options = Options::from_iter([Options::ENABLE_STRIKETHROUGH]);
    let parser: Parser<'_> = Parser::new_ext(markdown_text, options);

    let mut heading_parsed_markdown = ParsedMarkdown {
        heading_level: None,
        heading_text: None,
        html: None,
    };
    let mut html_events: Vec<Event> = vec![];
    parser.for_each(|x| {
        let x = &x.clone();
        match x {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    if 0 < html_events.len() {
                        let mut html_buf = String::new();
                        push_html(&mut html_buf, html_events.clone().into_iter());
                        ret.push(ParsedMarkdown {
                            heading_level: None,
                            heading_text: None,
                            html: Some(html_buf),
                        });
                        html_events = vec![];
                    }

                    let heading_level = level
                        .to_string()
                        .chars()
                        .skip(1)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    heading_parsed_markdown.heading_level = Some(heading_level);
                }
                _ => {
                    if heading_parsed_markdown.heading_level.is_none() {
                        html_events.push(x.clone());
                    }
                }
            },
            Event::Text(s)
            | Event::Code(s)
            | Event::Html(s)
            | Event::InlineHtml(s)
            | Event::InlineMath(s)
            | Event::DisplayMath(s) => {
                if heading_parsed_markdown.heading_level.is_some() {
                    heading_parsed_markdown.heading_text = Some(format!(
                        "{}{}",
                        heading_parsed_markdown
                            .heading_text
                            .clone()
                            .unwrap_or_default(),
                        s
                    ));
                } else {
                    html_events.push(x.clone());
                }
            }
            Event::End(tag) => match tag {
                TagEnd::Heading { .. } => {
                    ret.push(ParsedMarkdown {
                        heading_level: heading_parsed_markdown.heading_level.clone(),
                        heading_text: heading_parsed_markdown.heading_text.clone(),
                        html: None,
                    });

                    heading_parsed_markdown = ParsedMarkdown {
                        heading_level: None,
                        heading_text: None,
                        html: None,
                    };
                }
                _ => {
                    if heading_parsed_markdown.heading_level.is_none() {
                        html_events.push(x.clone());
                    }
                }
            },
            _ => {}
        }
    });

    ret
}
