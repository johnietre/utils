use std::fmt;
use std::process::exit;

#[cfg(not(test))]
mod die_mod {
    /// Die functions the same as panic, but exits using `process::exit` rather than panicking.
    /// If no exit code is given, the default is 1.
    #[macro_export]
    macro_rules! die {
        () => { crate::die!(code: 1) };
        (code: $code:expr $(;)?) => { crate::die($code) };
        (msg: $($args:tt)*) => { crate::die!(code: 1; msg: $($args)*) };
        (code: $code:expr; msg: $($args:tt)*) => {{
            ::std::eprintln!($($args)*);
            crate::die!(code: $code)
        }};
    }
}

#[cfg(test)]
mod die_mod {
    #[macro_export]
    macro_rules! die {
        () => { crate::die!(code: 1) };
        (code: $code:expr $(;)?) => { ::std::format!("CODE: {}", $code) };
        (msg: $($args:tt)*) => { crate::die!(code: 1; msg: $($args)*) };
        (code: $code:expr; msg: $($args:tt)*) => {{
            ::std::format!("CODE: {}|", $code) + ::std::format!($($args)*).as_str()
        }};
    }
}

pub trait OrDie<T>: Sized {
    fn or_die_code_msg(self, code: i32, msg: &str) -> T;

    fn or_die(self) -> T {
        self.or_die_code_msg(1, "")
    }
    fn or_die_code(self, code: i32) -> T {
        self.or_die_code_msg(code, "")
    }
    fn or_die_msg(self, msg: &str) -> T {
        self.or_die_code_msg(1, msg)
    }
}

impl<T> OrDie<T> for Option<T> {
    #[inline]
    fn or_die_code_msg(self, code: i32, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                eprintln!("{msg}");
                exit(code)
            }
        }
    }

    #[inline]
    fn or_die(self) -> T {
        self.or_die_code(1)
    }

    #[inline]
    fn or_die_code(self, code: i32) -> T {
        self.or_die_code_msg(code, "called `Option::or_die()` on a `None` value")
    }

    #[inline]
    fn or_die_msg(self, msg: &str) -> T {
        self.or_die_code_msg(1, msg)
    }
}

impl<T, E: fmt::Debug> OrDie<T> for Result<T, E> {
    #[inline]
    fn or_die_code_msg(self, code: i32, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{msg}: {e:?}");
                exit(code)
            }
        }
    }

    #[inline]
    fn or_die(self) -> T {
        self.or_die_code(1)
    }

    #[inline]
    fn or_die_code(self, code: i32) -> T {
        self.or_die_code_msg(code, "called `Result::or_die()` on an `Err` value")
    }

    #[inline]
    fn or_die_msg(self, msg: &str) -> T {
        self.or_die_code_msg(1, msg)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn die() {
        use crate::die;

        let res = die!();
        assert_eq!(res, "CODE: 1");

        let res = die!(code: 123);
        assert_eq!(res, "CODE: 123");

        let code = 5;
        let res = die!(code: code);
        assert_eq!(res, format!("CODE: {code}"));

        let res = die!(msg: "hello");
        assert_eq!(res, format!("CODE: 1|hello"));

        let res = die!(code: 15; msg: "how are you");
        assert_eq!(res, format!("CODE: 15|how are you"));

        let msg = "goodbye";
        let res = die!(code: code; msg: "{}", msg);
        assert_eq!(res, format!("CODE: {code}|{msg}"));
    }
}
