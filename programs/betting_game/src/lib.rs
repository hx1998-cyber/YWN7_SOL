use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::{clock, rent};
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod betting_game {
    use super::*;

    const FIXED_BET_AMOUNT: u64 = 1_000_000; // 1 USDT (假设有6位小数)

    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64) -> ProgramResult {
        let betting_game = &mut ctx.accounts.betting_game;
        let player = &ctx.accounts.player;

        // Ensure the bet amount is fixed
        if bet_amount != FIXED_BET_AMOUNT {
            return Err(ErrorCode::InvalidBetAmount.into());
        }

        // Ensure that the player hasn't already placed a bet in this round
        if betting_game.players.contains(&player.key()) {
            return Err(ErrorCode::AlreadyBet.into());
        }

        // Ensure that the round hasn't reached the required number of players
        if betting_game.players.len() >= 10 {
            return Err(ErrorCode::RoundFull.into());
        }

        // Transfer bet_amount from player to the betting pool
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.player_token.to_account_info(),
            to: ctx.accounts.pool.to_account_info(),
            authority: player.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, bet_amount)?;

        betting_game.players.push(player.key());
        betting_game.total_bet += bet_amount;

        // If the round is full, select the winner
        if betting_game.players.len() == 10 {
            let winner = select_winner(&betting_game.players);
            distribute_prizes(&ctx.accounts.pool, &winner, betting_game.total_bet, ctx.accounts.treasury.key())?;
            betting_game.is_complete = true;
        }

        Ok(())
    }

    pub fn start_new_round(ctx: Context<StartNewRound>) -> ProgramResult {
        let betting_game = &mut ctx.accounts.betting_game;
        betting_game.players.clear();
        betting_game.total_bet = 0;
        betting_game.is_complete = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub betting_game: Account<'info, BettingGame>,
    pub player: Signer<'info>,
    #[account(mut)]
    pub player_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    pub treasury: AccountInfo<'info>, // 国库地址
}

#[derive(Accounts)]
pub struct StartNewRound<'info> {
    #[account(mut)]
    pub betting_game: Account<'info, BettingGame>,
    pub admin: Signer<'info>,
}

#[account]
pub struct BettingGame {
    pub players: Vec<Pubkey>,
    pub total_bet: u64,
    pub is_complete: bool,
}

fn select_winner(players: &[Pubkey]) -> Pubkey {
    // Simple random selection (use clock for randomness)
    let index = (clock::get().unix_timestamp as usize) % players.len();
    players[index]
}

fn distribute_prizes(pool: &AccountInfo, winner: &Pubkey, total_bet: u64, treasury: &Pubkey) -> ProgramResult {
    // Split the total bet and transfer the prizes
    let winner_prize = (total_bet * 70) / 100;
    let treasury_prize = total_bet - winner_prize;

    // Transfer winner's prize
    let cpi_accounts = token::Transfer {
        from: pool.to_account_info(),
        to: winner.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(pool.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, winner_prize)?;

    // Transfer treasury's prize
    let cpi_accounts_treasury = token::Transfer {
        from: pool.to_account_info(),
        to: treasury.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_ctx_treasury = CpiContext::new(pool.to_account_info(), cpi_accounts_treasury);
    token::transfer(cpi_ctx_treasury, treasury_prize)?;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Player has already placed a bet in this round.")]
    AlreadyBet,
    #[msg("The round is full. No more players can place bets.")]
    RoundFull,
    #[msg("Invalid bet amount.")]
    InvalidBetAmount,
}
