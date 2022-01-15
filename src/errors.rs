use std::error;

#[derive(Debug)]
pub enum Error {
    CannotOpenAddonDirectory(String, Box<dyn error::Error>),
    CannotRemoveAddon(String, Box<dyn error::Error>),
    CannotLoadConfig,
    CannotDownloadAddon(String, Box<dyn error::Error>),
    Other(Box<dyn error::Error>),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::CannotOpenAddonDirectory(dir, err) => {
                f.write_str(&format!("cannot open addon directory {}: {}", dir, err))
            }
            Error::CannotRemoveAddon(name, err) => {
                f.write_str(&format!("cannot remove addon {}: {}", name, err))
            }
            Error::CannotLoadConfig => f.write_str("cannot load config"),
            Error::CannotDownloadAddon(url, err) => {
                f.write_str(&format!("cannot download addon {}: {}", url, err))
            }
            Error::Other(err) => err.fmt(f),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl error::Error for Error {}

//pub trait ErrorChain<T> {
//fn chain_err(self, msg: &str) -> std::result::Result<T, Box<dyn error::Error>>;
//}

//impl<T> ErrorChain<T> for std::result::Result<T, Box<dyn error::Error>> {
//fn chain_err(self, msg: &str) -> std::result::Result<T, Box<dyn error::Error>> {
//self.map_err(|e| Box::new(simple_error!("{}: {}", msg, e)) as Box<dyn error::Error>)
//}
//}

//impl<T> ErrorChain<T> for std::result::Result<T, std::io::Error> {
//fn chain_err(self, msg: &str) -> std::result::Result<T, Box<dyn error::Error>> {
//self.map_err(|e| Box::new(simple_error!("{}: {}", msg, e)) as Box<dyn error::Error>)
//}
//}
