use protobuf::descriptor::DescriptorProto;
use anyhow::Result;

use crate::process::template::{HeaderGenerator, SourceGenerator};

impl super::processor::ProtoProcessor {
    pub(crate) fn generate_header_content(&self, message: &DescriptorProto, csv_file_name: &str) -> Result<String> {
        HeaderGenerator::generate(message, csv_file_name)
    }

    pub(crate) fn generate_source_content(&self, message: &DescriptorProto, csv_file_name: &str) -> Result<String> {
        let mut generator = SourceGenerator::new(self);
        generator.generate(message, csv_file_name)
    }
}