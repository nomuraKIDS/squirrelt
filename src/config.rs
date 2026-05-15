use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct Config {
    pub show_all: bool,
    pub sort_size: bool,
    pub sort_time: bool,
    pub path: PathBuf,
}

impl Config {
    pub fn from_args<I>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = OsString>,
    {
        let mut show_all = false;
        let mut sort_size = false;
        let mut sort_time = false;
        let mut path = None;

        let mut args = args.into_iter();
        args.next();

        for arg in args {
            if let Some(flag) = arg.to_str() {
                match flag {
                    "--sort-size" => sort_size = true,
                    "-t" => sort_time = true,
                    "-a" => show_all = true,
                    _ if flag.starts_with('-') => {
                        return Err(format!("unknown option: {}", flag));
                    }
                    _ => path = Some(PathBuf::from(flag)),
                }
            } else {
                path = Some(PathBuf::from(arg));
            }
        }

        let path = path.unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        Ok(Config {
            show_all,
            sort_size,
            sort_time,
            path,
        })
    }
}
