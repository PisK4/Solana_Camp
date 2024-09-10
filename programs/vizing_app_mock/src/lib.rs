pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_omni::*;

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

    pub fn initialize_vizing_emitter(ctx: Context<VizingEmitterInitialize>) -> Result<()> {
        VizingEmitterInitialize::handler(ctx)
    }

    pub fn launch_vizing(ctx: Context<LaunchAppOpTemplate>) -> Result<()> {
        let params = LaunchParams {
            erliest_arrival_timestamp: VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT,
            latest_arrival_timestamp: VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT,
            relayer: VIZING_RELAYER_DEFAULT,
            sender: ctx.accounts.user.key(),
            value: 886,
            dest_chainid: 28516,
            addition_params: AdditionalParams {
                mode: 0,
                signature: vec![],
            },
            message: Message {
                mode: 1,
                target_program: [
                    227, 2, 10, 198, 15, 69, 132, 42, 116, 127, 96, 8, 57, 13, 13, 40, 221, 187,
                    217, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                execute_gas_limit: VIZING_GASLIMIT_DEFAULT,
                max_fee_per_gas: 100,
                signature: vec![],
            },
        };

        launch_2_vizing(
            params,
            &ctx.program_id,
            &ctx.accounts.vizing_pad_program.to_account_info(),
            &ctx.accounts.user.to_account_info(),
            &ctx.accounts.vizing_app_message_authority.to_account_info(),
            &ctx.accounts.vizing_pad_config.to_account_info(),
            &ctx.accounts.vizing_pad_fee_collector.to_account_info(),
            &ctx.accounts.mapping_fee_config.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )

        // Ok(())
    }

    #[access_control(assert_vizing_authority(&ctx.accounts.vizing_authority))]
    pub fn receive_from_vizing(ctx: Context<LandingAppOp>, params: VizingMessage) -> Result<()> {
        msg!(
            "authority from vizing: {}",
            ctx.accounts.vizing_authority.key()
        );

        let sig_slice = &params.signature;
        let a_slice = &sig_slice[..8];
        let b_slice = &sig_slice[8..16];

        let mut a = [0u8; 8];
        a.copy_from_slice(a_slice);

        let mut b = [0u8; 8];
        b.copy_from_slice(b_slice);

        let a_number = u64::from_be_bytes(a);
        let b_number = u64::from_be_bytes(b);

        let c = a_number + b_number;

        ctx.accounts.result_account.result = c;

        msg!("{} + {} = {}", a_number, b_number, c);

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
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,

    #[account(seeds = [VIZING_MESSAGE_AUTHORITY_SEED], bump = vizing_app_message_authority.bump)]
    pub vizing_app_message_authority: Account<'info, VizingMessageAuthority>,

    /// CHECK: 1. Vizing config account
    pub vizing_pad_config: AccountInfo<'info>,

    /// CHECK: 2. Vizing fee collector account
    #[account(mut)]
    pub vizing_pad_fee_collector: AccountInfo<'info>,

    /// CHECK: 3. Vizing Pad
    pub vizing_pad_program: AccountInfo<'info>,

    /// CHECK: 4. Vizing fee account
    pub mapping_fee_config: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LandingAppOp<'info> {
    /// CHECK: 1. Vizing Authority account
    #[account(signer)]
    pub vizing_authority: AccountInfo<'info>,

    #[account(seeds = [VIZING_APP_SOL_RECEIVER_SEED], bump = sol_pda_receiver.bump)]
    pub sol_pda_receiver: Account<'info, VizingSolReceiver>,

    /// CHECK: 2. Vizing config account
    pub vizing_pad_config: AccountInfo<'info>,

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
