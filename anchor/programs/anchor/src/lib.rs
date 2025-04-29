pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9G5BStcx7rpUvPG6jJVbhvqhwQTGKSFy2J6ZdMEsJwnH");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
