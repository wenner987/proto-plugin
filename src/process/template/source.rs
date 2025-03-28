use anyhow::Result;
use handlebars::Handlebars;
use protobuf::descriptor::DescriptorProto;
use crate::reader::TemplateReader;
use crate::process::options::OptionsExtractor;
use serde::Serialize;
use super::constants::*;
use super::validation::ValidationGenerator;
use super::field::FieldGenerator;

#[derive(Debug, Serialize)]
struct TemplateData {
    message_name: String,
    csv_file_name: String,
    header: String,
    read_fields: String,
    write_fields: String,
}

pub(crate) struct SourceGenerator<'a> {
    extractor: &'a dyn OptionsExtractor,
    handlebars: Handlebars<'a>,
}

impl<'a> SourceGenerator<'a> {
    pub fn new(extractor: &'a dyn OptionsExtractor) -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        Self::register_templates(&mut handlebars);
        Self { extractor, handlebars }
    }

    fn register_templates(handlebars: &mut Handlebars) {
        // 注册字段相关模板
        handlebars.register_template_string("field_read", 
            std::str::from_utf8(FIELD_READ_TEMPLATE).unwrap()).unwrap();
        handlebars.register_template_string("field_validation",
            std::str::from_utf8(FIELD_VALIDATION_TEMPLATE).unwrap()).unwrap();

        // 注册验证模板
        let templates: serde_json::Value = serde_json::from_slice(VALIDATION_TEMPLATES)
            .expect("Failed to parse validation templates");
        
        // 注册基本验证模板
        for (name, template) in templates.as_object().unwrap().iter() {
            if name != "range" {
                handlebars.register_template_string(
                    &format!("validation_{}", name),
                    template.as_str().unwrap()
                ).unwrap();
            }
        }
        
        // 注册范围验证模板
        let range_templates = templates["range"].as_object().unwrap();
        for (type_name, template) in range_templates.iter() {
            handlebars.register_template_string(
                &format!("validation_range_{}", type_name),
                template.as_str().unwrap()
            ).unwrap();
        }

        // 注册源代码模板
        let reader = TemplateReader::new("templates");
        if let Ok(template) = reader.read_source_template() {
            handlebars.register_template_string("source", &template).unwrap();
        }
    }

    pub fn generate(&mut self, message: &DescriptorProto, csv_file_name: &str) -> Result<String> {
        let validation_generator = ValidationGenerator::new(self.extractor, &self.handlebars);
        let field_generator = FieldGenerator::new(&self.handlebars, validation_generator);
        
        let field_names: Vec<_> = message.field.iter()
            .map(|f| format!("\"{}\"", f.name()))
            .collect();
        
        let data = TemplateData {
            message_name: message.name().to_string(),
            csv_file_name: csv_file_name.to_string(),
            header: field_names.join(", "),
            read_fields: field_generator.generate_read_fields(message),
            write_fields: field_generator.generate_write_fields(message),
        };

        // 添加调试日志
        println!("生成的数据: {:?}", data);
        println!("模板列表: {:?}", self.handlebars.get_templates());

        Ok(self.handlebars.render("source", &data)?)
    }
}