use chirp_worker::prelude::*;

use std::collections::HashSet;

struct TestNamespace {
	display_name: String,
	name_id: String,
}

impl TestNamespace {
	fn generate(count: usize) -> Vec<TestNamespace> {
		std::iter::repeat_with(|| TestNamespace {
			display_name: util::faker::display_name(),
			name_id: util::faker::ident(),
		})
		.take(count)
		.collect()
	}
}

#[worker_test]
async fn empty(ctx: TestCtx) {
	let game_namespaces = TestNamespace::generate(8);

	let game_create_res = op!([ctx] faker_game {
		skip_namespaces_and_versions: true,
		..Default::default()
	})
	.await
	.unwrap();

	let version_create_res = op!([ctx] game_version_create {
		game_id: game_create_res.game_id,
		display_name: util::faker::ident(),
	})
	.await
	.unwrap();

	let mut namespace_ids = HashSet::<Uuid>::new();
	for namespace in &game_namespaces {
		let create_res = op!([ctx] game_namespace_create {
			game_id: game_create_res.game_id,
			display_name: namespace.display_name.clone(),
			version_id: version_create_res.version_id,
			name_id: namespace.name_id.clone(),
		})
		.await
		.unwrap();
		namespace_ids.insert(create_res.namespace_id.unwrap().as_uuid());
	}

	let res = op!([ctx] game_resolve_namespace_id {
		namespace_ids: namespace_ids
			.clone()
			.into_iter()
			.map(common::Uuid::from)
			.collect::<Vec<_>>(),
	})
	.await
	.unwrap();
	assert_eq!(1, res.games.len());
	let game_a = &res.games[0];
	assert_eq!(namespace_ids.len(), game_a.namespace_ids.len());
}
