use juniper::{EmptyMutation, RootNode, EmptySubscription};

struct Member {
    id: i32,
    name: String
}


#[juniper::graphql_object]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn members() -> Vec<Member>{
        vec![
            Member{
                id: 1,
                name: "Link".to_owned(),
            },
            Member {
                id: 2,
                name: "Edwards".to_owned(),
            }
        ]
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}