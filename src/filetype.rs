pub struct FileType {
    name: String,
    hl_opts: HighlightOptions,
}

#[derive(Default, Copy, Clone)]
pub struct HighlightOptions {
    numbers: bool,
    strings: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn highlighting_options(&self) -> HighlightOptions {
        self.hl_opts
    }
    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightOptions {
                    numbers: true,
                    strings: true,
                },
            };
        }
        Self::default()
    }
}

impl HighlightOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }
    pub fn strings(self) -> bool {
        self.strings
    }
}
