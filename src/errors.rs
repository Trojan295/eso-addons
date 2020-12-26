use std::error::Error;

pub trait ErrorChain<T> {
    fn chain_err(self, msg: &str) -> Result<T, Box<dyn Error>>;
}

impl<T> ErrorChain<T> for Result<T, Box<dyn Error>> {
    fn chain_err(self, msg: &str) -> Result<T, Box<dyn Error>> {
        self.map_err(|e| Box::new(simple_error!("{}: {}", msg, e)) as Box<dyn Error>)
    }
}

impl<T> ErrorChain<T> for Result<T, std::io::Error> {
    fn chain_err(self, msg: &str) -> Result<T, Box<dyn Error>> {
        self.map_err(|e| Box::new(simple_error!("{}: {}", msg, e)) as Box<dyn Error>)
    }
}
