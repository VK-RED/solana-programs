use anchor_lang::prelude::*;

#[error_code]
pub enum TokenErrors{
    #[msg("Name is Too Large")]
    NameTooLarge,
    #[msg("Symbol is Too Large")]
    SymbolTooLarge,
    #[msg("ATA not derived from the Mint Account")]
    InvalidMintAccount,
    #[msg("You dont have permission for this action")]
    PermissionDenied,
    #[msg("Cannot mint tokens as the mintAccess is revoked")]
    MintAccessRevoked,
    #[msg("Tokens too low to transfer")]
    TokensTooLow,
    #[msg("Insufficient Funds")]
    InsufficientFunds
}