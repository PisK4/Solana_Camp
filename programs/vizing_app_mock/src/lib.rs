pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_omni::*;
use vizing_pad::program::VizingPad;
use vizing_pad::{self, cpi::accounts::LaunchOp};

declare_id!("2xiuj4ozxygvkmC1WKJTGZyJXSD8dtbFxWkuJiMLzrTg");

pub const RESULT_DATA_SEED: &[u8] = b"result_data_seed";

#[program]
pub mod vizing_app_mock {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.result_account.result = 0;
        ctx.accounts.result_account.bump = *ctx.bumps.get("result_account").unwrap();
        Ok(())
    }

    pub fn initialize_vizing_receiver(ctx: Context<VizingSolReceiverInitialize>) -> Result<()> {
        VizingSolReceiverInitialize::handler(ctx)
    }

    pub fn initialize_vizing_sender(ctx: Context<VizingEmitterInitialize>) -> Result<()> {
        VizingEmitterInitialize::handler(ctx)
    }

    pub fn launch2vizing(ctx: Context<LaunchAppOpTemplate>) -> Result<()> {
        let params = LaunchParams {
            erliest_arrival_timestamp: VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT,
            latest_arrival_timestamp: VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT,
            relayer: VIZING_RELAYER_DEFAULT,
            sender: ctx.accounts.user.key(),
            value: 0,
            dest_chainid: 1,
            addition_params: vec![],
            message: Message {
                mode: 1,
                target_program: [0; 32],
                execute_gas_limit: VIZING_GASLIMIT_DEFAULT,
                max_fee_per_gas: 0,
                signature: vec![],
            },
        };
        Ok(())
    }

    #[access_control(assert_vizing_authority(&ctx.accounts.vizing_authority))]
    pub fn receive_from_vizing(ctx: Context<LandingAppOp>, params: VizingMessage) -> Result<()> {
        msg!(
            "authority from vizing: {}",
            ctx.accounts.vizing_authority.key()
        );

        let a = 10;
        let b = 20;
        let c = a + b;

        ctx.accounts.result_account.result = c;

        msg!("{} + {} = {}", a, b, c);

        msg!("src_chainid: {}", params.src_chainid);

        msg!("src_contract: {:?}", params.src_contract);

        msg!("value: {}", params.value);

        msg!("signature: {:?}", params.signature);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LaunchAppOpTemplate<'info> {
    /// CHECK: 0. dapp Authority account
    #[account(signer)]
    pub user: AccountInfo<'info>,

    #[account(seeds = [VIZING_MESSAGE_AUTHORITY_SEED], bump = message_pda_authority.bump)]
    pub message_pda_authority: Account<'info, VizingMessageAuthority>,

    /// CHECK: 1. Vizing config account
    pub vizing: AccountInfo<'info>,

    /// CHECK: 2. Vizing fee collector account
    pub vizing_fee_collector: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LandingAppOp<'info> {
    /// CHECK: 1. Vizing Authority account
    #[account(signer)]
    pub vizing_authority: AccountInfo<'info>,

    /// CHECK: 2. Vizing config account
    pub vizing: AccountInfo<'info>,

    #[account(seeds = [VIZING_APP_SOL_RECEIVER_SEED], bump = sol_pda_receiver.bump)]
    pub sol_pda_receiver: Account<'info, VizingSolReceiver>,

    #[account(seeds = [RESULT_DATA_SEED], bump = result_account.bump)]
    pub result_account: Account<'info, ResultData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + ResultData::INIT_SPACE, seeds = [RESULT_DATA_SEED], bump)]
    pub result_account: Account<'info, ResultData>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct ResultData {
    pub result: u64,
    pub bump: u8,
}
