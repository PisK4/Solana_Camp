pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_omni::*;

declare_id!("mokB6FzEZx6vPVmasd19CyDDuqZ98auke1Bk59hmzVE");

pub const RESULT_DATA_SEED: &[u8] = b"result_data_seed";

// 0x4d20A067461fD60379DA001EdEC6E8CFb9862cE4
pub const TRUSTED_ENDPOINT: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x20, 0xa0, 0x67,
    0x46, 0x1f, 0xd6, 0x03, 0x79, 0xda, 0x00, 0x1e, 0xde, 0xc6, 0xe8, 0xcf, 0xb9, 0x86, 0x2c, 0xe4,
];

#[program]
pub mod vizing_app_mock {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.result_account.result = 0;
        let (_, bump) = Pubkey::find_program_address(&[RESULT_DATA_SEED], &ctx.program_id);
        ctx.accounts.result_account.bump = bump;
        Ok(())
    }

    pub fn initialize_vizing_receiver(ctx: Context<VizingSolReceiverInitialize>) -> Result<()> {
        VizingSolReceiverInitialize::handler(ctx)
    }

    pub fn initialize_vizing_emitter(ctx: Context<VizingEmitterInitialize>) -> Result<()> {
        VizingEmitterInitialize::handler(ctx)
    }

    pub fn launch_vizing(
        ctx: Context<LaunchAppOpTemplate>,
        target_program: [u8; 32],
        meta: Vec<u8>,
    ) -> Result<()> {
        let params = LaunchParams {
            erliest_arrival_timestamp: VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT,
            latest_arrival_timestamp: VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT,
            relayer: VIZING_RELAYER_DEFAULT,
            sender: ctx.accounts.user.key(),
            value: 0,
            dest_chainid: 28516,
            addition_params: AdditionalParams {
                mode: 0,
                signature: vec![],
            },
            message: Message {
                mode: 1,
                target_program,
                execute_gas_limit: 200000,
                max_fee_per_gas: 35,
                signature: meta.clone(),
            },
        };

        msg!("meta: {:?}", meta);

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
    pub fn receive_from_vizing(
        ctx: Context<LandingAppOpTemplate>,
        params: VizingMessage,
    ) -> Result<()> {
        require!(
            TRUSTED_ENDPOINT == params.src_contract,
            ErrorCode::ConstraintAddress
        );

        msg!("src_chainid: {}", params.src_chainid);

        msg!("src_contract: {:?}", params.src_contract);

        msg!("value: {}", params.value);

        msg!("signature: {:?}", params.signature);

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
pub struct LandingAppOpTemplate<'info> {
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
