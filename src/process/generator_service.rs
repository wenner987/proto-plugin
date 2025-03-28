use anyhow::Result;
use log::{info, debug};
use protobuf::plugin::{code_generator_response::File, CodeGeneratorRequest, CodeGeneratorResponse};
use crate::process::template::SourceGenerator;
use super::processor::ProtoProcessor;
use crate::process::options::OptionsExtractor;

pub struct GeneratorService {
    processor: ProtoProcessor,
}

impl GeneratorService {
    pub fn new() -> Self {
        Self {
            processor: ProtoProcessor::new(),
        }
    }

    pub fn generate(&self, request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse> {
        let mut response = CodeGeneratorResponse::default();
        
        for file in request.proto_file.iter() {
            if file.name().contains("descriptor.proto") {
                continue;
            }
            
            debug!("Processing file: {}", file.name());
            
            for message in file.message_type.iter() {
                debug!("Checking message options: {:?}", message);
                
                if !self.processor.should_process_message(message) {
                    debug!("Skip message: {} (Mark not found)", message.name());
                    continue;
                }

                info!("Generating CSV reader: {}", message.name());
                
                let csv_file_name = self.processor.get_csv_filename(message);
                
                let mut generator = SourceGenerator::new(&self.processor);
                match generator.generate(message, &csv_file_name) {
                    Ok(content) => {
                        let mut output_file = File::default();
                        output_file.name = Some(format!("{}_csv_reader.h", message.name().to_lowercase()));
                        output_file.content = Some(content);
                        response.file.push(output_file);
                        
                        info!("Successfully generated file: {}_csv_reader.h", message.name().to_lowercase());
                    }
                    Err(e) => {
                        info!("Generation failed: {}", e);
                    }
                }
            }
        }

        info!("Number of generated files: {}", response.file.len());
        Ok(response)
    }
}