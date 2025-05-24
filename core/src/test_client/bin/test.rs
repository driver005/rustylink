use sdk::prelude::*;

#[tokio::main]
async fn main() {
	let mutation = test_client::test::Mutation {
		client: GraphQLClient::new("http://localhost:8000/graphql"),
	};

	let data = mutation
		.simple_create_one(test_client::test::SimpleInsertInput {
			id: "11e8b1fc-44c8-402f-b185-1ef3fc6a22e1".to_owned(),
			task_model_id: None,
		})
		.await
		.unwrap();

	println!("{:?}", data);
}
