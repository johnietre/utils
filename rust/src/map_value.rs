pub trait MapValue: Sized {
    fn map_value<U>(self, f: impl Fn(Self) -> U) -> U {
        f(self)
    }
}

impl<T: Sized> MapValue for T {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_value() {
        let v = 1i32;
        let mv = v.map_value(|i| i + 4);
        assert_eq!(mv, v + 4);

        let v = 1i32;
        let mv = v.map_value(|mut i| {
            i += 4;
            i
        });
        assert_eq!(mv, v + 4);
    }
}
