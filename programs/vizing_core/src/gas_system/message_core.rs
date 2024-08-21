// use anchor_lang::prelude::*;
// use anchor_lang::system_program::{transfer, Transfer as SolTransfer};
// use anchor_spl::token::{self, Token, TokenAccount, Transfer};
// use spl_associated_token_account::get_associated_token_address;

// pub mod error;
// pub mod state;

// declare_id!("Ga4UfvXHBB4V1FgA5bvvrHT4gg7rraGLG1vshzxndW4i");

// #[program]
// mod message_core {

//     pub fn launch(
//         ctx: Context<Launch>,
//         earliest_arrival_timestamp: u64,
//         latest_arrival_timestamp: u64,
//         relayer: Pubkey,
//         sender: Pubkey,
//         value: u64,
//         dest_chain_id: u64,
//         addition_params: Vec<u8>,
//         message: Vec<u8>,
//     ) -> Result<()> {

//     }

//     pub fn landing(
//         ctx: Context<LaunchMultiChain>,
//         message_id: Vec<u8>,
//         earliest_arrival_timestamp: u64,
//         latest_arrival_timestamp: u64,
//         src_chain_id: u64,
//         src_tx_hash: Vec<u8>,
//         src_contract: u64,
//         src_chain_nonce: u32,
//         sender: u64,
//         value: u64,
//         addition_params: Vec<u8>,
//         message: Vec<u8>,
//         proofs: Vec<u8>,
//     ) -> Result<()> {
//          Ok(())
//     }

//     pub fn pause_engine(ctx: Context<PauseEngine>, stop: u8) -> Result<()> {
//         let power_user = &mut ctx.accounts.power_user;
//         let user_key = &mut ctx.accounts.user.key();
//         require!(power_user.engine_admin == user_key, ErrorCode::NonEngine);
//         let chain_state = &mut ctx.accounts.chain_state;
//         chain_state.engine_state = stop;
//         Ok(())
//     }

//     pub fn estimate_gas(
//         ctx: Context<EstimateGas>,
//         valueGroup: Vec<u64>,
//         dest_chain_id_group: Vec<u64>,
//         addition_params: Vec<Vec<u8>>,
//         message: Vec<Vec<u8>>,
//     ) -> Result<()>{

//     }

//     pub fn estimate_price(
//         ctx: Context<EstimatePrice>,
//         sender: Pubkey,
//         dest_chain_id: u64,
//     ) -> Result<()>{

//     }

// }

// #[derive(Accounts)]
// pub struct Launch<'info> {}

// #[derive(Accounts)]
// pub struct PauseEngine<'info> {
//     #[account(
//         mut,
//         seeds = [b"init_power_user".as_ref()],
//         bump
//     )]
//     pub power_user: Account<'info, PowerUser>,
//     #[account(mut)]
//     pub chain_state: Account<'info, ChainState>,
//     #[account(mut)]
//     pub user: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct EstimateGas<'info> {

// }

// #[derive(Accounts)]
// pub struct EstimatePrice<'info> {

// }
