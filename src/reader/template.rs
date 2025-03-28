use anyhow::Result;
use std::path::PathBuf;

pub struct TemplateReader {
    template_dir: PathBuf,
}

impl TemplateReader {
    pub fn new(template_dir: &str) -> Self {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(template_dir);
        Self {
            template_dir: path,
        }
    }

    pub fn read_header_template(&self) -> Result<String> {
        let template_path = self.template_dir.join("csv_reader.h.template");
        std::fs::read_to_string(&template_path)
            .map_err(|e| anyhow::anyhow!("Failed to read header template: {}, path: {}", e, template_path.display()))
    }

    pub fn read_source_template(&self) -> Result<String> {
        let template_path = self.template_dir.join("csv_reader.cc.template");
        std::fs::read_to_string(&template_path)
            .map_err(|e| anyhow::anyhow!("Failed to read source template: {}, path: {}", e, template_path.display()))
    }

    pub fn read_template(&self, template_name: &str) -> Result<String> {
        let template_path = self.template_dir.join(template_name);
        Ok(std::fs::read_to_string(template_path)?)
    }
}