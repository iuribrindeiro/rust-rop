use crate::rop::{error, errors, succeed, MapFailures, RopResult};

use super::StrError;

impl<TResult, TErr> RunValidators<TResult, TErr> for [fn(&TResult) -> RopResult<TResult, TErr>; 2] {
    fn run_validators(self, value: TResult) -> RopResult<TResult, TErr> {
        let results: Vec<RopResult<TResult, TErr>> = self.into_iter().map(|f| f(&value)).collect();
        let errs = results.combine_errors();
        if errs.is_empty() {
            return succeed(value);
        }
        errors(errs)
    }
}

pub fn str_is_not_empty(str: &String) -> RopResult<String, StrError> {
    if str.is_empty() {
        return error(StrError::NotEmpty);
    }

    return succeed(str.into());
}

pub fn str_max_len(max: usize, str: &String) -> RopResult<String, StrError> {
    if !str.is_empty() && str.len() <= max {
        return succeed(str.into());
    }
    return error(StrError::StrMax(max));
}

pub trait RunValidators<TResult, TErr> {
    fn run_validators(self, value: TResult) -> RopResult<TResult, TErr>;
}
