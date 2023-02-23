mod languages;

use inquire::{error::InquireResult, Select, Text};
use languages::{c::c, cpp::cpp, py::python, Language};

fn main() -> InquireResult<()> {
    let lang = Select::new("Lanugage", Language::LANGUAGES.to_vec()).prompt()?;
    let name = Text::new("Name").with_default("main").prompt()?;

    match lang {
        Language::Py => python(name)?,
        Language::C => c(name)?,
        Language::Cpp => cpp(name)?,
    }
    Ok(())
}
