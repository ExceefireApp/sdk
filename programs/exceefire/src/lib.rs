use anchor_lang::prelude::*;

declare_id!("Exceefire11111111111111111111111111111111");

#[program]
pub mod exceefire {
    use super::*;

    pub fn open_position(
        ctx: Context<OpenPosition>,
        size: u64,
        leverage: u8,
    ) -> Result<()> {
        let position = &mut ctx.accounts.position;
        position.owner = ctx.accounts.user.key();
        position.size = size;
        position.leverage = leverage;
        position.is_open = true;

        Ok(())
    }

    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        let position = &mut ctx.accounts.position;

        require!(position.is_open, ErrorCode::PositionClosed);
        require!(
            position.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );

        position.is_open = false;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub position: Account<'info, Position>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub position: Account<'info, Position>,
    pub user: Signer<'info>,
}

#[account]
pub struct Position {
    pub owner: Pubkey,
    pub size: u64,
    pub leverage: u8,
    pub is_open: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Position already closed")]
    PositionClosed,
    #[msg("Unauthorized action")]
    Unauthorized,
}
