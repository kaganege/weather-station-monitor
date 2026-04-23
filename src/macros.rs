#[macro_export]
macro_rules! define_peripherals {
    {
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$inner:meta])*
                $peripheral:ident
            ),+
            $(,)?
        }
    } => {
        #[allow(non_snake_case, missing_docs)]
        $(#[$meta])*
        pub struct $name<'a> {
            $(
                $(#[$inner:meta])*
                pub $peripheral: embassy_rp::Peri<'a, embassy_rp::peripherals::$peripheral>,
            )+
        }
    };
}

#[macro_export]
macro_rules! borrow_peripherals {
    ($peripherals:expr, $name:ident {
        $($peripheral:ident),*
    }) => {
        $name {
            $(
                $peripheral: $peripherals.$peripheral,
            )*
        }
    };
}

#[macro_export]
macro_rules! bounded_str {
    // Inclusively below and exclusively above (min..max)
    ($str:expr, $min:literal..$max:literal) => {{
        const _: () = assert!($str.len() >= $min && $str.len() < $max);
        $str
    }};

    // Inclusively below (min..)
    ($str:expr, $min:literal..) => {{
        const _: () = assert!($str.len() >= $min);
        $str
    }};

    // Inclusively below and above (min..=max)
    ($str:expr, $min:literal..=$max:literal) => {{
        const _: () = assert!($str.len() >= $min && $str.len() <= $max);
        $str
    }};

    // Exclusively above (..max)
    ($str:expr, ..$max:literal) => {{
        const _: () = assert!($str.len() < $max);
        $str
    }};

    // Inclusively above (..=max)
    ($str:expr, ..=$max:literal) => {{
        const _: () = assert!($str.len() <= $max);
        $str
    }};
}
