use anchor_lang::prelude::*;
use vizing_pad::{
    self,
    cpi::accounts::{
        ComputeTradeFee1, ComputeTradeFee2, EstimateGas, EstimatePrice1, EstimatePrice2,
        EstimateTotalFee, EstimateVizingGasFee1, EstimateVizingGasFee2, ExactInput, ExactOutput,
        LaunchOp,
    },
    cpi::{
        compute_trade_fee1, compute_trade_fee2, estimate_gas, estimate_price1, estimate_price2,
        estimate_total_fee, estimate_vizing_gas_fee1, estimate_vizing_gas_fee2, exact_input,
        exact_output, launch,
    },
};

pub const VIZING_MESSAGE_AUTHORITY_SEED: &[u8] = b"Vizing_Message_Authority_Seed";
pub const VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_RELAYER_DEFAULT: [u8; 32] = [0; 32];
pub const VIZING_GASLIMIT_DEFAULT: u64 = 10000000;

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
        let (_, bump) =
            Pubkey::find_program_address(&[VIZING_MESSAGE_AUTHORITY_SEED], &ctx.program_id);
        ctx.accounts.message_pda_authority.bump = bump;
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
    pub execute_gas_limit: u32,
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
    mapping_fee_config: &AccountInfo<'info>,
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
            mapping_fee_config: mapping_fee_config.clone(),
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

//dev get data
pub fn fetch_compute_trade_fee1<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    dest_chain_id: u64,
    amount_out: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        ComputeTradeFee1 {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = compute_trade_fee1(cpi_ctx, dest_chain_id, amount_out);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_compute_trade_fee2<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    target_program: [u8; 32],
    dest_chain_id: u64,
    amount_out: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        ComputeTradeFee2 {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = compute_trade_fee2(cpi_ctx, target_program, dest_chain_id, amount_out);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_estimate_price1<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    target_program: [u8; 32],
    dest_chain_id: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        EstimatePrice1 {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = estimate_price1(cpi_ctx, target_program, dest_chain_id);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_estimate_price2<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    dest_chain_id: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        EstimatePrice2 {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = estimate_price2(cpi_ctx, dest_chain_id);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_estimate_gas<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    amount_out: u64,
    dest_chain_id: u64,
    message: Message,
) -> Result<()> {
    let cpi_message = vizing_pad::vizing_omni::Message {
        mode: message.mode,
        target_program: message.target_program,
        execute_gas_limit: message.execute_gas_limit,
        max_fee_per_gas: message.max_fee_per_gas,
        signature: message.signature,
    };

    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        EstimateGas {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = estimate_gas(cpi_ctx, amount_out, dest_chain_id, cpi_message);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_estimate_total_fee<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    amount_out: u64,
    dest_chain_id: u64,
    message: Message,
) -> Result<()> {
    let cpi_message = vizing_pad::vizing_omni::Message {
        mode: message.mode,
        target_program: message.target_program,
        execute_gas_limit: message.execute_gas_limit,
        max_fee_per_gas: message.max_fee_per_gas,
        signature: message.signature,
    };

    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        EstimateTotalFee {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = estimate_total_fee(cpi_ctx, amount_out, dest_chain_id, cpi_message);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_exact_output<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    dest_chain_id: u64,
    amount_out: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        ExactOutput {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = exact_output(cpi_ctx, dest_chain_id, amount_out);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_exact_input<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    dest_chain_id: u64,
    amount_in: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        ExactInput {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = exact_input(cpi_ctx, dest_chain_id, amount_in);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_estimate_vizing_gas_fee1<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    value: u64,
    dest_chain_id: u64,
    _addition_params: Vec<u8>,
    message: Vec<u8>,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        EstimateVizingGasFee1 {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res = estimate_vizing_gas_fee1(cpi_ctx, value, dest_chain_id, _addition_params, message);

    if res.is_ok() {
        return Ok(());
    } else {
        return err!(AppErrors::VizingCallFailed);
    }
}

pub fn fetch_estimate_vizing_gas_fee2<'c: 'info, 'info>(
    vizing_pad_program: &AccountInfo<'info>,
    mapping_fee_config: &AccountInfo<'info>,
    current_record_message: &AccountInfo<'info>,
    value: u64,
    dest_chain_id: u64,
    _addition_params: Vec<u8>,
    message: Message,
) -> Result<()> {
    let cpi_message = vizing_pad::vizing_omni::Message {
        mode: message.mode,
        target_program: message.target_program,
        execute_gas_limit: message.execute_gas_limit,
        max_fee_per_gas: message.max_fee_per_gas,
        signature: message.signature,
    };

    let cpi_ctx = CpiContext::new(
        vizing_pad_program.clone(),
        EstimateVizingGasFee2 {
            mapping_fee_config: mapping_fee_config.clone(),
            current_record_message: current_record_message.clone(),
        },
    );

    let res =
        estimate_vizing_gas_fee2(cpi_ctx, value, dest_chain_id, _addition_params, cpi_message);

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