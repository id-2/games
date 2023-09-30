use chirp_worker::prelude::*;
use proto::backend::{self, pkg::*};
use std::time::Duration;

#[worker_test]
async fn basic(ctx: TestCtx) {
	let alloc = Uuid::new_v4().to_string();
	let task = Uuid::new_v4().to_string();
	let now = util::timestamp::now();
	let messages = vec![b"Message 1", b"Message 2", b"Message 3", b"Message 4"];
	msg!([ctx] nomad_log::msg::entries(&alloc, &task, "stdout") {
		alloc: alloc.clone(),
		task: task.clone(),
		stream_type: backend::nomad_log::StreamType::StdOut as i32,
		entries: messages
			.iter()
			.enumerate()
			.map(|(i, msg)| backend::nomad_log::LogEntry {
				ts: now + i as i64 * 1000,
				idx: 0,
				message: msg.to_vec(),
			})
			.collect(),
	})
	.await
	.unwrap();

	tokio::time::sleep(Duration::from_secs(5)).await;

	// Export logs
	let request_id = Uuid::new_v4();
	let res = msg!([ctx] nomad_log::msg::export(request_id) -> nomad_log::msg::export_complete {
		request_id: Some(request_id.into()),
		alloc: alloc.clone(),
		task: task.clone(),
		stream_type: backend::nomad_log::StreamType::StdOut as i32,
	})
	.await
	.unwrap();

	// Fetch logs
	let s3_client = s3_util::Client::from_env("bucket-nomad-log-export").await?;
	let get_object = s3_client
		.get_object()
		.bucket(s3_client.bucket())
		.key(format!("{upload_id}/stdout.txt"))
		.send()
		.await?;
	let actual_buf = get_object.body().unwrap().collect().await.unwrap().to_vec();

	// Compare buffer
	let expected_buf = messages
		.iter()
		.flat_map(|x| x.iter().cloned().chain([b'\n']))
		.collect::<Vec<u8>>();
	assert_eq!(
		expected_buf,
		&actual_buf[..],
		"mismatching logs:\n\nexpected:\n{expected}\n\nactual:\n{actual}",
		expected = String::from_utf8_lossy(&expected_buf),
		actual = String::from_utf8_lossy(&actual_buf)
	);
}
