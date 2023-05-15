use juniper::{EmptySubscription, RootNode};

use crate::{
    controllers::basic_auth::{register_controller, RegisterInput, RegisterOutput},
    graphql::types::LikeServiceQueryReturn,
};

use super::{context::ContextUtil, types::CustomError};

pub struct QueryRoot;

#[juniper::graphql_object(
    Context = ContextUtil
)]
impl QueryRoot {
    async fn api_version(&self) -> &'static str {
        "1.0.0"
    }
    #[graphql(name = "_service")]
    fn _service(&self, context: &ContextUtil) -> LikeServiceQueryReturn {
        return LikeServiceQueryReturn {
            sdl: context.sdl.clone(),
        };
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(
    Context = ContextUtil
)]
impl MutationRoot {
    async fn register<'ctx>(&self, context: &ContextUtil, input: RegisterInput) -> RegisterOutput {
        register_controller(input)
    }
}

pub type GraphqlSchema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<ContextUtil>>;

pub fn schema() -> GraphqlSchema {
    GraphqlSchema::new(QueryRoot, MutationRoot, EmptySubscription::default())
}
