mod versions;
pub use versions::v0::SQUIDv0;

pub trait SQUID {
    fn generate(&mut self) -> String;
}

impl SQUID for SQUIDv0 {
    fn generate(&mut self) -> String {
        Self::generate(self)
    }
}
