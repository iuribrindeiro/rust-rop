use crate::rop::{MapR, RopResult};

use self::value_objects_utils::{str_is_not_empty, str_max_len, RunValidators};

mod value_objects_utils;

pub struct StrMax115 {
    value: String,
}

pub enum StrError {
    StrMax(usize),
    NotEmpty,
}

impl StrMax115 {
    pub fn new(value: String) -> RopResult<StrMax115, StrError> {
        [str_is_not_empty, |x: &String| str_max_len(115, x)]
            .run_validators(value)
            .map_r(|s| StrMax115 {
                value: s.to_string(),
            })
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
