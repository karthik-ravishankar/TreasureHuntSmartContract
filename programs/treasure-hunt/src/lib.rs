use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use squads_multisig_program::cpi::accounts::Execute as SquadsExecute;
use squads_multisig_program::program::SquadsMultisig;

declare_id!("Your_Program_ID");

#[program]
pub mod treasure_hunt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, team_size: u8, required_nfts: Vec<Pubkey>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.team = ctx.accounts.team.key();
        game.prize_mint = ctx.accounts.prize_mint.key();
        game.prize_account = ctx.accounts.prize_account.key();
        game.team_size = team_size;
        game.required_nfts = required_nfts;
        game.collected_nfts = 0;
        game.is_completed = false;
        Ok(())
    }

    pub fn collect_nft(ctx: Context<CollectNFT>, nft_mint: Pubkey) -> Result<()> {
        let game = &mut ctx.accounts.game;
        require!(!game.is_completed, ErrorCode::GameAlreadyCompleted);
        require!(game.required_nfts.contains(&nft_mint), ErrorCode::InvalidNFT);

        // TODO: Add logic to verify NFT ownership

        game.collected_nfts += 1;
        if game.collected_nfts == game.required_nfts.len() as u8 {
            game.is_completed = true;
        }
        Ok(())
    }

    pub fn redeem_prize(ctx: Context<RedeemPrize>) -> Result<()> {
        let game = &ctx.accounts.game;
        require!(game.is_completed, ErrorCode::GameNotCompleted);

        // Transfer prize tokens from escrow to the team's multisig wallet
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.prize_account.to_account_info(),
            to: ctx.accounts.team_prize_account.to_account_info(),
            authority: ctx.accounts.game.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, ctx.accounts.prize_account.amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 1 + 32 * 10 + 1 + 1)]
    pub game: Account<'info, Game>,
    pub team: AccountInfo<'info>,
    pub prize_mint: Account<'info, Mint>,
    #[account(mut)]
    pub prize_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CollectNFT<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub team_member: Signer<'info>,
}

#[derive(Accounts)]
pub struct RedeemPrize<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub prize_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_prize_account: Account<'info, TokenAccount>,
    pub team: AccountInfo<'info>,
    pub squads_program: Program<'info, SquadsMultisig>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Game {
    pub team: Pubkey,
    pub prize_mint: Pubkey,
    pub prize_account: Pubkey,
    pub team_size: u8,
    pub required_nfts: Vec<Pubkey>,
    pub collected_nfts: u8,
    pub is_completed: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The game is already completed")]
    GameAlreadyCompleted,
    #[msg("Invalid NFT for this treasure hunt")]
    InvalidNFT,
    #[msg("The game is not completed yet")]
    GameNotCompleted,
}