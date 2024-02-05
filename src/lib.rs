use mdbook::book::Book;
use mdbook::book::{BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

/// The Obsidian preprocessor.
pub struct Obsidian;

impl Default for Obsidian {
    fn default() -> Self {
        Self::new()
    }
}

impl Obsidian {
    pub fn new() -> Obsidian {
        Obsidian
    }
}

impl Preprocessor for Obsidian {
    fn name(&self) -> &str {
        "Obsidian"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut error: Option<Error> = None;
        book.for_each_mut(|item: &mut BookItem| {
            if error.is_some() {
                return;
            }
            if let BookItem::Chapter(ref mut chapter) = *item {
                if let Err(err) = handle_chapter(chapter) {
                    error = Some(err)
                }
            }
        });
        error.map_or(Ok(book), Err)
    }

    /// Check whether we support the specified renderer
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

/// Apply to all chapters
fn handle_chapter(_chapter: &mut Chapter) -> Result<(), Error> {
    // Add your additional syntax parsing here

    Ok(())
}
