pub type RopResult<TResult, TErr> = Result<(TResult, Vec<TErr>), Vec<TErr>>;

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
            }
            Err(errs) => Err(errs),
        }
    }
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

impl<TResult, TErr> MapFailures<TResult, TErr> for Vec<RopResult<TResult, TErr>> {
    fn combine_errors(self) -> Vec<TErr> {
        self.into_iter()
            .flat_map(|x| match x {
                Ok(_) => vec![],
                Err(errs) => errs,
            })
            .collect()
    }
}

impl<TResult, TResult2, TErr> MapR<TResult, TResult2, TErr> for RopResult<TResult, TErr> {
    fn map_r(self, func: fn(&TResult) -> TResult2) -> Result<(TResult2, Vec<TErr>), Vec<TErr>> {
        match self {
            Ok((v, _)) => succeed(func(&v)),
            Err(e) => errors(e),
        }
    }
}

pub fn error<T, E>(err: E) -> Result<(T, Vec<E>), Vec<E>> {
    Err(vec![err])
}

pub fn errors<T, E>(err: Vec<E>) -> Result<(T, Vec<E>), Vec<E>> {
    Err(err)
}

pub fn succeed<T, E>(value: T) -> RopResult<T, E> {
    Ok((value, vec![]))
}
