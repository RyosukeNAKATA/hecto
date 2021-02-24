pub struct FileType {
    name: String,
    hl_opts: HighlightOptions,
}

#[derive(Default)]
pub struct HighlightOptions {
    pub numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightOptions::default(),
        }
    }
}
