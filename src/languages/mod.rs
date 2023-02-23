use std::fmt::Display;

pub mod c;
pub mod cpp;
pub mod py;

#[derive(Clone)]
pub enum Language {
    Py,
    C,
    Cpp,
}
impl Language {
    pub const LANGUAGES: [Self; 3] = [Self::Py, Self::C, Self::Cpp];
}
impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayable = match self {
            Self::Py => "Python",
            Self::C => "C",
            Self::Cpp => "C++",
        };

        write!(f, "{displayable}")
    }
}
