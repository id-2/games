// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `FindLobby`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`find_lobby`](crate::client::Client::find_lobby).
///
/// See [`crate::client::fluent_builders::FindLobby`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct FindLobby {
	_private: (),
}
impl FindLobby {
	/// Creates a new builder-style object to manufacture [`FindLobbyInput`](crate::input::FindLobbyInput)
	pub fn builder() -> crate::input::find_lobby_input::Builder {
		crate::input::find_lobby_input::Builder::default()
	}
	/// Creates a new `FindLobby` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for FindLobby {
	type Output = std::result::Result<crate::output::FindLobbyOutput, crate::error::FindLobbyError>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_find_lobby_error(response)
		} else {
			crate::operation_deser::parse_find_lobby_response(response)
		}
	}
}

/// Operation shape for `GetGameStatistics`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_game_statistics`](crate::client::Client::get_game_statistics).
///
/// See [`crate::client::fluent_builders::GetGameStatistics`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetGameStatistics {
	_private: (),
}
impl GetGameStatistics {
	/// Creates a new builder-style object to manufacture [`GetGameStatisticsInput`](crate::input::GetGameStatisticsInput)
	pub fn builder() -> crate::input::get_game_statistics_input::Builder {
		crate::input::get_game_statistics_input::Builder::default()
	}
	/// Creates a new `GetGameStatistics` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for GetGameStatistics {
	type Output = std::result::Result<
		crate::output::GetGameStatisticsOutput,
		crate::error::GetGameStatisticsError,
	>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_get_game_statistics_error(response)
		} else {
			crate::operation_deser::parse_get_game_statistics_response(response)
		}
	}
}

/// Operation shape for `JoinLobby`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`join_lobby`](crate::client::Client::join_lobby).
///
/// See [`crate::client::fluent_builders::JoinLobby`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct JoinLobby {
	_private: (),
}
impl JoinLobby {
	/// Creates a new builder-style object to manufacture [`JoinLobbyInput`](crate::input::JoinLobbyInput)
	pub fn builder() -> crate::input::join_lobby_input::Builder {
		crate::input::join_lobby_input::Builder::default()
	}
	/// Creates a new `JoinLobby` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for JoinLobby {
	type Output = std::result::Result<crate::output::JoinLobbyOutput, crate::error::JoinLobbyError>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_join_lobby_error(response)
		} else {
			crate::operation_deser::parse_join_lobby_response(response)
		}
	}
}

/// Operation shape for `ListLobbies`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_lobbies`](crate::client::Client::list_lobbies).
///
/// See [`crate::client::fluent_builders::ListLobbies`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLobbies {
	_private: (),
}
impl ListLobbies {
	/// Creates a new builder-style object to manufacture [`ListLobbiesInput`](crate::input::ListLobbiesInput)
	pub fn builder() -> crate::input::list_lobbies_input::Builder {
		crate::input::list_lobbies_input::Builder::default()
	}
	/// Creates a new `ListLobbies` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for ListLobbies {
	type Output =
		std::result::Result<crate::output::ListLobbiesOutput, crate::error::ListLobbiesError>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_list_lobbies_error(response)
		} else {
			crate::operation_deser::parse_list_lobbies_response(response)
		}
	}
}

/// Operation shape for `ListRegions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_regions`](crate::client::Client::list_regions).
///
/// See [`crate::client::fluent_builders::ListRegions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRegions {
	_private: (),
}
impl ListRegions {
	/// Creates a new builder-style object to manufacture [`ListRegionsInput`](crate::input::ListRegionsInput)
	pub fn builder() -> crate::input::list_regions_input::Builder {
		crate::input::list_regions_input::Builder::default()
	}
	/// Creates a new `ListRegions` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for ListRegions {
	type Output =
		std::result::Result<crate::output::ListRegionsOutput, crate::error::ListRegionsError>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_list_regions_error(response)
		} else {
			crate::operation_deser::parse_list_regions_response(response)
		}
	}
}

/// Operation shape for `LobbyReady`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`lobby_ready`](crate::client::Client::lobby_ready).
///
/// See [`crate::client::fluent_builders::LobbyReady`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct LobbyReady {
	_private: (),
}
impl LobbyReady {
	/// Creates a new builder-style object to manufacture [`LobbyReadyInput`](crate::input::LobbyReadyInput)
	pub fn builder() -> crate::input::lobby_ready_input::Builder {
		crate::input::lobby_ready_input::Builder::default()
	}
	/// Creates a new `LobbyReady` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for LobbyReady {
	type Output =
		std::result::Result<crate::output::LobbyReadyOutput, crate::error::LobbyReadyError>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_lobby_ready_error(response)
		} else {
			crate::operation_deser::parse_lobby_ready_response(response)
		}
	}
}

/// Operation shape for `PlayerConnected`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`player_connected`](crate::client::Client::player_connected).
///
/// See [`crate::client::fluent_builders::PlayerConnected`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PlayerConnected {
	_private: (),
}
impl PlayerConnected {
	/// Creates a new builder-style object to manufacture [`PlayerConnectedInput`](crate::input::PlayerConnectedInput)
	pub fn builder() -> crate::input::player_connected_input::Builder {
		crate::input::player_connected_input::Builder::default()
	}
	/// Creates a new `PlayerConnected` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for PlayerConnected {
	type Output = std::result::Result<
		crate::output::PlayerConnectedOutput,
		crate::error::PlayerConnectedError,
	>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_player_connected_error(response)
		} else {
			crate::operation_deser::parse_player_connected_response(response)
		}
	}
}

/// Operation shape for `PlayerDisconnected`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`player_disconnected`](crate::client::Client::player_disconnected).
///
/// See [`crate::client::fluent_builders::PlayerDisconnected`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PlayerDisconnected {
	_private: (),
}
impl PlayerDisconnected {
	/// Creates a new builder-style object to manufacture [`PlayerDisconnectedInput`](crate::input::PlayerDisconnectedInput)
	pub fn builder() -> crate::input::player_disconnected_input::Builder {
		crate::input::player_disconnected_input::Builder::default()
	}
	/// Creates a new `PlayerDisconnected` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for PlayerDisconnected {
	type Output = std::result::Result<
		crate::output::PlayerDisconnectedOutput,
		crate::error::PlayerDisconnectedError,
	>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_player_disconnected_error(response)
		} else {
			crate::operation_deser::parse_player_disconnected_response(response)
		}
	}
}

/// Operation shape for `SetLobbyClosed`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`set_lobby_closed`](crate::client::Client::set_lobby_closed).
///
/// See [`crate::client::fluent_builders::SetLobbyClosed`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SetLobbyClosed {
	_private: (),
}
impl SetLobbyClosed {
	/// Creates a new builder-style object to manufacture [`SetLobbyClosedInput`](crate::input::SetLobbyClosedInput)
	pub fn builder() -> crate::input::set_lobby_closed_input::Builder {
		crate::input::set_lobby_closed_input::Builder::default()
	}
	/// Creates a new `SetLobbyClosed` operation.
	pub fn new() -> Self {
		Self { _private: () }
	}
}
impl aws_smithy_http::response::ParseStrictResponse for SetLobbyClosed {
	type Output =
		std::result::Result<crate::output::SetLobbyClosedOutput, crate::error::SetLobbyClosedError>;
	fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
		if !response.status().is_success() && response.status().as_u16() != 200 {
			crate::operation_deser::parse_set_lobby_closed_error(response)
		} else {
			crate::operation_deser::parse_set_lobby_closed_response(response)
		}
	}
}
