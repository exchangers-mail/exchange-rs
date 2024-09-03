mod types;

pub mod prelude {
    pub use super::types::*;
}

pub trait Mailbox {
    fn mailbox_name() -> Option<String> {
        None
    }
}
