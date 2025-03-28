use anyhow::Result;
use protobuf::descriptor::DescriptorProto;
use crate::reader::TemplateReader;

pub(crate) struct HeaderGenerator;

impl HeaderGenerator {
    pub fn generate(message: &DescriptorProto, _csv_file_name: &str) -> Result<String> {
        let reader = TemplateReader::new("templates");
        let template = reader.read_header_template()?;
        Ok(template.replace("{message_name}", message.name()))
    }
}