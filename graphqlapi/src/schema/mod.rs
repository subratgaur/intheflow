use juniper::{EmptyMutation, RootNode};

pub struct Query;

#[juniper::object]
impl Query {
    fn hello() -> &str {
        "Hello, world!"
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}