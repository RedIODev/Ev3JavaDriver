use std::convert;

pub trait Flatten<T,E> {
    fn flatten(self) -> Result<T, E>;
}

impl<T,E> Flatten<T,E> for Result<Result<T,E>,E> {
    fn flatten(self) -> Result<T, E> {
        self.and_then(convert::identity)
    }
}

pub trait FlattenInto<T,E1,E2>
{
    fn flatten_into<E>(self) -> Result<T,E>
    where 
        E1: Into<E>,
        E2: Into<E>;
}

impl<T, E1, E2> FlattenInto<T,E1,E2> for Result<Result<T,E1>, E2> 
     
{
    fn flatten_into<E>(self) -> Result<T,E> 
    where 
        E1: Into<E>,
        E2: Into<E>
    {
        let r = self.map(|r| r.map_err(E1::into))
            .map_err(E2::into);
        Flatten::flatten(r)
    }
}

