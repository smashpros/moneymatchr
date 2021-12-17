use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Game State
/// 0 - Waiting player to join
/// 1 - Player joined
/// 2 - Game started
/// 3 - Game finished and waiting for election
/// 4 - Game finished and winner elected

const SEED: &[u8; 11] = b"moneymatchr";

#[program]
pub mod smashpros_moneymatchr {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        bump: u8,
        amount: u64,
        wins_needed: u64,
        id: String
    ) -> ProgramResult {
        let moneymatchr = &mut ctx.accounts.moneymatchr;
        let player = &mut ctx.accounts.player;

        // Number of needed wins needs to be positive
        if wins_needed % 2 == 0 {
            return Err(ErrorCode::EvenMatchesForbidden.into());
        }

        // You have to bet at least 1 lamport
        if amount <= 0 {
            return Err(ErrorCode::AmountNeedsToBePositive.into());
        }

        moneymatchr.id = id;
        moneymatchr.initiator = player.key();
        moneymatchr.vault = moneymatchr.key();
        moneymatchr.amount = amount;
        moneymatchr.wins_needed = wins_needed;
        moneymatchr.challenger_wins = 0;
        moneymatchr.initiator_wins = 0;
        moneymatchr.state = 0;

        // Prepare instruction for transfering lamports from initiator to program account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.player.key(),
            &ctx.accounts.moneymatchr.key(),
            amount,
        );

        // Execute a signed tx on behalf of program account
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.moneymatchr.to_account_info(),
            ],
            &[
                &[
                    SEED.as_ref(),
                    ctx.accounts.player.key.as_ref(),
                    &[bump]
                ]
            ]
        )
    }

    pub fn register(_ctx: Context<Register>, _amount: u64) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub player: AccountInfo<'info>,
    #[account(
        init,
        seeds = [
            SEED.as_ref(),
            player.key.as_ref()
        ],
        bump = bump,
        payer = player,
        space = 256
    )]
    pub moneymatchr: Account<'info, Moneymatchr>,
    pub system_program: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub moneymatchr: Account<'info, Moneymatchr>,
    #[account(mut)]
    pub player: AccountInfo<'info>,
}

#[account]
pub struct Moneymatchr {
    pub id: String,
    pub amount: u64,
    pub initiator: Pubkey,
    pub challenger: Pubkey,
    pub vault: Pubkey,
    pub state: u64,
    pub wins_needed: u64,
    pub initiator_wins: u64,
    pub challenger_wins: u64
}

#[error]
pub enum ErrorCode {
    #[msg("Amount does not match lamports")]
    Register,
    #[msg("Number of matches has to be odd")]
    EvenMatchesForbidden,
    #[msg("Amount of lamports needs to be positive")]
    AmountNeedsToBePositive,
}