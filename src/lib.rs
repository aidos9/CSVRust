mod csv_document;

use crate::csv_document::CSVDocument;

pub mod parser {
    use super::*;
    pub fn parse_string(string: &String) -> Result<CSVDocument, &'static str>
    {
        return Ok(CSVDocument::new());
    }
}
