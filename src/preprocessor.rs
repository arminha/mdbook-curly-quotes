use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event};

pub struct CurlyQuotes;

impl CurlyQuotes {
    pub fn new() -> Self {
        CurlyQuotes
    }

    fn process_item(&self, item: &mut BookItem) -> Result<(), Error> {
        if let BookItem::Chapter(ref mut ch) = item {
            ch.content = self.process_content(&ch.content)?;
        }
        Ok(())
    }

    fn process_content(&self, content: &str) -> Result<String, Error> {
        let parser = mdbook::utils::new_cmark_parser(content);
        let mut converter = EventQuoteConverter::new();
        let events = parser.map(|event| converter.convert(event));
        let mut buffer = String::new();
        pulldown_cmark_to_cmark::cmark(events, &mut buffer, None)
            .map_err(|err| Error::new(err).context("Markdown serialization failed"))?;
        Ok(buffer)
    }
}

impl Preprocessor for CurlyQuotes {
    fn name(&self) -> &str {
        "curly-quotes-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut err = None;
        book.for_each_mut(|item| {
            self.process_item(item).unwrap_or_else(|e| {
                if err.is_none() {
                    err = Some(e);
                }
            })
        });
        if let Some(err) = err {
            Err(err)
        } else {
            Ok(book)
        }
    }
}

struct EventQuoteConverter;

impl EventQuoteConverter {
    fn new() -> Self {
        EventQuoteConverter
    }

    fn convert<'a>(&mut self, event: Event<'a>) -> Event<'a> {
        match event {
            Event::Text(ref text) => Event::Text(CowStr::from(convert_quotes_to_curly(text))),
            _ => event,
        }
    }
}

fn convert_quotes_to_curly(original_text: &str) -> String {
    // We'll consider the start to be "whitespace".
    let mut preceded_by_whitespace = true;

    original_text
        .chars()
        .map(|original_char| {
            let converted_char = match original_char {
                '\'' => {
                    if preceded_by_whitespace {
                        '‘'
                    } else {
                        '’'
                    }
                }
                '"' => {
                    if preceded_by_whitespace {
                        '“'
                    } else {
                        '”'
                    }
                }
                _ => original_char,
            };

            preceded_by_whitespace = original_char.is_whitespace();

            converted_char
        })
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn process_content() {
        let preprocessor = CurlyQuotes::new();
        let new_content = preprocessor
            .process_content("[\"example\"](https://www.rust-lang.org/\")")
            .unwrap();
        assert_eq!(new_content, "[“example”](https://www.rust-lang.org/\")")
    }
}
