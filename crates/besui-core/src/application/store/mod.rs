pub mod exchange;
pub mod token;

pub struct Query {}

pub struct Mutation {}

impl Default for Mutation {
    fn default() -> Self {
        Mutation {}
    }
}
