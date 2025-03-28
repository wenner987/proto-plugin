use log::{info, error};
use protoc_gen_cyber::{reader, GeneratorService};
use std::io::{self};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Start processing request...");

    let request = reader::RequestReader::read_request(&mut io::stdin())?;
    let service = GeneratorService::new();

    match service.generate(&request) {
        Ok(response) => {
            info!("=== Generation Result ===");
            info!("Number of generated files: {}", response.file.len());
            
            // let file_writer = writer::FileWriter::new();
            // for file in &response.file {
            //     if let (Some(name), Some(content)) = (&file.name, &file.content) {
            //         file_writer.write_to_file(name, content)?;
            //         info!("Generated file: {}", name);
            //     }
            // }

            info!("Processing completed");
            Ok(())
        }
        Err(e) => {
            error!("Generation failed: {}", e);
            Err(e)
        }
    }
}