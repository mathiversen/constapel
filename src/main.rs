use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

const DONT_EDIT_COPY: &str = r"DON'T EDIT THIS FILE - IT'S GENERATED";

#[derive(Debug, StructOpt)]
struct Cli {
    /// The .json input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Optional js output directory
    #[structopt(long = "js")]
    dir_js: Option<PathBuf>,
    /// Optional css output directory
    #[structopt(long = "css")]
    dir_css: Option<PathBuf>,
    /// Optional scss output directory
    #[structopt(long = "scss")]
    dir_scss: Option<PathBuf>,
}

trait ValidInput {
    fn is_valid(&self);
}

impl ValidInput for Cli {
    fn is_valid(&self) {
        if self.dir_css.is_none() && self.dir_scss.is_none() && self.dir_js.is_none() {
            panic!("The program requires that one of the optional outputs have been declared!")
        }
    }
}

fn create_scss_files(name: &str, value: &Value, path: &PathBuf) -> Result<()> {
    let mut file_css = File::create(format!("{}/_{}.scss", &path.to_string_lossy(), &name))?;
    file_css.write_all(format!("// {}\n\n", DONT_EDIT_COPY).as_bytes())?;
    for v in value.as_object().expect("Failed to parse value") {
        let x = match v.1 {
            Value::String(x) => format!("${}: {};\n", v.0, x.replace("\"", "")),
            Value::Number(x) => format!("${}: {};\n", v.0, x),
            _ => panic!("Only strings and numbers have been implemented for"),
        };
        file_css.write_all(x.as_bytes())?;
    }
    Ok(())
}

fn create_css_files(value: &Value, path: &PathBuf, is_last: bool) -> Result<()> {
    let file_path = format!("{}root.css", &path.to_string_lossy());
    let file_exist = File::open(&file_path).is_ok();
    let mut file = if file_exist {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)?
    } else {
        File::create(&file_path)?
    };
    if !file_exist {
        file.write_all(format!("/* {} */\n\n", DONT_EDIT_COPY).as_bytes())?;
        file.write_all(":root {\n".as_bytes())?;
    }
    for v in value.as_object().expect("Failed to parse value") {
        let x = match v.1 {
            Value::String(x) => format!("    --{}: {};\n", v.0, x.replace("\"", "")),
            Value::Number(x) => format!("    --{}: {};\n", v.0, x),
            _ => panic!("Only strings and numbers have been implemented for"),
        };
        file.write_all(x.as_bytes()).expect("failed to write");
    }
    if is_last {
        file.write_all("}\n".as_bytes())?;
    }
    Ok(())
}

fn create_js_files(name: &str, value: &Value, path: &PathBuf) -> Result<()> {
    let mut file_css = File::create(format!("{}/{}.js", &path.to_string_lossy(), &name))?;
    file_css.write_all(format!("// {}\n\n", DONT_EDIT_COPY).as_bytes())?;
    file_css.write_all("export default {\n".as_bytes())?;
    for v in value.as_object().expect("Failed to parse value") {
        file_css.write_all(format!("    {}: {},\n", v.0, v.1).as_bytes())?;
    }
    file_css.write_all("}\n".as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let opt = Cli::from_args();
    opt.is_valid();
    let mut file = File::open(&opt.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let c: Value = serde_json::from_str(&contents)?;
    let root = c.as_object().expect("The file provided is not a json");
    for (index, (key, value)) in root.iter().enumerate() {
        if opt.dir_scss.is_some() {
            create_scss_files(key, value, &opt.dir_scss.clone().unwrap())?;
        }
        if opt.dir_css.is_some() {
            let last_value = index == root.len() - 1;
            create_css_files(value, &opt.dir_css.clone().unwrap(), last_value)?;
        }
        if opt.dir_js.is_some() {
            create_js_files(key, value, &opt.dir_js.clone().unwrap())?;
        }
    }
    Ok(())
}
