use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Planet cannot be deleted. Has funds")]
    PlanetDeleteHasFundsError,
    #[msg("To and from cannot be the same")]
    HopErrorToAndFromAreSame,
    #[msg("Stars IDs must be unique")]
    HopErrorStarsMustBeUnique,
    #[msg("Planet does not have enough lamports to cover transaction!")]
    PlanetNotEnoughFundsError,
    #[msg("Star split calculations do not add up!")]
    StarHopCalculationError,
}