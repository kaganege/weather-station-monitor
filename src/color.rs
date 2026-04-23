use core::fmt;

#[macro_export]
macro_rules! colored {
    ( $s:expr ) => {
        $crate::color::Colored::new($s)
    };

    ( $s:expr, $fg:ident ) => {
        $crate::color::Colored::new($s).with_fg($crate::color::Color::$fg)
    };

    ( $s:expr, on $bg:ident ) => {
        $crate::color::Colored::new($s).with_bg($crate::color::Color::$bg)
    };

    ( $s:expr, $fg:ident, on $bg:ident ) => {
        $crate::color::Colored::new($s)
            .with_fg($crate::color::Color::$fg)
            .with_bg($crate::color::Color::$bg)
    };
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Ansi(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Foreground(pub Color);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Background(pub Color);

impl From<Color> for Foreground {
    fn from(color: Color) -> Self {
        Foreground(color)
    }
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background(color)
    }
}

impl fmt::Display for Foreground {
    #[cfg(feature = "colorize")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self.0 {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
            Color::Ansi(code) => return write!(f, "38;5;{code}"),
        };

        f.write_str(code)
    }

    #[cfg(not(feature = "colorize"))]
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl fmt::Display for Background {
    #[cfg(feature = "colorize")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self.0 {
            Color::Black => "40",
            Color::Red => "41",
            Color::Green => "42",
            Color::Yellow => "43",
            Color::Blue => "44",
            Color::Magenta => "45",
            Color::Cyan => "46",
            Color::White => "47",
            Color::BrightBlack => "100",
            Color::BrightRed => "101",
            Color::BrightGreen => "102",
            Color::BrightYellow => "103",
            Color::BrightBlue => "104",
            Color::BrightMagenta => "105",
            Color::BrightCyan => "106",
            Color::BrightWhite => "107",
            Color::Ansi(code) => return write!(f, "48;5;{code}"),
        };

        f.write_str(code)
    }

    #[cfg(not(feature = "colorize"))]
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[cfg(feature = "colorize")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Colored<T>
where
    T: fmt::Display,
{
    pub input: T,
    pub fgcolor: Option<Foreground>,
    pub bgcolor: Option<Background>,
}

#[cfg(not(feature = "colorize"))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Colored<T>
where
    T: fmt::Display,
{
    pub input: T,
}

impl<T> Colored<T>
where
    T: fmt::Display,
{
    #[allow(dead_code)]
    pub const fn is_plain(&self) -> bool {
        #[cfg(feature = "colorize")]
        {
            self.fgcolor.is_none() && self.bgcolor.is_none()
        }

        #[cfg(not(feature = "colorize"))]
        {
            true
        }
    }

    #[allow(dead_code)]
    pub const fn new(input: T) -> Self {
        Self {
            input,
            #[cfg(feature = "colorize")]
            fgcolor: None,
            #[cfg(feature = "colorize")]
            bgcolor: None,
        }
    }

    #[cfg(feature = "colorize")]
    #[allow(dead_code)]
    #[inline]
    pub const fn with_fg(mut self, color: Color) -> Self {
        self.fgcolor = Some(Foreground(color));
        self
    }

    #[cfg(not(feature = "colorize"))]
    #[allow(dead_code)]
    #[inline]
    pub const fn with_fg(self, _color: Color) -> Self {
        self
    }

    #[cfg(feature = "colorize")]
    #[allow(dead_code)]
    #[inline]
    pub const fn with_bg(mut self, color: Color) -> Self {
        self.bgcolor = Some(Background(color));
        self
    }

    #[cfg(not(feature = "colorize"))]
    #[allow(dead_code)]
    #[inline]
    pub const fn with_bg(self, _color: Color) -> Self {
        self
    }
}

#[allow(dead_code)]
pub(crate) trait Colorize<T>: Sized
where
    T: fmt::Display,
{
    fn colored(self) -> Colored<Self>
    where
        Self: fmt::Display,
    {
        Colored::new(self)
    }

    // Font Colors
    fn black(self) -> Colored<T> {
        self.color(Color::Black)
    }
    fn red(self) -> Colored<T> {
        self.color(Color::Red)
    }
    fn green(self) -> Colored<T> {
        self.color(Color::Green)
    }
    fn yellow(self) -> Colored<T> {
        self.color(Color::Yellow)
    }
    fn blue(self) -> Colored<T> {
        self.color(Color::Blue)
    }
    fn magenta(self) -> Colored<T> {
        self.color(Color::Magenta)
    }
    fn purple(self) -> Colored<T> {
        self.color(Color::Magenta)
    }
    fn cyan(self) -> Colored<T> {
        self.color(Color::Cyan)
    }
    fn white(self) -> Colored<T> {
        self.color(Color::White)
    }
    fn bright_black(self) -> Colored<T> {
        self.color(Color::BrightBlack)
    }
    fn bright_red(self) -> Colored<T> {
        self.color(Color::BrightRed)
    }
    fn bright_green(self) -> Colored<T> {
        self.color(Color::BrightGreen)
    }
    fn bright_yellow(self) -> Colored<T> {
        self.color(Color::BrightYellow)
    }
    fn bright_blue(self) -> Colored<T> {
        self.color(Color::BrightBlue)
    }
    fn bright_magenta(self) -> Colored<T> {
        self.color(Color::BrightMagenta)
    }
    fn bright_purple(self) -> Colored<T> {
        self.color(Color::BrightMagenta)
    }
    fn bright_cyan(self) -> Colored<T> {
        self.color(Color::BrightCyan)
    }
    fn bright_white(self) -> Colored<T> {
        self.color(Color::BrightWhite)
    }
    fn ansi_color<C: Into<u8>>(self, color: C) -> Colored<T> {
        self.color(Color::Ansi(color.into()))
    }
    fn color<C: Into<Foreground>>(self, color: C) -> Colored<T>;

    // Background Colors
    fn on_black(self) -> Colored<T> {
        self.on_color(Color::Black)
    }
    fn on_red(self) -> Colored<T> {
        self.on_color(Color::Red)
    }
    fn on_green(self) -> Colored<T> {
        self.on_color(Color::Green)
    }
    fn on_yellow(self) -> Colored<T> {
        self.on_color(Color::Yellow)
    }
    fn on_blue(self) -> Colored<T> {
        self.on_color(Color::Blue)
    }
    fn on_magenta(self) -> Colored<T> {
        self.on_color(Color::Magenta)
    }
    fn on_purple(self) -> Colored<T> {
        self.on_color(Color::Magenta)
    }
    fn on_cyan(self) -> Colored<T> {
        self.on_color(Color::Cyan)
    }
    fn on_white(self) -> Colored<T> {
        self.on_color(Color::White)
    }
    fn on_bright_black(self) -> Colored<T> {
        self.on_color(Color::BrightBlack)
    }
    fn on_bright_red(self) -> Colored<T> {
        self.on_color(Color::BrightRed)
    }
    fn on_bright_green(self) -> Colored<T> {
        self.on_color(Color::BrightGreen)
    }
    fn on_bright_yellow(self) -> Colored<T> {
        self.on_color(Color::BrightYellow)
    }
    fn on_bright_blue(self) -> Colored<T> {
        self.on_color(Color::BrightBlue)
    }
    fn on_bright_magenta(self) -> Colored<T> {
        self.on_color(Color::BrightMagenta)
    }
    fn on_bright_purple(self) -> Colored<T> {
        self.on_color(Color::BrightMagenta)
    }
    fn on_bright_cyan(self) -> Colored<T> {
        self.on_color(Color::BrightCyan)
    }
    fn on_bright_white(self) -> Colored<T> {
        self.on_color(Color::BrightWhite)
    }
    fn on_ansi_color<C: Into<u8>>(self, color: C) -> Colored<T> {
        self.on_color(Color::Ansi(color.into()))
    }
    fn on_color<C: Into<Background>>(self, color: C) -> Colored<T>;
}

#[cfg(feature = "colorize")]
impl<T> Colorize<T> for Colored<T>
where
    T: fmt::Display,
{
    fn color<S: Into<Foreground>>(mut self, color: S) -> Colored<T> {
        self.fgcolor = Some(color.into());
        self
    }

    fn on_color<S: Into<Background>>(mut self, color: S) -> Colored<T> {
        self.bgcolor = Some(color.into());
        self
    }
}

#[cfg(not(feature = "colorize"))]
impl<T> Colorize<T> for Colored<T>
where
    T: fmt::Display,
{
    #[inline]
    fn color<S: Into<Foreground>>(self, _color: S) -> Colored<T> {
        self
    }

    #[inline]
    fn on_color<S: Into<Background>>(self, _color: S) -> Colored<T> {
        self
    }
}

#[cfg(feature = "colorize")]
impl<T> Colorize<T> for T
where
    T: fmt::Display,
{
    fn color<S: Into<Foreground>>(self, color: S) -> Colored<T> {
        Colored {
            input: self,
            fgcolor: Some(color.into()),
            bgcolor: None,
        }
    }

    fn on_color<S: Into<Background>>(self, color: S) -> Colored<T> {
        Colored {
            input: self,
            fgcolor: None,
            bgcolor: Some(color.into()),
        }
    }
}

#[cfg(not(feature = "colorize"))]
impl<T> Colorize<T> for T
where
    T: fmt::Display,
{
    #[inline]
    fn color<S: Into<Foreground>>(self, _color: S) -> Colored<T> {
        Colored { input: self }
    }

    #[inline]
    fn on_color<S: Into<Background>>(self, _color: S) -> Colored<T> {
        Colored { input: self }
    }
}

impl<T> fmt::Display for Colored<T>
where
    T: fmt::Display,
{
    #[cfg(feature = "colorize")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Write;

        if self.is_plain() {
            return <T as fmt::Display>::fmt(&self.input, f);
        }

        let mut has_written = false;

        f.write_str("\x1B[")?;

        if let Some(bgcolor) = &self.bgcolor {
            if has_written {
                f.write_char(';')?;
            }

            write!(f, "{bgcolor}")?;
            has_written = true;
        }

        if let Some(fgcolor) = &self.fgcolor {
            if has_written {
                f.write_char(';')?;
            }

            write!(f, "{fgcolor}")?;
        }

        write!(f, "m{}\x1B[0m", self.input)?;

        Ok(())
    }

    #[cfg(not(feature = "colorize"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <T as fmt::Display>::fmt(&self.input, f)
    }
}
