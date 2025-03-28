use protobuf::descriptor::FieldDescriptorProto;
use handlebars::Handlebars;
use serde_json::{Map, Value as JsonValue};
use super::constants::*;
use super::super::options::OptionsExtractor;
use protobuf::descriptor::field_descriptor_proto::Type;
use crate::process::validator::FieldValidator;

pub(crate) struct ValidationGenerator<'a> {
    extractor: &'a dyn OptionsExtractor,
    handlebars: &'a Handlebars<'a>,
}

impl<'a> ValidationGenerator<'a> {
    pub fn new(extractor: &'a dyn OptionsExtractor, handlebars: &'a Handlebars<'a>) -> Self {
        Self { 
            extractor, 
            handlebars,
        }
    }

    pub fn generate_validation_code(&self, field: &FieldDescriptorProto) -> String {
        let mut validations = Vec::new();
        
        // Create validator for this specific field
        let validator = FieldValidator::new(field);
        
        // Get field value and validate
        if let Some(value) = field.default_value.as_deref() {
            if let Ok(_) = validator.validate(value) {
                // Add validation code if validation passes
                // TODO: Generate appropriate validation code
            }
        }
        
        self.add_not_null_validation(&mut validations, field);
        self.add_range_validations(&mut validations, field);
        self.add_in_range_validation(&mut validations, field);

        if validations.is_empty() {
            String::new()
        } else {
            validations.join("\n        ") + "\n        "
        }
    }

    fn add_not_null_validation(&self, validations: &mut Vec<String>, field: &FieldDescriptorProto) {
        if self.extractor.is_field_not_null(field) {
            let mut data = Map::new();
            data.insert("error_message".to_string(), JsonValue::String(
                ERR_FIELD_EMPTY.replace("{}", &field.name())
            ));
            validations.push(self.handlebars.render("validation_not_null", &data).unwrap());
        }
    }

    fn add_range_validations(&self, validations: &mut Vec<String>, field: &FieldDescriptorProto) {
        // 大于验证
        if let Some(gt) = self.extractor.get_field_gt(field) {
            let mut data = Map::new();
            data.insert("value".to_string(), JsonValue::String(gt.to_string()));
            data.insert("error_message".to_string(), JsonValue::String(
                ERR_VALUE_GT.replace("{}", &gt.to_string())
            ));
            validations.push(self.handlebars.render("validation_gt", &data).unwrap());
        }

        // 小于验证
        if let Some(lt) = self.extractor.get_field_lt(field) {
            let mut data = Map::new();
            data.insert("value".to_string(), JsonValue::String(lt.to_string()));
            data.insert("error_message".to_string(), JsonValue::String(
                ERR_VALUE_LT.replace("{}", &lt.to_string())
            ));
            validations.push(self.handlebars.render("validation_lt", &data).unwrap());
        }

        // 大于等于验证
        if let Some(gte) = self.extractor.get_field_gte(field) {
            let mut data = Map::new();
            data.insert("value".to_string(), JsonValue::String(gte.to_string()));
            data.insert("error_message".to_string(), JsonValue::String(
                ERR_VALUE_GTE.replace("{}", &gte.to_string())
            ));
            validations.push(self.handlebars.render("validation_gte", &data).unwrap());
        }

        // 小于等于验证
        if let Some(lte) = self.extractor.get_field_lte(field) {
            let mut data = Map::new();
            data.insert("value".to_string(), JsonValue::String(lte.to_string()));
            data.insert("error_message".to_string(), JsonValue::String(
                ERR_VALUE_LTE.replace("{}", &lte.to_string())
            ));
            validations.push(self.handlebars.render("validation_lte", &data).unwrap());
        }
    }

    fn add_in_range_validation(&self, validations: &mut Vec<String>, field: &FieldDescriptorProto) {
        if let Some(range_str) = self.extractor.get_field_in_range(field) {
            let mut data = Map::new();
            data.insert("error_message".to_string(), JsonValue::String(
                ERR_VALUE_IN_RANGE.replace("{}", &range_str)
            ));
            
            // match field.r#type_ {
            //     Type::TYPE_STRING => {
            //         data.insert("values".to_string(), JsonValue::String(
            //             range_str.split(',')
            //                 .map(|s| format!("\"{}\"", s.trim()))
            //                 .collect::<Vec<_>>()
            //                 .join(", ")
            //         ));
            //         validations.push(self.handlebars.render("validation_range_string", &data).unwrap());
            //     },
            //     Type::TYPE_INT32 => {
            //         data.insert("values".to_string(), JsonValue::String(
            //             range_str.split(',')
            //                 .map(|s| s.trim())
            //                 .collect::<Vec<_>>()
            //                 .join(", ")
            //         ));
            //         validations.push(self.handlebars.render("validation_range_int", &data).unwrap());
            //     },
            //     _ => {}
            // }
        }
    }
}