use std::collections::HashMap;
use anyhow::Result;
use log::info;
use protobuf::plugin::code_generator_response::File as ProtoFile;

pub struct FileWriter {
    files: HashMap<String, ProtoFile>,
}

impl FileWriter {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, name: &str, content: &str) -> ProtoFile {
        let mut file = ProtoFile::new();
        file.set_name(name.to_string());
        file.set_content(content.to_string());
        self.files.insert(name.to_string(), file.clone());
        
        file
    }

    pub fn generate_header(&mut self, name: &str, content: &str) -> Result<ProtoFile> {
        let file_name = format!("{}_csv.h", name.to_lowercase());
        info!("Generating header file: {}", file_name);
        Ok(self.add_file(&file_name, content))
    }

    pub fn generate_source(&mut self, name: &str, content: &str) -> Result<ProtoFile> {
        let file_name = format!("{}_csv.cc", name.to_lowercase());
        info!("Generating source file: {}", file_name);
        Ok(self.add_file(&file_name, content))
    }

}