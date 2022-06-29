/// A struct giving details of where a piece of data was declared in a source.
pub struct SourceLocation {
    ln: usize,
    col_start: usize,
    col_end: usize,
}

impl SourceLocation {
    /// Get the line number of the source that this piece of data was declared on.
    pub fn get_ln(&self) -> usize {
        self.ln
    }

    /// Get the column start of the line that this piece of data was declared on.
    pub fn get_col_start(&self) -> usize {
        self.col_start
    }

    /// Get the column end of the line that this piece of data was declared on.
    pub fn get_col_end(&self) -> usize {
        self.col_end
    }

    /// Create a new SourceLocation.
    pub fn new(ln: usize, col_start: usize, col_end: usize) -> Self {
        Self {
            ln,
            col_start,
            col_end,
        }
    }
}
