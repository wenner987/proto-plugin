pub(crate) const ERR_FIELD_EMPTY: &str = "Field cannot be empty: {}";
pub(crate) const ERR_VALUE_LT: &str = "Value must be less than {}";
pub(crate) const ERR_VALUE_GT: &str = "Value must be greater than {}";
pub(crate) const ERR_VALUE_LTE: &str = "Value must be less than or equal to {}";
pub(crate) const ERR_VALUE_GTE: &str = "Value must be greater than or equal to {}";
pub(crate) const ERR_VALUE_IN_RANGE: &str = "Value must be in range: {}";

pub(crate) const VALIDATION_TEMPLATES: &[u8] = include_bytes!("../../../templates/validation.template");
pub(crate) const FIELD_READ_TEMPLATE: &[u8] = include_bytes!("../../../templates/field_read.template");
pub(crate) const FIELD_VALIDATION_TEMPLATE: &[u8] = include_bytes!("../../../templates/field_validation.template");