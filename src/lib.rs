mod color {
    macro_rules! color {
        ($color_name:ident, $code:expr) => {
            pub const $color_name: &str = concat!("\x1B[", $code, "m");
        };
    }

    color!(reset, 0);
    color!(black, 30);
    color!(red, 31);
    color!(green, 32);
    color!(yellow, 33);
    color!(blue, 34);
    color!(magenta, 35);
    color!(cyan, 36);
    color!(white, 37);
}

macro_rules! colorize {
    ($($color_name:ident),*) => {
        pub trait ColorizeDisplay {
            $(
                fn $color_name(&self) -> String;
            )*
        }
        pub trait ColorizeDebug {
            $(
                fn $color_name(&self) -> String;
            )*
        }
    };
}

colorize!(black, red, green, yellow, blue, magenta, cyan, white);

macro_rules! impl_colorize {
    ($($color_name:ident),*) => {
        impl<T: std::fmt::Display> ColorizeDisplay for T {
            $(
                fn $color_name(&self) -> String {
                    format!("{}{}{}", color::$color_name, self, color::reset)
                }
            )*
        }
        impl<T: std::fmt::Debug> ColorizeDebug for T {
            $(
                fn $color_name(&self) -> String {
                    format!("{}{:?}{}", color::$color_name, self, color::reset)
                }
            )*
        }
    };
}

impl_colorize!(black, red, green, yellow, blue, magenta, cyan, white);