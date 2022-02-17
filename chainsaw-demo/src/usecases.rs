//

//!
//!
//!

#[cfg_attr(test, mockall::automock)]
pub trait HelloUseCase: std::fmt::Debug + Send + Sync {
    fn execute(&self, name: &str) -> String;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Hello;

impl HelloUseCase for Hello {
    fn execute(&self, name: &str) -> String {
        format!("Hello {}", name)
    }
}
