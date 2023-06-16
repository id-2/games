use proto::backend::pkg::*;
use rivet_operation::prelude::*;
use serde_json::json;

#[operation(name = "captcha-request")]
async fn handle(
	ctx: OperationContext<captcha::request::Request>,
) -> GlobalResult<captcha::request::Response> {
	// TODO: This service is pretty slow, optimize by using counters, Redis, and/or ClickHouse

	let crdb = ctx.crdb("db-captcha").await?;

	let captcha_config = internal_unwrap!(ctx.captcha_config);
	let user_id = ctx.user_id.as_ref().map(common::Uuid::as_uuid);
	let namespace_id = ctx.namespace_id.as_ref().map(common::Uuid::as_uuid);

	let topic_value = serde_json::to_value(&ctx.topic)?;
	let topic_str = util_captcha::serialize_topic_str(&ctx.topic)?;

	// Fetch last successful query
	let success_res = sqlx::query_as::<_, (i64,)>(indoc!(
		"
		SELECT complete_ts
		FROM captcha_verifications
		WHERE topic_str = $1 AND remote_address = $2 AND success = true
		ORDER BY complete_ts DESC
		LIMIT 1
		FOR SHARE SKIP LOCKED
		"
	))
	.bind(&topic_str)
	.bind(&ctx.remote_address)
	.fetch_optional(&crdb)
	.await?;

	tracing::info!(?success_res, "fetched captcha verifications");
	let needs_verification = if let Some((success_complete_ts,)) = success_res {
		let duration_since_verification = ctx.ts() - success_complete_ts;
		if duration_since_verification >= captcha_config.verification_ttl {
			tracing::info!(?duration_since_verification, verification_ttl = ?captcha_config.verification_ttl, "been too long since last verification");
			true
		} else {
			// Fetch requests since the last success. Include limit since we
			// don't need to get an exact count beyond the max requests.
			// Otherwise, abusers could spam requests to make this query slower
			// by O(n) with every request.
			let (req_count,) = sqlx::query_as::<_, (i64,)>(indoc!(
				"
				SELECT count(*)
				FROM captcha_requests
				WHERE topic_str = $1 AND remote_address = $2 AND create_ts > $3
				LIMIT $4
				"
			))
			.bind(&topic_str)
			.bind(&ctx.remote_address)
			.bind(success_complete_ts)
			.bind(captcha_config.requests_before_reverify as i64)
			.fetch_one(&crdb)
			.await?;
			tracing::info!(?req_count, ts = ?ctx.ts(), "fetched request count");

			// Check if reached max request count since a verification
			if req_count >= captcha_config.requests_before_reverify as i64 {
				tracing::info!("max requests since verification");
				true
			} else {
				tracing::info!("no captcha required");
				false
			}
		}
	} else {
		// Never had successful captcha
		tracing::info!("never had successful captcha");
		true
	};

	// Insert a request
	sqlx::query(indoc!(
		"
		INSERT INTO captcha_requests (request_id, topic, topic_str, remote_address, create_ts, expire_ts, user_id, namespace_id)
		VALUES ($1, $2, $3, $4, $5, to_timestamp($6::float / 1000), $7, $8)
		"
	))
	.bind(Uuid::new_v4())
	.bind(&topic_value)
	.bind(&topic_str)
	.bind(&ctx.remote_address)
	.bind(ctx.ts())
	.bind(ctx.ts() + captcha_config.verification_ttl)
	.bind(user_id)
	.bind(namespace_id)
	.execute(&crdb)
	.await?;

	if needs_verification {
		msg!([ctx] analytics::msg::event_create() {
			events: vec![
				analytics::msg::event_create::Event {
					name: "captcha.needs_verification".into(),
					properties_json: Some(serde_json::to_string(&json!({
						"user_id": user_id,
						"namespace_id": namespace_id,
						"topic": ctx.topic,
						"requests_before_reverify": captcha_config.requests_before_reverify,
						"verification_ttl": captcha_config.verification_ttl,
						"has_hcaptcha": captcha_config.hcaptcha.is_some(),
						"has_turnstile": captcha_config.turnstile.is_some(),
					}))?),
					..Default::default()
				}
			],
		})
		.await?;
	}

	Ok(captcha::request::Response { needs_verification })
}
