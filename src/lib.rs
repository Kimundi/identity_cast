#![doc = include_str!("../README.md")]

/// Checks if `T == U`, and returns `Ok(U)` if they are.
pub fn into_same<T, U>(val: T) -> Result<U, T>
where
    T: 'static,
    U: 'static,
{
    let mut opt: Option<T> = Some(val);

    {
        let any: &mut dyn std::any::Any = &mut opt;

        if let Some(s) = any.downcast_mut::<Option<U>>() {
            return Ok(s.take().unwrap());
        }
    }

    Err(opt.unwrap())
}

/// Checks if `T == U`, and returns `Ok(&U)` if they are.
pub fn as_same<T, U>(val: &T) -> Result<&U, &T>
where
    T: 'static,
    U: 'static,
{
    let any: &dyn std::any::Any = val;
    any.downcast_ref().ok_or(val)
}

/// Checks if `T == U`, and returns `Ok(&mut U)` if they are.
pub fn as_mut_same<T, U>(val: &mut T) -> Result<&mut U, &mut T>
where
    T: 'static,
    U: 'static,
{
    let any: &dyn std::any::Any = val;
    if any.is::<U>() {
        return Ok((val as &mut dyn std::any::Any).downcast_mut().unwrap());
    }
    Err(val)
}

pub trait IdentityCast: Sized + 'static {
    /// Checks if `Self == U`, and returns `Ok(U)` if they are.
    fn into_same<U>(self) -> Result<U, Self>
    where
        U: 'static,
    {
        into_same(self)
    }

    /// Checks if `Self == U`, and returns `Ok(U)` if they are.
    fn as_same<U>(&self) -> Result<&U, &Self>
    where
        U: 'static,
    {
        as_same(self)
    }

    /// Checks if `Self == U`, and returns `Ok(U)` if they are.
    fn as_mut_same<U>(&mut self) -> Result<&mut U, &mut Self>
    where
        U: 'static,
    {
        as_mut_same(self)
    }
}

impl<T: Sized + 'static> IdentityCast for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned() {
        assert_eq!(into_same(42).ok(), Some(42));
        assert_eq!(into_same(false).ok(), None::<i32>);
        assert_eq!(42.into_same().ok(), Some(42));
        assert_eq!(false.into_same::<i32>().ok(), None);
    }

    #[test]
    fn reference() {
        assert_eq!(as_same(&42).ok(), Some(&42));
        assert_eq!(as_same(&false).ok(), None::<&i32>);
        assert_eq!(42.as_same().ok(), Some(&42));
        assert_eq!(false.as_same::<i32>().ok(), None);
    }

    #[test]
    fn mut_reference() {
        assert_eq!(as_mut_same(&mut 42).ok(), Some(&mut 42));
        assert_eq!(as_mut_same(&mut false).ok(), None::<&mut i32>);
        assert_eq!(42.as_mut_same().ok(), Some(&mut 42));
        assert_eq!(false.as_mut_same::<i32>().ok(), None);
    }
}
