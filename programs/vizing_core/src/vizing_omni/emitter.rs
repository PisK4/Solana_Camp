use anchor_lang::prelude::*;
use vizing_pad::{self, cpi::accounts::LaunchOp, cpi::launch};

pub const VIZING_MESSAGE_AUTHORITY_SEED: &[u8] = b"Vizing_Message_Authority_Seed";
pub const VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_RELAYER_DEFAULT: [u8; 32] = [0; 32];
pub const VIZING_GASLIMIT_DEFAULT: u64 = 0;

#[derive(Accounts)]
pub struct VizingEmitterInitialize<'info> {
    #[account(init, payer = payer, space = 8 + VizingMessageAuthority::INIT_SPACE, seeds = [VIZING_MESSAGE_AUTHORITY_SEED], bump)]
    pub message_pda_authority: Account<'info, VizingMessageAuthority>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VizingFeeRouter {
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VizingMessageAuthority {
    pub bump: u8,
}

impl VizingEmitterInitialize<'_> {
    pub fn handler(ctx: Context<Self>) -> Result<()> {
        ctx.accounts.message_pda_authority.bump = *ctx.bumps.get("message_pda_authority").unwrap();
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct LaunchParams {
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub relayer: [u8; 32],
    pub sender: Pubkey,
    pub value: u64,
    pub dest_chainid: u64,
    pub addition_params: AdditionalParams,
    pub message: Message,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Message {
    pub mode: u8,
    pub target_program: [u8; 32],
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(1024)]
    pub signature: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct AdditionalParams {
    pub mode: u8,
    #[max_len(512)]
    pub signature: Vec<u8>,
}

pub fn launch_2_vizing<'c: 'info, 'info>(
    launch_params: LaunchParams,
    curr_program_id: &Pubkey,
    vizing_pad_program: &AccountInfo<'info>,
    vizing_app_fee_payer: &AccountInfo<'info>,
    vizing_app_message_authority: &AccountInfo<'info>,
    vizing_pad_config: &AccountInfo<'info>,
    vizing_pad_fee_collector: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
) -> Result<()> {
    let params = vizing_pad::vizing_omni::LaunchParams {
        erliest_arrival_timestamp: launch_params.erliest_arrival_timestamp,
        latest_arrival_timestamp: launch_params.latest_arrival_timestamp,
        relayer: launch_params.relayer,
        sender: launch_params.sender,
        value: launch_params.value,
        dest_chainid: launch_params.dest_chainid,
        addition_params: vizing_pad::vizing_omni::AdditionalParams {
            mode: launch_params.addition_params.mode,
            signature: launch_params.addition_params.signature,
        },
        message: vizing_pad::vizing_omni::Message {
            mode: launch_params.message.mode,
            target_program: launch_params.message.target_program,
            execute_gas_limit: launch_params.message.execute_gas_limit,
            max_fee_per_gas: launch_params.message.max_fee_per_gas,
            signature: launch_params.message.signature,
        },
    };

    let (_, bump_authority) =
        Pubkey::find_program_address(&[VIZING_MESSAGE_AUTHORITY_SEED], curr_program_id);

    let seeds = &[VIZING_MESSAGE_AUTHORITY_SEED, &[bump_authority]];

    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        vizing_pad_program.clone(),
        LaunchOp {
            vizing_app_fee_payer: vizing_app_fee_payer.clone(),
            vizing_app_message_authority: vizing_app_message_authority.clone(),
            vizing_pad_config: vizing_pad_config.clone(),
            vizing_pad_fee_collector: vizing_pad_fee_collector.clone(),
            system_program: system_program.clone(),
        },
        signer,
    );

    let res = launch(cpi_ctx, params);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

#[error_code]
pub enum AppErrors {
    #[msg("vizing call failed")]
    VizingCallFailed,
}
