use super::types::ParsedMarkdown;
use mdka::from_html;
use pulldown_cmark::{html::push_html, Event, Options, Parser, Tag, TagEnd};

pub fn parse(markdown_text: &str) -> Vec<ParsedMarkdown> {
    let mut ret: Vec<ParsedMarkdown> = vec![];

    let options = Options::from_iter([Options::ENABLE_STRIKETHROUGH]);
    let parser: Parser<'_> = Parser::new_ext(markdown_text, options);

    let mut heading_parsed_markdown = ParsedMarkdown {
        node_id: 0,
        ancestors: vec![],
        nesting_level: 0,
        heading_level: None,
        heading_text: None,
        html: None,
    };
    let mut html_events: Vec<Event> = vec![];
    let mut node_id: usize = 0;
    let mut ancestors: Vec<usize> = vec![];
    parser.for_each(|x| {
        let x = &x.clone();
        match x {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    if 0 < html_events.len() {
                        let mut html_buf = String::new();
                        push_html(&mut html_buf, html_events.clone().into_iter());
                        ret.push(ParsedMarkdown {
                            node_id,
                            ancestors: ancestors.clone(),
                            nesting_level: ancestors.len(),
                            heading_level: None,
                            heading_text: None,
                            html: Some(html_buf),
                        });
                        node_id += 1;
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

                    while heading_level <= ancestors.len() {
                        ancestors.pop();
                    }
                    heading_parsed_markdown.ancestors = ancestors.clone();
                }
                _ => {
                    if heading_parsed_markdown.heading_level.is_none() {
                        html_events.push(x.clone());
                    }
                }
            },
            Event::End(tag) => match tag {
                TagEnd::Heading { .. } => {
                    ret.push(ParsedMarkdown {
                        node_id,
                        ancestors: heading_parsed_markdown.ancestors.clone(),
                        nesting_level: heading_parsed_markdown.ancestors.len(),
                        heading_level: heading_parsed_markdown.heading_level.clone(),
                        heading_text: heading_parsed_markdown.heading_text.clone(),
                        html: None,
                    });
                    ancestors.push(node_id);
                    node_id += 1;

                    heading_parsed_markdown = ParsedMarkdown {
                        node_id,
                        ancestors: ancestors.clone(),
                        nesting_level: ancestors.len(),
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
            _ => {}
        }
    });
    if 0 < html_events.len() {
        let mut html_buf = String::new();
        push_html(&mut html_buf, html_events.clone().into_iter());
        ret.push(ParsedMarkdown {
            node_id,
            ancestors: ancestors.clone(),
            nesting_level: ancestors.len(),
            heading_level: None,
            heading_text: None,
            html: Some(html_buf),
        });
    }

    ret
}

pub fn compose(parsed_markdowns: Vec<ParsedMarkdown>) -> String {
    parsed_markdowns
        .iter()
        .map(|x| {
            if x.heading_level.is_some() {
                format!(
                    "{} {}",
                    "#".repeat(x.heading_level.unwrap()),
                    x.heading_text.clone().unwrap()
                )
            } else {
                let s = from_html(&x.html.clone().unwrap());
                s.lines()
                    .map(|line| {
                        // escape headings-like leading
                        if line.starts_with('#') {
                            format!(r"\{}", line)
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
