
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
        self.map(|r| r.map_err(E1::into))
            .map_err(E2::into)
            .flatten()
    }
}

