use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
use anchor_lang::system_program::{transfer, Transfer};
use crate::gas_system::vizing_gas_system::MappingFeeConfig;
use crate::gas_system::*;
use crate::governance::*;
use crate::library::*;
use crate::state::*;
use crate::vizing_omni::*;

#[derive(Accounts)]
pub struct LaunchOp<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub fee_payer: AccountInfo<'info>,

    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub message_authority: AccountInfo<'info>,

    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.is_paused != true @VizingError::VizingNotActivated)]
    pub vizing: Account<'info, VizingPadSettings>,

    /// CHECK: We need this account as to receive the fee
    #[account(mut, constraint = fee_collector.key() == vizing.fee_receiver @VizingError::FeeCollectorInvalid)]
    pub fee_collector: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,

    pub system_program: Program<'info, System>,
}

impl LaunchOp<'_> {
    pub fn vizing_launch(ctx: &mut Context<LaunchOp>, params: LaunchParams) -> Result<()> {
        msg!(
            "message_authority: {}",
            ctx.accounts.message_authority.key()
        );

        msg!(
            "erliest_arrival_timestamp: {}",
            params.erliest_arrival_timestamp
        );
        msg!(
            "latest_arrival_timestamp: {}",
            params.latest_arrival_timestamp
        );
        msg!("value: {}", params.value);

        msg!("dest_chainid: {}", params.dest_chainid);

        msg!("addition_params: {:?}", params.addition_params);

        msg!("mode: {}", params.message.mode);

        msg!("execute_gas_limit: {}", params.message.execute_gas_limit);

        msg!("max_fee_per_gas: {}", params.message.max_fee_per_gas);

        msg!("signature: {:?}", params.message.signature);

        let message = &params.message;
        let serialized_data: Vec<u8> = message.try_to_vec()?;

        msg!("serialized_data: {:?}", serialized_data);

        let dest_chain_id = params.dest_chainid;

        let dapp = &params.message.target_contract;
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let gas_system_global = &mut ctx.accounts.gas_system_global;

        let get_fee_config = mapping_fee_config
            .get_fee_config(dest_chain_id)
            .ok_or(errors::ErrorCode::FeeConfigNotFound)?;
        let get_trade_fee_config = mapping_fee_config
            .get_trade_fee_config(dest_chain_id, *dapp)
            .ok_or(errors::ErrorCode::TradeFeeConfigNotFound)?;
        let get_dapp_config = mapping_fee_config
            .get_dapp_config(dest_chain_id, *dapp)
            .ok_or(errors::ErrorCode::DappConfigNotFound)?;

        let fee = vizing_gas_system::estimate_gas(
            gas_system_global.global_base_price,
            get_fee_config.base_price,
            get_dapp_config.value,
            get_fee_config.molecular_decimal,
            get_fee_config.denominator_decimal,
            get_fee_config.molecular,
            get_trade_fee_config.molecular,
            get_trade_fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            gas_system_global.default_gas_limit,
            params.value,
            dest_chain_id,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateGasNotFound)?;

        // mock fee
        // let fee: u64 = 1000000000;
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.fee_payer.to_account_info(),
                    to: ctx.accounts.fee_collector.to_account_info(),
                },
            ),
            fee,
        )?;

        emit!(SuccessfulLaunchMessage {
            erliest_arrival_timestamp: params.erliest_arrival_timestamp,
            latest_arrival_timestamp: params.latest_arrival_timestamp,
            relayer: params.relayer,
            sender: params.sender,
            src_contract: ctx.accounts.message_authority.key(),
            value: params.value,
            fee: fee,
            dest_chainid: params.dest_chainid,
            addition_params: params.addition_params,
            message: params.message.clone(),
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(params: LandingParams)]
pub struct LandingOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub relayer: AccountInfo<'info>,

    #[account(mut, 
        seeds = [contants::VIZING_PAD_SETTINGS_SEED], 
        bump = vizing.bump,
        constraint = vizing.trusted_relayers.contains(&relayer.key()) @VizingError::NotRelayer, 
        constraint = vizing.is_paused != true @VizingError::VizingNotActivated
    )]
    pub vizing: Account<'info, VizingPadSettings>,

    /// CHECK: We need this PDA as a signer
    #[account(seeds = [contants::VIZING_AUTHORITY_SEED],bump = vizing_authority.bump)]
    pub vizing_authority: Account<'info, VizingAuthorityParams>,

    /// CHECK: target contract
    #[account(mut, constraint = target_contract.key() == params.message.target_contract @VizingError::TargetContractInvalid)]
    pub target_contract: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl LandingOp<'_> {
    pub fn vizing_landing(ctx: &mut Context<LandingOp>, params: LandingParams) -> Result<()> {
        let balance_before = ctx.accounts.relayer.lamports();

        let account_info = ctx
            .remaining_accounts
            .iter()
            .map(|acc| {
                let mut account = acc.to_account_metas(None)[0].clone();
                account.is_signer = account.pubkey == ctx.accounts.vizing_authority.key();
                account
            }).collect::<Vec<_>>();

        let ix = Instruction {
            program_id: ctx.accounts.target_contract.key(),
            accounts: account_info,
            data: build_landing_ix_data(&params)?,
        };

        invoke_signed(
            &ix,
            &[ctx.remaining_accounts].concat(),
            &[&[VIZING_AUTHORITY_SEED, &[ctx.accounts.vizing_authority.bump]]],
        ).map_err(|_| VizingError::CallingFailed)?;

        require!(
            ctx.accounts.relayer.lamports() <= balance_before + params.value,
            VizingError::InsufficientBalance
        );

        emit!(SuccessfulLanding {
            message_id: params.message_id,
            erliest_arrival_timestamp: params.erliest_arrival_timestamp,
            latest_arrival_timestamp: params.latest_arrival_timestamp,
            src_chainid: params.src_chainid,
            src_tx_hash: params.src_tx_hash,
            src_contract: params.src_contract,
            src_chain_nonce: params.src_chain_nonce,
            sender: params.sender,
            value: params.value,
            addition_params: params.addition_params,
            message: params.message,
        });

        Ok(())
    }
}

fn build_landing_ix_data(params: &LandingParams) -> Result<Vec<u8>> {
    let mut data = Vec::with_capacity(LandingParams::INIT_SPACE);
    data.extend(RECEIVE_FROM_VIZING_DISCRIMINATOR);
    params.serialize(&mut data)?;
    Ok(data)
}

//get
#[derive(Accounts)]
pub struct ComputeTradeFee1<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
}
impl ComputeTradeFee1<'_> {
    pub fn get_compute_trade_fee1(
        ctx: Context<ComputeTradeFee1>,
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<u64> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let fee_config = mapping_fee_config.get_fee_config(dest_chain_id).ok_or(errors::ErrorCode::FeeConfigNotFound)?;
        let gas_system_global = &mut ctx.accounts.gas_system_global;
        let fee: u64 = vizing_gas_system::compute_trade_fee1(
            fee_config.molecular,
            fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            dest_chain_id,
            amount_out,
        ).ok_or(errors::ErrorCode::ComputeTradeFee1NotFound)?;
        Ok(fee)
    }
}
#[derive(Accounts)]
pub struct ComputeTradeFee2<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
}
impl ComputeTradeFee2<'_> {
    pub fn get_compute_trade_fee2(
        ctx: Context<ComputeTradeFee2>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<u64> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let trade_fee_config = mapping_fee_config.get_trade_fee_config(dest_chain_id,target_contract).ok_or(errors::ErrorCode::TradeFeeConfigNotFound)?;
        let gas_system_global = &mut ctx.accounts.gas_system_global;
        let fee: u64 = vizing_gas_system::compute_trade_fee2(
            trade_fee_config.molecular,
            trade_fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            target_contract,
            dest_chain_id,
            amount_out,
        ).ok_or(errors::ErrorCode::ComputeTradeFee2NotFound)?;
        Ok(fee)
    }
}

#[derive(Accounts)]
pub struct EstimatePrice2<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
}
impl EstimatePrice2<'_> {
    pub fn get_estimate_price2(
        ctx: Context<EstimatePrice2>,
        dest_chain_id: u64,
    ) -> Result<u64> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let fee_config = mapping_fee_config.get_fee_config(dest_chain_id).ok_or(errors::ErrorCode::FeeConfigNotFound)?;
        let gas_system_global = &mut ctx.accounts.gas_system_global;

        let base_price: u64 = vizing_gas_system::estimate_price2(
            gas_system_global.global_base_price,
            fee_config.base_price,
            dest_chain_id,
        ).ok_or(errors::ErrorCode::EstimatePrice2NotFound)?;
        Ok(base_price)
    }
}

#[derive(Accounts)]
pub struct EstimateGas<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>
}
impl EstimateGas<'_> {
    pub fn get_estimate_gas(
        ctx: Context<EstimateGas>,
        amount_out: u64,
        dest_chain_id: u64,
        message: Message,
    ) -> Result<u64> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let gas_system_global = &mut ctx.accounts.gas_system_global;

        let serialized_data: Vec<u8> = message.try_to_vec()?;
        let Some((_, dapp, _, _, _))=message_monitor::slice_message(&serialized_data) else { todo!() };

        let fee_config = mapping_fee_config.get_fee_config(dest_chain_id).ok_or(errors::ErrorCode::FeeConfigNotFound)?;
        let trade_fee_config = mapping_fee_config.get_trade_fee_config(dest_chain_id,dapp).ok_or(errors::ErrorCode::TradeFeeConfigNotFound)?;
        let dapp_config = mapping_fee_config.get_dapp_config(dest_chain_id,dapp).ok_or(errors::ErrorCode::DappConfigNotFound)?;

        let fee: u64 = vizing_gas_system::estimate_gas(
            gas_system_global.global_base_price,
            fee_config.base_price,
            dapp_config.value,
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            fee_config.molecular,
            trade_fee_config.molecular,
            trade_fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            gas_system_global.default_gas_limit,
            amount_out,
            dest_chain_id,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateGasNotFound)?;
        Ok(fee)
    }
}

#[derive(Accounts)]
pub struct EstimateTotalFee<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
    #[account(
        mut,
        seeds = [b"amount_in_thresholds".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub amount_in_thresholds: Account<'info, MappingAmountInThresholds>,
}
impl EstimateTotalFee<'_> {
    pub fn get_estimate_total_fee(
        ctx: Context<EstimateTotalFee>,
        dest_chain_id: u64,
        amount_out: u64,
        message: Message,
    ) -> Result<u64> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let gas_system_global = &mut ctx.accounts.gas_system_global;
        let mapping_amount_in_thresholds = &mut ctx.accounts.amount_in_thresholds;

        let serialized_data: Vec<u8> = message.try_to_vec()?;
        let Some((_, dapp, _, _, _))=message_monitor::slice_message(&serialized_data) else { todo!() };

        let fee_config = mapping_fee_config.get_fee_config(dest_chain_id).ok_or(errors::ErrorCode::FeeConfigNotFound)?;
        let trade_fee_config = mapping_fee_config.get_trade_fee_config(dest_chain_id,dapp).ok_or(errors::ErrorCode::TradeFeeConfigNotFound)?;
        let dapp_config = mapping_fee_config.get_dapp_config(dest_chain_id,dapp).ok_or(errors::ErrorCode::DappConfigNotFound)?;
        let token_amount_limit=mapping_amount_in_thresholds.get_amount_in_thresholds(dest_chain_id).ok_or(errors::ErrorCode::AmountInThresholdsNotFound)?;

        let fee: u64 = vizing_gas_system::estimate_total_fee(
            token_amount_limit,
            trade_fee_config.molecular,
            trade_fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            dapp_config.value,
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            fee_config.molecular,
            gas_system_global.default_gas_limit,
            gas_system_global.global_base_price,
            fee_config.base_price,
            dest_chain_id,
            amount_out,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateTotalFeeNotFound)?;
        Ok(fee)
    }
}

#[derive(Accounts)]
pub struct ExactOutput<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
}
impl ExactOutput<'_> {
    pub fn get_exact_output(
        ctx: Context<ExactOutput>,
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<u64> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let fee_config = mapping_fee_config.get_fee_config(dest_chain_id).ok_or(errors::ErrorCode::FeeConfigNotFound)?;

        let amount_in: u64 = vizing_gas_system::exact_output(
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            dest_chain_id,
            amount_out,
        ).ok_or(errors::ErrorCode::ExactOutputNotFound)?;
        Ok(amount_in)
    }
}

#[derive(Accounts)]
pub struct ExactInput<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
}
impl ExactInput<'_> {
    pub fn get_exact_input(
        ctx: Context<ExactInput>,
        dest_chain_id: u64,
        amount_in: u64,
    ) -> Result<u64> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let fee_config = mapping_fee_config.get_fee_config(dest_chain_id).ok_or(errors::ErrorCode::FeeConfigNotFound)?;
        
        let amount_out: u64 = vizing_gas_system::exact_input(
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            dest_chain_id,
            amount_in,
        ).ok_or(errors::ErrorCode::ExactInputputNotFound)?;
        Ok(amount_out)
    }
}
