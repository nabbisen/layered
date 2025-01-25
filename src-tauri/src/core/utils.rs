use super::types;

pub fn contents_str(contents: &Vec<types::ContentType>, heading_level: u8) -> String {
    contents
        .iter()
        .map(|x| {
            if x.children.is_none() {
                x.text.to_owned()
            } else {
                let children = x.clone().children;
                format!(
                    "{} {}\n\n{}",
                    "#".repeat(heading_level as usize),
                    x.text,
                    contents_str(children.unwrap().as_ref(), heading_level + 1)
                )
            }
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}
