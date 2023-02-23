use std::{fs::File, io::Write};

use inquire::{error::InquireResult, InquireError};

const PYTHON_FILE: &[u8; 76] =
    b"#!/usr/bin/env python\n\ndef main():\n\tpass\n\nif __name__ == \"__main__\":\n\tmain()";

pub fn python(name: String) -> InquireResult<()> {
    let mut file = File::create(format!("{name}.py")).map_err(InquireError::IO)?;
    file.write_all(PYTHON_FILE).map_err(InquireError::IO)?;
    Ok(())
}
