
pub trait FlattenInto<T, E>
{
    fn flatten_into(self) -> Result<T,E>;
}

impl<T,E, E1, E2> FlattenInto<T,E> for Result<Result<T,E1>, E2> 
where E: From<E1> + From<E2>     
{
    fn flatten_into(self) -> Result<T,E> {
        self.map(|r| r.map_err(E::from))
            .map_err(E::from)
            .flatten()
    }
}