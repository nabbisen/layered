use super::types::ParsedMarkdown;
use mdka::from_html;
use pulldown_cmark::{html::push_html, Event, Options, Parser, Tag, TagEnd};

pub fn parse(markdown_text: &str) -> Vec<ParsedMarkdown> {
    let mut ret: Vec<ParsedMarkdown> = vec![];

    let options = Options::from_iter([Options::ENABLE_STRIKETHROUGH]);
    let parser: Parser<'_> = Parser::new_ext(markdown_text, options);

    let mut heading_parsed_markdown = ParsedMarkdown {
        node_id: 0,
        is_heading: false,
        heading_level: 0,
        text: None,
        parent_node_id: None,
        ancestors: vec![],
    };
    let mut node_id: usize = 0;
    let mut heading_level: usize = 0;
    let mut html_events: Vec<Event> = vec![];
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
                            is_heading: false,
                            heading_level: heading_level,
                            text: Some(html_buf),
                            parent_node_id: ancestors.last().copied(),
                            ancestors: ancestors.clone(),
                        });
                        node_id += 1;
                        html_events = vec![];
                    }

                    heading_level = level
                        .to_string()
                        .chars()
                        .skip(1)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();

                    heading_parsed_markdown = ParsedMarkdown {
                        node_id,
                        is_heading: true,
                        heading_level: heading_level,
                        text: None,
                        parent_node_id: None,
                        ancestors: vec![],
                    };

                    while heading_level <= ancestors.len() {
                        ancestors.pop();
                    }
                    heading_parsed_markdown.parent_node_id = ancestors.last().copied();
                    heading_parsed_markdown.ancestors = ancestors.clone();
                }
                _ => {
                    if !heading_parsed_markdown.is_heading {
                        html_events.push(x.clone());
                    }
                }
            },
            Event::End(tag) => match tag {
                TagEnd::Heading { .. } => {
                    ret.push(ParsedMarkdown {
                        node_id,
                        is_heading: true,
                        heading_level: heading_level,
                        text: heading_parsed_markdown.text.clone(),
                        parent_node_id: heading_parsed_markdown.parent_node_id,
                        ancestors: heading_parsed_markdown.ancestors.clone(),
                    });
                    ancestors.push(node_id);
                    node_id += 1;

                    heading_parsed_markdown.is_heading = false;
                }
                _ => {
                    if !heading_parsed_markdown.is_heading {
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
                if heading_parsed_markdown.is_heading {
                    heading_parsed_markdown.text = Some(format!(
                        "{}{}",
                        heading_parsed_markdown.text.clone().unwrap_or_default(),
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
            is_heading: false,
            heading_level: heading_level,
            text: Some(html_buf),
            parent_node_id: ancestors.last().copied(),
            ancestors: ancestors.clone(),
        });
    }

    ret
}

pub fn compose(parsed_markdowns: Vec<ParsedMarkdown>) -> String {
    parsed_markdowns
        .iter()
        .map(|x| {
            if x.is_heading {
                format!(
                    "{} {}",
                    "#".repeat(x.heading_level),
                    x.text.clone().unwrap()
                )
            } else {
                let s = from_html(&x.text.clone().unwrap());
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
