use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The .json input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Optional js output directory
    #[structopt(long = "js")]
    pub dir_js: Option<PathBuf>,
    /// Optional css output directory
    #[structopt(long = "css")]
    pub dir_css: Option<PathBuf>,
    /// Optional scss output directory
    #[structopt(long = "scss")]
    pub dir_scss: Option<PathBuf>,
}

pub trait Validator {
    fn is_valid(self) -> Self;
}

impl Validator for Cli {
    fn is_valid(self) -> Self {
        if self.dir_css.is_none() && self.dir_scss.is_none() && self.dir_js.is_none() {
            panic!("The program requires that one of the optional outputs have been declared!")
        } else {
            self
        }
    }
}