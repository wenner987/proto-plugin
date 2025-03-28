use log::debug;
use protobuf::{
    descriptor::{DescriptorProto, FieldDescriptorProto},
    UnknownValueRef,
};

pub trait OptionsExtractor {
    fn get_csv_filename(&self, message: &DescriptorProto) -> String;
    fn get_index(&self, message: &DescriptorProto) -> Vec<String>;
    fn should_process_message(&self, message: &DescriptorProto) -> bool;
    fn is_field_not_null(&self, field: &FieldDescriptorProto) -> bool;
    fn get_field_lt(&self, field: &FieldDescriptorProto) -> Option<i32>;
    fn get_field_gt(&self, field: &FieldDescriptorProto) -> Option<i32>;
    fn get_field_lte(&self, field: &FieldDescriptorProto) -> Option<i32>;
    fn get_field_gte(&self, field: &FieldDescriptorProto) -> Option<i32>;
    fn get_field_in_range(&self, field: &FieldDescriptorProto) -> Option<String>;
}

impl OptionsExtractor for super::processor::ProtoProcessor {
    fn get_csv_filename(&self, message: &DescriptorProto) -> String {
        message
            .options
            .as_ref()
            .and_then(|opt| {
                opt.special_fields
                    .unknown_fields()
                    .iter()
                    .find(|o| o.0 == 50000)
                    .map(|o| {
                        if let UnknownValueRef::LengthDelimited(v) = o.1 {
                            String::from_utf8_lossy(v).to_string()
                        } else {
                            String::new()
                        }
                    })
            })
            .unwrap_or(format!("{}.csv", message.name().to_lowercase()))
    }

    fn should_process_message(&self, message: &DescriptorProto) -> bool {
        message
            .options
            .as_ref()
            .and_then(|opt| {
                opt.special_fields
                    .unknown_fields()
                    .iter()
                    .find(|o| o.0 == 50001)
                    .map(|o| {
                        debug!("should_process_message: {:?}", o.1);
                        if let UnknownValueRef::Varint(v) = o.1 {
                            v == 1
                        } else {
                            false
                        }
                    })
            })
            .unwrap_or(false)
    }

    fn get_index(&self, message: &DescriptorProto) -> Vec<String> {
        message
            .options
            .as_ref()
            .and_then(|opt| {
                opt.special_fields
                    .unknown_fields()
                    .iter()
                    .find(|o| o.0 == 50002)
                    .map(|o| {
                        if let UnknownValueRef::LengthDelimited(v) = o.1 {
                            String::from_utf8_lossy(v)
                                .to_string()
                                .split(",")
                                .map(|s| s.to_string())
                                .collect()
                        } else {
                            Vec::new()
                        }
                    })
            })
            .unwrap_or(Vec::new())
    }

    fn is_field_not_null(&self, field: &FieldDescriptorProto) -> bool {
        field
            .options
            .as_ref()
            .and_then(|opt| {
                opt.uninterpreted_option
                    .iter()
                    .find(|o| {
                        o.name
                            .iter()
                            .any(|n| n.name_part == Some("not_null".to_string()))
                    })
                    .and_then(|o| o.string_value.clone())
            })
            .map(|s| s == b"true")
            .unwrap_or(false)
    }

    fn get_field_lt(&self, field: &FieldDescriptorProto) -> Option<i32> {
        field
            .options
            .as_ref()
            .and_then(|opt| {
                opt.uninterpreted_option
                    .iter()
                    .find(|o| o.name.iter().any(|n| n.name_part == Some("lt".to_string())))
                    .and_then(|o| o.string_value.clone())
            })
            .and_then(|s| String::from_utf8(s).ok())
            .and_then(|s| s.parse().ok())
    }

    fn get_field_gt(&self, field: &FieldDescriptorProto) -> Option<i32> {
        field
            .options
            .as_ref()
            .and_then(|opt| {
                opt.uninterpreted_option
                    .iter()
                    .find(|o| o.name.iter().any(|n| n.name_part == Some("gt".to_string())))
                    .and_then(|o| o.string_value.clone())
            })
            .and_then(|s| String::from_utf8(s).ok())
            .and_then(|s| s.parse().ok())
    }

    fn get_field_lte(&self, field: &FieldDescriptorProto) -> Option<i32> {
        field
            .options
            .as_ref()
            .and_then(|opt| {
                opt.uninterpreted_option
                    .iter()
                    .find(|o| {
                        o.name
                            .iter()
                            .any(|n| n.name_part == Some("lte".to_string()))
                    })
                    .and_then(|o| o.string_value.clone())
            })
            .and_then(|s| String::from_utf8(s).ok())
            .and_then(|s| s.parse().ok())
    }

    fn get_field_gte(&self, field: &FieldDescriptorProto) -> Option<i32> {
        field
            .options
            .as_ref()
            .and_then(|opt| {
                opt.uninterpreted_option
                    .iter()
                    .find(|o| {
                        o.name
                            .iter()
                            .any(|n| n.name_part == Some("gte".to_string()))
                    })
                    .and_then(|o| o.string_value.clone())
            })
            .and_then(|s| String::from_utf8(s).ok())
            .and_then(|s| s.parse().ok())
    }

    fn get_field_in_range(&self, field: &FieldDescriptorProto) -> Option<String> {
        field
            .options
            .as_ref()
            .and_then(|opt| {
                opt.uninterpreted_option
                    .iter()
                    .find(|o| {
                        o.name
                            .iter()
                            .any(|n| n.name_part == Some("range".to_string()))
                    })
                    .and_then(|o| o.string_value.clone())
            })
            .and_then(|s| String::from_utf8(s).ok())
    }
}
