use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser as CmdlineParser;

use crate::tokenizer::Source;

#[derive(CmdlineParser, Debug)]
#[clap(author="kxxt", version, about="Jack compiler for nand2tetris course", long_about = None)]
struct Args {
    /// input file or folder
    #[clap(value_parser)]
    input: String,

    /// output file or folder
    #[clap(short, long, value_parser)]
    output: Option<String>,
}

impl Args {
    fn get_inputs(&self) -> Result<Vec<Source>, io::Error> {
        let input = Path::new(&self.input);
        let files = if input.is_dir() {
            fs::read_dir(input)?
                .filter_map(|s| {
                    s.ok().and_then(|entry| {
                        let path = entry.path();
                        if path.is_file() && path.extension().is_some_and(|ext| *ext == "jack") {
                            Some(path)
                        } else {
                            None
                        }
                    })
                })
                .collect()
        } else {
            vec![PathBuf::from(input)]
        };
        if files.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "No source code found in directory {}!",
                    input.to_string_lossy()
                ),
            ));
        }
        Ok(files
            .iter()
            .map(|file| {
                Ok(Source {
                    content: fs::read_to_string(&file)?,
                    name: Path::new(file)
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                })
            })
            .collect::<Result<Vec<Source>, io::Error>>()?)
    }
}
