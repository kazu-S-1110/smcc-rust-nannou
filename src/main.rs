use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;

const BOM: &[u8; 3] = &[0xEF, 0xBB, 0xBF]; // UTF-8

fn main() -> Result<()> {
    let output_dir = Path::new("out");
    create_dir_all(&output_dir)?;

    let mut w = BufWriter::new(File::create(output_dir.join("smcc.csv"))?);

    w.write_all(BOM)?;

    writeln!(w, "id,1")?;
    writeln!(w, "name,hoge")?;
    writeln!(w, "age,40")?;

    Ok(())
}
