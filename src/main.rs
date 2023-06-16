use value_objects::{StrMax115, TapRError, StrError, MapR};

mod value_objects {
    type RopResult<TResult, TErr> = Result<(TResult, Vec<TErr>), Vec<TErr>>;

    pub struct StrMax115 {
        value: String,
    }

    pub enum StrError {
        StrMax(usize),
        NotEmpty
    }

    impl StrMax115 {
        pub fn new(value: String) -> RopResult<StrMax115, StrError> {
            [
                str_is_not_empty, 
                |x: &String| str_max_len(115, x)
            ].run_validators(value.into())
            .map_r(|s| StrMax115 { value: s.to_string() })
        }

        pub fn value(&self) -> &String {
            &self.value
        }
    }

    pub trait MapFailures<TResult, TErr> {
        fn combine_errors(self) -> Vec<TErr>;    
    }

    pub trait MapR<TResult, TResult2, TErr> {
        fn map_r(self, func: fn(&TResult) -> TResult2) -> RopResult<TResult2, TErr>; 
    }

    pub trait TapR<TResult, TErr> {
        fn tap_r(self, func: fn(&TResult) -> ()) -> RopResult<TResult, TErr>; 
    }

    impl<TResult, TErr> TapR<TResult, TErr> for RopResult<TResult, TErr> {
        fn tap_r(self, func: fn(&TResult) -> ()) -> RopResult<TResult, TErr> {
            match self {
                Ok((v, errs)) => {
                    func(&v);
                    Ok((v, errs))
                },
                Err(errs) => Err(errs)
            }
        }
    }

    pub trait RunValidators<TResult, TErr> {
        fn run_validators(self, value: TResult) -> RopResult<TResult, TErr>;
    }

    pub trait TapRError<TResult, TErr> {
        fn tap_error(self, func: fn(&Vec<TErr>) -> ()) -> RopResult<TResult, TErr>; 
    }

    impl<TResult, TErr> TapRError<TResult, TErr> for RopResult<TResult, TErr> {
        fn tap_error(self, func: fn(&Vec<TErr>) -> ()) -> RopResult<TResult, TErr> {
            match self {
                Ok((v, errs)) => Ok((v, errs)),
                Err(errs) => {
                    func(&errs);
                    Err(errs)
                }
            }
        }
    }

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

    impl<TResult, TErr> MapFailures<TResult, TErr> for Vec<RopResult<TResult, TErr>> {
        fn combine_errors(self) -> Vec<TErr> {
            self.into_iter()
                .flat_map(|x| {
                    match x {
                        Ok(_) => vec![],
                        Err(errs) => errs
                    }
                }).collect()
        }
    }

    impl<TResult, TResult2, TErr> MapR<TResult, TResult2, TErr> for RopResult<TResult, TErr> {
        fn map_r(self, func: fn(&TResult) -> TResult2) -> Result<(TResult2, Vec<TErr>), Vec<TErr>> {
            match self {
                Ok((v, _)) => succeed(func(&v)),
                Err(e) => errors(e)
            }
        }
    }

    fn str_is_not_empty(str: &String) -> Result<(String, Vec<StrError>), Vec<StrError>> {
        if str.is_empty() {
            return error(StrError::NotEmpty);
        }

        return succeed(str.into());
    }

    fn str_max_len(max: usize, str: &String) -> Result<(String, Vec<StrError>), Vec<StrError>> {
        if !str.is_empty() && str.len() <= max {
            return succeed(str.into());
        }
        return error(StrError::StrMax(max));
    }

    fn error<T, E>(err: E) -> Result<(T, Vec<E>), Vec<E>> {
        Err(vec![err])
    }

    fn errors<T, E>(err: Vec<E>) -> Result<(T, Vec<E>), Vec<E>> {
        Err(err)
    }

    fn succeed<T, E>(value: T) -> RopResult<T, E> {
        Ok((value, vec![]))
    }
}

fn main() {
    let str_result = StrMax115::new(("Iuri Brindeiro").to_string())
        .tap_error(|x| x.iter().for_each(|e| print_error(e)))
        .map_r(|r| r.value().to_string());

    match str_result {
        Ok((v, _)) => println!("Success value is {}", v),
        Err(_) => println!("Fix the errors above")
    }
}

fn print_error(err: &StrError) {
    match err {
        StrError::StrMax(max) => println!("Value can't have more than {} characters", max),
        StrError::NotEmpty => println!("Value can't be left blank"),
    }
}