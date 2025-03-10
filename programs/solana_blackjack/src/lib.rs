use anchor_lang::prelude::*;
use crate::blackjack::game;
use crate::blackjack::game::*;
use crate::blackjack::player::*;
use crate::blackjack::treasury;
use crate::blackjack::treasury::*;

pub mod utils;
pub mod blackjack;

declare_id!("9dpmwY9gDHNohq7SeHgegqhTKDJjaJ8JP8QP7pxbSdCr");
/*
TODO

 - You can always optimize the code, but it's not necessary for this project (lower compute unit consumption)

*/

pub const ADMIN_KEY: Pubkey = Pubkey::new_from_array([
    197, 167, 114, 217, 106, 43, 59, 136, 211, 194, 163, 225, 1, 184, 182, 126, 123, 150, 33, 112,
    183, 212, 150, 80, 85, 50, 4, 236, 154, 82, 155, 212,
]);

pub const TREASURY_SEED: &[u8] = b"treasury";

#[program]
pub mod solana_blackjack {
    use blackjack::{ player, ResetGame, WithdrawFunds };

    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, player: Pubkey) -> Result<()> {
        game::initialize_game(ctx, player)
    }

    pub fn place_bet(ctx: Context<GameContext>, bet_amount: u64) -> Result<()> {
        game::place_bet(ctx, bet_amount)
    }

    pub fn hit(ctx: Context<GameContext>) -> Result<()> {
        game::hit(ctx)
    }

    pub fn stand(ctx: Context<GameContext>) -> Result<()> {
        game::stand(ctx)
    }

    //Player stuff

    pub fn add_funds(ctx: Context<AddFunds>, lamports: u64) -> Result<()> {
        player::add_funds(ctx, lamports)
    }

    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, amount: u64) -> Result<()> {
        player::withdraw_funds(ctx, amount)
    }

    pub fn reset_game(ctx: Context<ResetGame>) -> Result<()> {
        player::reset_game(ctx)
    }

    //ADMIN STUFF
    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        treasury::initialize_treasury(ctx)
    }

    pub fn add_funds_to_treasury(ctx: Context<AddFundsToTreasury>, amount: u64) -> Result<()> {
        treasury::add_funds_to_treasury(ctx, amount)
    }

    pub fn withdraw_from_treasury(ctx: Context<WithdrawFromTreasury>, amount: u64) -> Result<()> {
        treasury::withdraw_from_treasury(ctx, amount)
    }
}
