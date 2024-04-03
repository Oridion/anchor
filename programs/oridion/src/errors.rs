use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Rdm oe error")]
    CometRdmOEError,
    #[msg("Rdm br error")]
    CometRdmBrError,
    #[msg("Planet cannot be deleted. Has funds")]
    PlanetDeleteHasFundsError,
    #[msg("Comet id length error")]
    CometIdLengthError,
    #[msg("From planet is not the same")]
    HopErrorFromPlanetNotCorrect,
    #[msg("To and from cannot be the same")]
    HopErrorToAndFromAreSame,
    #[msg("Stars IDs must be unique")]
    HopErrorStarsMustBeUnique,
    #[msg("Planet does not have enough lamports to cover transaction!")]
    PlanetNotEnoughFundsError,
    #[msg("Star split calculations do not add up!")]
    StarHopCalculationError,
}