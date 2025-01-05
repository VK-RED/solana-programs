use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError{
    #[msg("You have reached the maximum limit")]
    ReachedLimit,

    #[msg("Title length has to be less than or equal to 15")]
    TitleLimit,

    #[msg("Description length has to be less than or equal to 100")]
    DescriptionLimit

}