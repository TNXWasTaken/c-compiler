use std::{fs, io::Write};

fn compile(contents: String)-> Result<(), Box<dyn std::error::Error>> {
    let mut out = String::new();
    let mut out_file = fs::File::create("/workspaces/c-compiler/main.o")?;
    for line in contents.lines() {
        out += line.split("//").nth(0).unwrap_or(line);
        out += "\n";
    }
    out_file.write_all(&out.into_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents: String = fs::read_to_string("/workspaces/c-compiler/main.c").unwrap();
    return compile(contents);
}
