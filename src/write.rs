use color_eyre::{Result, eyre::WrapErr};
use edit::{edit_file, Builder};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::PathBuf;

const TEMPLATE: &[u8;2] = b"# ";

pub fn write(
    garden_path: PathBuf, 
    title: Option<String>
) -> Result<()> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create wip file")?
        .keep()
        .wrap_err("Failed to keep tempfile")?;
    
    file.write_all(TEMPLATE)?;

    // Let the user write their stuff in their editor. Then return to cli and finish up
    edit_file(filepath)?;

    // Read user changes back from the file to a string
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut contents)?;

    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|v| v.starts_with("# "))
            .map(|maybe_line| {
                maybe_line  
                    .trim_start_matches("# ")
                    .to_string()
            })
    });

    dbg!(contents, document_title);
    todo!()
}
