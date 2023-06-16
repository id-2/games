// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
	/// An error thrown when the requestee has sent an invalid or malformed request.
	BadRequestError(crate::error::BadRequestError),
	/// An error thrown when the requestee requests a resource they do not have access to.
	ForbiddenError(crate::error::ForbiddenError),
	/// An error caused by internal server problems.
	InternalError(crate::error::InternalError),
	/// An error thrown when the requestee requests a non existant resource.
	NotFoundError(crate::error::NotFoundError),
	/// An error thrown when the requestee has hit a rate limit. You are sending too many requests too quickly.
	RateLimitError(crate::error::RateLimitError),
	/// An error thrown when the requestee is not authenticated.
	UnauthorizedError(crate::error::UnauthorizedError),
	/// An unhandled error occurred.
	Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::BadRequestError(inner) => inner.fmt(f),
			Error::ForbiddenError(inner) => inner.fmt(f),
			Error::InternalError(inner) => inner.fmt(f),
			Error::NotFoundError(inner) => inner.fmt(f),
			Error::RateLimitError(inner) => inner.fmt(f),
			Error::UnauthorizedError(inner) => inner.fmt(f),
			Error::Unhandled(inner) => inner.fmt(f),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePartyError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::CreatePartyError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::CreatePartyErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::CreatePartyErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::CreatePartyErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::CreatePartyErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::CreatePartyErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::CreatePartyErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::CreatePartyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePartyInviteError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::CreatePartyInviteError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::CreatePartyInviteErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::CreatePartyInviteErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::CreatePartyInviteErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::CreatePartyInviteErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::CreatePartyInviteErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::CreatePartyInviteErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::CreatePartyInviteErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPartyFromInviteError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::GetPartyFromInviteError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::GetPartyFromInviteErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::GetPartyFromInviteErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::GetPartyFromInviteErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::GetPartyFromInviteErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::GetPartyFromInviteErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::GetPartyFromInviteErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::GetPartyFromInviteErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPartyProfileError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPartyProfileError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::GetPartyProfileErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::GetPartyProfileErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::GetPartyProfileErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::GetPartyProfileErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::GetPartyProfileErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::GetPartyProfileErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::GetPartyProfileErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPartySelfProfileError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::GetPartySelfProfileError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::GetPartySelfProfileErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::GetPartySelfProfileErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::GetPartySelfProfileErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::GetPartySelfProfileErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::GetPartySelfProfileErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::GetPartySelfProfileErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::GetPartySelfProfileErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPartySelfSummaryError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::GetPartySelfSummaryError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::GetPartySelfSummaryErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::GetPartySelfSummaryErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::GetPartySelfSummaryErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::GetPartySelfSummaryErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::GetPartySelfSummaryErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::GetPartySelfSummaryErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::GetPartySelfSummaryErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPartySummaryError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPartySummaryError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::GetPartySummaryErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::GetPartySummaryErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::GetPartySummaryErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::GetPartySummaryErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::GetPartySummaryErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::GetPartySummaryErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::GetPartySummaryErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::JoinPartyError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::JoinPartyError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::JoinPartyErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::JoinPartyErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::JoinPartyErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::JoinPartyErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::JoinPartyErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::JoinPartyErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::JoinPartyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::KickMemberError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::KickMemberError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::KickMemberErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::KickMemberErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::KickMemberErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::KickMemberErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::KickMemberErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::KickMemberErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::KickMemberErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::LeavePartyError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::LeavePartyError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::LeavePartyErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::LeavePartyErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::LeavePartyErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::LeavePartyErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::LeavePartyErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::LeavePartyErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::LeavePartyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RevokePartyInviteError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::RevokePartyInviteError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::RevokePartyInviteErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::RevokePartyInviteErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::RevokePartyInviteErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::RevokePartyInviteErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::RevokePartyInviteErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::RevokePartyInviteErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::RevokePartyInviteErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendJoinRequestError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::SendJoinRequestError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::SendJoinRequestErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::SendJoinRequestErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::SendJoinRequestErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::SendJoinRequestErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::SendJoinRequestErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::SendJoinRequestErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::SendJoinRequestErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetPartyPublicityError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::SetPartyPublicityError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::SetPartyPublicityErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::SetPartyPublicityErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::SetPartyPublicityErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::SetPartyPublicityErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::SetPartyPublicityErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::SetPartyPublicityErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::SetPartyPublicityErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TransferPartyOwnershipError, R>>
	for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::TransferPartyOwnershipError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::TransferPartyOwnershipErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::TransferPartyOwnershipErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::TransferPartyOwnershipErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::TransferPartyOwnershipErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::TransferPartyOwnershipErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::TransferPartyOwnershipErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::TransferPartyOwnershipErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::FindMatchmakerLobbyForPartyError, R>>
	for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::FindMatchmakerLobbyForPartyError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::FindMatchmakerLobbyForPartyErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::FindMatchmakerLobbyForPartyErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::FindMatchmakerLobbyForPartyErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::FindMatchmakerLobbyForPartyErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::FindMatchmakerLobbyForPartyErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::FindMatchmakerLobbyForPartyErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::FindMatchmakerLobbyForPartyErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::JoinMatchmakerLobbyForPartyError, R>>
	for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::JoinMatchmakerLobbyForPartyError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::JoinMatchmakerLobbyForPartyErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::JoinMatchmakerLobbyForPartyErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::JoinMatchmakerLobbyForPartyErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::JoinMatchmakerLobbyForPartyErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::JoinMatchmakerLobbyForPartyErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::JoinMatchmakerLobbyForPartyErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::JoinMatchmakerLobbyForPartyErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::MatchmakerSelfReadyError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::MatchmakerSelfReadyError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::MatchmakerSelfReadyErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::MatchmakerSelfReadyErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::MatchmakerSelfReadyErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::MatchmakerSelfReadyErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::MatchmakerSelfReadyErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::MatchmakerSelfReadyErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::MatchmakerSelfReadyErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetPartyToIdleError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::SetPartyToIdleError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::SetPartyToIdleErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::SetPartyToIdleErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::SetPartyToIdleErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::SetPartyToIdleErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::SetPartyToIdleErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::SetPartyToIdleErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::SetPartyToIdleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetSelfInactiveError, R>> for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(err: aws_smithy_http::result::SdkError<crate::error::SetSelfInactiveError, R>) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::SetSelfInactiveErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::SetSelfInactiveErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::SetSelfInactiveErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::SetSelfInactiveErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::SetSelfInactiveErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::SetSelfInactiveErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::SetSelfInactiveErrorKind::Unhandled(inner) => Error::Unhandled(inner),
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl std::error::Error for Error {}
