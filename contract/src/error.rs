//! Error handling on the casper platform.
use casper_types::ApiError;

/// Errors which can be returned by the library.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// Where a smart contract consuming this library needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
///
#[repr(u16)]
#[derive(Debug)]
pub enum Error {
    /// ERC20 contract called from within an invalid context.
    PermissionDenied = 0u16,
    RequireApprove,
    /// Spender does not have enough balance.
    FinishedOrder,
    NotOrderCreator,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance,
    InsufficientBalance,
    InvalidPayToken,
    /// Operation would cause an integer overflow.
    Overflow,
    InvalidContext,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}
