use lopdf;
use std::{
    ops::Range,
    path::{Path, PathBuf},
};

pub struct PDF(lopdf::Document);

impl PDF {
    pub fn new(path: &PathBuf) -> Self {
        Self(lopdf::Document::load(path).unwrap())
    }

    /// Returns page length of the document
    pub fn length(&self) -> usize {
        self.0.page_iter().count()
    }

    /// Extract a given range into a new document
    fn extract_range(&self, range: Range<usize>) -> Result<Self, lopdf::Error> {
        let pages_map = self.0.get_pages(); // BTreeMap<u32, ObjectId>, 1-based page numbers
        // Collect all page numbers that are not in the requested range
        let pages_to_delete: Vec<u32> = pages_map
            .keys()
            .filter(|&&page_num_1based| {
                // convert to 0-based
                let idx = (page_num_1based - 1) as usize;
                !range.contains(&idx)
            })
            .cloned()
            .collect();
        // Clone the original document so we don't mutate it
        let mut new_inner_document = self.0.clone();
        new_inner_document.delete_pages(&pages_to_delete);
        Ok(Self(new_inner_document))
    }

    /// Extract a series of ranges, each into a new document
    pub fn extract_ranges(&self, ranges: &[Range<usize>]) -> Result<Vec<Self>, lopdf::Error> {
        ranges
            .iter()
            .map(|range| self.extract_range(range.clone()))
            .collect()
    }

    /// Splits the PDF into single-page PDFs, returning a Vec<PDF>
    pub fn split_into_single_pages(&self) -> Result<Vec<Self>, lopdf::Error> {
        let length = self.length();

        // Create a Vec of single-page ranges: 0..1, 1..2, 2..3, ..., (length-1)..length
        let single_page_ranges: Vec<Range<usize>> = (0..length).map(|i| i..i + 1).collect();

        // Extract each range as a separate PDF
        self.extract_ranges(&single_page_ranges)
    }

    pub fn flush(&mut self, path: &Path) -> Result<(), lopdf::Error> {
        self.0.save(path)?;
        Ok(())
    }
}
