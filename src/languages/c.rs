use std::{fs::File, io::Write};

use inquire::{error::InquireResult, InquireError};

const C_FILE: &[u8; 44] = b"#include <stdio.h>\n\nint main(){\n\treturn 0;\n}";

pub fn c(name: String) -> InquireResult<()> {
    let mut file = File::create(format!("{name}.c")).map_err(InquireError::IO)?;
    file.write_all(C_FILE).map_err(InquireError::IO)?;
    Ok(())
}
