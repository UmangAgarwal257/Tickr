#![allow(unexpected_cfgs)]
use anchor_lang::{prelude::*, system_program::Transfer};

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{state::Marketplace, Listing};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,
    pub maker_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = maker_mint,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Box<Account<'info, Listing>>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Purchase<'info> {}
