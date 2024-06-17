use anchor_lang::Result;

pub trait Validate<'info> {
    /// Validates the account struct.
    fn validate(&self) -> Result<()>;
}
