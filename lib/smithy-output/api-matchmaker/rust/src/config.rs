// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Service config.
///
pub struct Config {
	pub(crate) auth: Option<String>,
	pub(crate) uri: String,
}
impl std::fmt::Debug for Config {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut config = f.debug_struct("Config");
		config.finish()
	}
}
impl Config {
	/// Constructs a config builder.
	pub fn builder() -> Builder {
		Builder::default()
	}
}
/// Builder for creating a `Config`.
#[derive(Default)]
pub struct Builder {
	pub(crate) auth: Option<String>,
	pub(crate) uri: Option<String>,
}
impl Builder {
	/// Constructs a config builder.
	pub fn new() -> Self {
		Self::default()
	}
	/// Sets the bearer token to be used with the Smithy client.
	pub fn set_bearer_token(mut self, bearer_token: impl std::fmt::Display) -> Self {
		self.auth = Some(format!("Bearer {}", bearer_token));
		self
	}
	/// Sets the base URI to be used with the Smithy client.
	pub fn set_uri(mut self, uri: impl ToString) -> Self {
		self.uri = Some(uri.to_string());
		self
	}
	/// Builds a [`Config`].
	pub fn build(self) -> Config {
		Config {
			auth: self
				.auth
				.or_else(|| std::env::var("RIVET_LOBBY_TOKEN").ok())
				.or_else(|| std::env::var("RIVET_CLIENT_TOKEN").ok()),
			uri: self
				.uri
				.or_else(|| std::env::var("RIVET_MATCHMAKER_API_URL").ok())
				.unwrap_or_else(|| "https://matchmaker.api.rivet.gg/v1".to_string()),
		}
	}
	/// Builds a default [`Client`] from the given config.
	pub fn build_client(self) -> crate::client::ClientWrapper {
		let raw_client = aws_smithy_client::Builder::dyn_https()
			.middleware(tower::layer::util::Identity::new())
			.sleep_impl(None)
			.build();
		crate::client::ClientWrapper {
			client: crate::Client::with_config(raw_client, self.build()),
		}
	}
}
