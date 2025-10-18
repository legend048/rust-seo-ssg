use pulldown_cmark::{Parser, html::push_html};

/// Turn a Markdown string into an HTML string.
pub fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new(md);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::markdown_to_html;

    #[test]
    fn headers_and_lists() {
        let md = "# Hello\n\n- one\n- two";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<li>one</li>"));
    }
}
