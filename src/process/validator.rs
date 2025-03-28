use protobuf::descriptor::FieldDescriptorProto;
use anyhow::{Result, anyhow};

pub struct FieldValidator<'a> {
    field: &'a FieldDescriptorProto,
}

impl<'a> FieldValidator<'a> {
    pub fn new(field: &'a FieldDescriptorProto) -> Self {
        Self { field }
    }

    pub fn validate(&self, value: &str) -> Result<()> {
        // Not null check
        if self.is_not_null() && value.is_empty() {
            return Err(anyhow!("Field cannot be empty: {}", self.field.name()));
        }

        // Number range check
        if let Ok(num_value) = value.parse::<i32>() {
            if let Some(lt) = self.get_lt() {
                if num_value >= lt {
                    return Err(anyhow!("Value must be less than {}", lt));
                }
            }
            if let Some(gt) = self.get_gt() {
                if num_value <= gt {
                    return Err(anyhow!("Value must be greater than {}", gt));
                }
            }
            if let Some(lte) = self.get_lte() {
                if num_value > lte {
                    return Err(anyhow!("Value must be less than or equal to {}", lte));
                }
            }
            if let Some(gte) = self.get_gte() {
                if num_value < gte {
                    return Err(anyhow!("Value must be greater than or equal to {}", gte));
                }
            }
        }

        Ok(())
    }

    fn is_not_null(&self) -> bool {
        self.field.options.as_ref()
            .and_then(|opt| opt.uninterpreted_option.iter()
                .find(|o| o.name.iter().any(|n| n.name_part == Some("not_null".to_string())))
                .and_then(|o| o.identifier_value.as_deref()))
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    fn get_lt(&self) -> Option<i32> {
        self.get_number_option("lt")
    }

    fn get_gt(&self) -> Option<i32> {
        self.get_number_option("gt")
    }

    fn get_lte(&self) -> Option<i32> {
        self.get_number_option("lte")
    }

    fn get_gte(&self) -> Option<i32> {
        self.get_number_option("gte")
    }

    fn get_number_option(&self, name: &str) -> Option<i32> {
        self.field.options.as_ref()
            .and_then(|opt| opt.uninterpreted_option.iter()
                .find(|o| o.name.iter().any(|n| n.name_part == Some(name.to_string())))
                .and_then(|o| o.positive_int_value))
            .map(|v| v as i32)
    }
}