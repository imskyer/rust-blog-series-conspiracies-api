// I've added the derive debug so I can use println! to print it out
#[derive(Debug)]
pub struct WikiPage  {
    title: String, 
    page_id: String,
    summary: String,
    content: String,
    background: String, 
}

impl WikiPage {
    pub fn new(title: String, page_id: String, summary: String, content: String, background: String) -> WikiPage {
        WikiPage{
            title: title,
            page_id: page_id,
            summary: summary,
            content: content,
            background: background,
        }
    }
}
