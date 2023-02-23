use std::{fs::File, io::Write};

use inquire::{error::InquireResult, InquireError};

const CPP_FILE: &[u8; 45] = b"#include <iostream>\n\nint main(){\n\treturn 0;\n}";

pub fn cpp(name: String) -> InquireResult<()> {
    let mut file = File::create(format!("{name}.cpp")).map_err(InquireError::IO)?;
    file.write_all(CPP_FILE).map_err(InquireError::IO)?;
    Ok(())
}
