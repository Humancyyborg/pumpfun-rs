//! Instruction for buying tokens from bonding curves
//!
//! This module provides the functionality to buy tokens from bonding curves.
//! It includes the instruction data structure and helper function to build the Solana instruction.

use crate::{constants, PumpFun};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};
use spl_associated_token_account::get_associated_token_address;

/// Instruction data for buying tokens from a bonding curve
///
/// # Fields
///
/// * `amount` - Amount of tokens to buy (in token smallest units)
/// * `max_sol_cost` - Maximum acceptable SOL cost for the purchase (slippage protection)
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct Buy {
    pub amount: u64,
    pub max_sol_cost: u64,
}

impl Buy {
    /// Instruction discriminator used to identify this instruction
    pub const DISCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];

    /// Serializes the instruction data with the appropriate discriminator
    ///
    /// # Returns
    ///
    /// Byte vector containing the serialized instruction data
    pub fn data(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(256);
        data.extend_from_slice(&Self::DISCRIMINATOR);
        self.serialize(&mut data).unwrap();
        data
    }
}

/// Creates an instruction to buy tokens from a bonding curve
///
/// Buys tokens by providing SOL. The amount of tokens received is calculated based on
/// the bonding curve formula. A portion of the SOL is taken as a fee and sent to the
/// fee recipient account. The price increases as more tokens are purchased according to
/// the bonding curve function.
pub fn buy(
    payer: &Keypair,
    mint: &Pubkey,
    fee_recipient: &Pubkey,
    creator: &Pubkey,
    args: Buy,
) -> Instruction {
    let bonding_curve: Pubkey = PumpFun::get_bonding_curve_pda(mint).unwrap();
    let creator_vault: Pubkey = PumpFun::get_creator_vault_pda(creator).unwrap();
    Instruction::new_with_bytes(
        constants::accounts::PUMPFUN,
        &args.data(),
        vec![
            AccountMeta::new_readonly(PumpFun::get_global_pda(), false),
            AccountMeta::new_readonly(PumpFun::get_fee_config_pda(), false), // 👈 NEW account
            AccountMeta::new(*fee_recipient, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(get_associated_token_address(&bonding_curve, mint), false),
            AccountMeta::new(get_associated_token_address(&payer.pubkey(), mint), false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(constants::accounts::SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::TOKEN_PROGRAM, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(constants::accounts::EVENT_AUTHORITY, false),
            AccountMeta::new_readonly(constants::accounts::PUMPFUN, false),
            AccountMeta::new(constants::accounts::GLOBAL_VOLUME_ACCUMULATOR, false),
            AccountMeta::new(
                PumpFun::get_user_volume_accumulator_pda(&payer.pubkey()),
                false,
            ),
        ],
    )
}
