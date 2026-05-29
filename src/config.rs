use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::env;

    #[test]
    fn from_args_parses_flags_and_path() {
        let args = vec![
            OsString::from("squirrelt"),
            OsString::from("-a"),
            OsString::from("--sort-size"),
            OsString::from("-t"),
            OsString::from("tests"),
        ];

        let config = Config::from_args(args).unwrap();

        assert!(config.show_all);
        assert!(config.sort_size);
        assert!(config.sort_time);
        assert_eq!(config.path, std::path::PathBuf::from("tests"));
    }

    #[test]
    fn from_args_uses_current_dir_when_path_is_missing() {
        let args = vec![OsString::from("squirrelt")];
        let config = Config::from_args(args).unwrap();
        assert_eq!(config.path, env::current_dir().unwrap());
    }

    #[test]
    fn from_args_returns_error_for_unknown_option() {
        let args = vec![OsString::from("squirrelt"), OsString::from("--bad")];
        let err = Config::from_args(args).unwrap_err();
        assert_eq!(err, "unknown option: --bad");
    }
}
