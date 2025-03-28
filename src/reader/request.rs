use anyhow::Result;
use log::debug;
use protobuf::{plugin::CodeGeneratorRequest, Message};
use std::io::Read;

pub struct RequestReader;

impl RequestReader {
    pub fn read_request<R: Read>(input: &mut R) -> Result<CodeGeneratorRequest> {
        debug!("Start reading request data...");
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        debug!("Read {} bytes of request data", buf.len());

        debug!("Start decoding request...");
        let mut req = CodeGeneratorRequest::default();
        req.merge_from_bytes(buf.as_slice())
            .map_err(|e| anyhow::anyhow!("Failed to decode request: {}", e))?;
        debug!("Successfully decoded request");
        
        Ok(req)
    }
}