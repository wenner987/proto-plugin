use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto, field_descriptor_proto::Type};
use handlebars::Handlebars;
use serde_json::{Map, Value as JsonValue};
use super::validation::ValidationGenerator;

pub(crate) struct FieldGenerator<'a> {
    handlebars: &'a Handlebars<'a>,
    validation_generator: ValidationGenerator<'a>,
}

impl<'a> FieldGenerator<'a> {
    pub fn new(handlebars: &'a Handlebars<'a>, validation_generator: ValidationGenerator<'a>) -> Self {
        Self { handlebars, validation_generator }
    }

    pub fn generate_read_fields(&self, message: &DescriptorProto) -> String {
        let validations = message.field.iter()
            .map(|f| self.generate_field_validation(f))
            .collect::<Vec<_>>()
            .join("\n\n");
            
        let reads = message.field.iter()
            .enumerate()
            .map(|(i, f)| self.generate_field_read(i, f))
            .collect::<Vec<_>>()
            .join("\n        ");
            
        format!("{}\n\n        {}", validations, reads)
    }

    pub fn generate_write_fields(&self, message: &DescriptorProto) -> String {
        let fields: Vec<_> = message.field.iter()
            .enumerate()
            .map(|(i, f)| format!("doc.SetCell({}, row, message.{}());", i, f.name()))
            .collect();
        format!("int row = 0;\nfor (const auto& message : messages) {{\n    {} \n    row++;\n}}", 
            fields.join("\n    "))
    }

    fn generate_field_read(&self, index: usize, field: &FieldDescriptorProto) -> String {
        let mut data = Map::new();
        data.insert("field_name".to_string(), JsonValue::String(field.name().to_string()));
        data.insert("index".to_string(), JsonValue::String(index.to_string()));
        // data.insert("type_name".to_string(), JsonValue::String(match field.r#type() {
        //     Type::String => "std::string".to_string(),
        //     Type::Int32 => "int32_t".to_string(),
        //     _ => return String::new()
        // }));

        self.handlebars.render("field_read", &data).unwrap()
    }

    fn generate_field_validation(&self, field: &FieldDescriptorProto) -> String {
        let type_name = self.map_field_type(field);
        let validations = self.generate_validation_rules(field);
        
        let data = self.prepare_validation_data(field, &type_name, &validations);
        self.handlebars.render("field_validation", &data).unwrap()
    }

    fn map_field_type(&self, field: &FieldDescriptorProto) -> String {
        match field.r#type() {
            Type::String => "std::string".to_string(),
            Type::Int32 => "int32_t".to_string(),
            _ => String::new()
        }
    }

    fn generate_validation_rules(&self, field: &FieldDescriptorProto) -> String {
        self.validation_generator.generate_validation_code(field)
    }

    fn prepare_validation_data(&self, field: &FieldDescriptorProto, type_name: &str, validations: &str) -> Map<String, JsonValue> {
        let mut data = Map::new();
        data.insert("field_name".to_string(), JsonValue::String(field.name().to_string()));
        data.insert("type_name".to_string(), JsonValue::String(type_name.to_string()));
        data.insert("validations".to_string(), JsonValue::String(validations.to_string()));
        data
    }
}