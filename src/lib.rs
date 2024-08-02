mod types;

pub mod prelude {
    pub use super::types::*;
}

pub trait ExchangeMailboxProvider {
    fn mailbox_name() -> Option<String> {
        None
    }
}
