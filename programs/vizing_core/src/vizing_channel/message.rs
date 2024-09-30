use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::solana_program::program::set_return_data;
use crate::gas_system::*;
use crate::governance::*;
use crate::library::*;
use crate::vizing_omni::*;
use crate::library::{Uint256, VIZING_APP_CONFIG_SEED};

#[derive(Accounts)]
pub struct LaunchOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(mut, signer)]
    pub vizing_app_fee_payer: AccountInfo<'info>,
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub vizing_app_message_authority: AccountInfo<'info>,

    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.is_paused != true @VizingError::VizingNotActivated)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    /// CHECK: We need this account as to receive the fee
    #[account(mut, address = vizing_pad_config.fee_collector @VizingError::FeeCollectorInvalid)]
    pub vizing_pad_fee_collector: AccountInfo<'info>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    pub system_program: Program<'info, System>,
}

impl LaunchOp<'_> {
    pub fn vizing_launch(ctx: &mut Context<LaunchOp>, params: LaunchParams) -> Result<VizingReceipt> {
        msg!("### VizingLauchOp::launch ###");
        msg!("sender: {} authority: {}", ctx.accounts.vizing_app_fee_payer.key(), ctx.accounts.vizing_app_message_authority.key());
        msg!("destChainId:{}, destProgram: {:?}", params.dest_chainid, params.message.target_program);
        let message = &params.message;
        let serialized_data: Vec<u8> = message.try_to_vec()?;

        let dest_chain_id = params.dest_chainid;

        let dapp = &params.message.target_program;
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;

        let get_gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let get_fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let get_trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id, *dapp);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let dapp_config_value=get_trade_fee_config.value;
        let dest_value=Uint256::new(params.value.high,params.value.low);

        msg!("gaslimit: {}, price: {}", params.message.execute_gas_limit, params.message.max_fee_per_gas);

        let fee = vizing_gas_system::estimate_total_fee(
            get_gas_system_global.amount_in_threshold,
            trade_fee.molecular,
            trade_fee.denominator,
            get_trade_fee_config.molecular,
            get_trade_fee_config.denominator,
            get_gas_system_global.molecular,
            get_gas_system_global.denominator,
            dapp_config_value,
            get_fee_config.molecular_decimal,
            get_fee_config.denominator_decimal,
            get_fee_config.molecular,
            get_fee_config.denominator,
            get_gas_system_global.default_gas_limit,
            get_gas_system_global.global_base_price,
            get_fee_config.base_price,
            dest_chain_id,
            dest_value,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateGasNotFound)?;

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vizing_app_fee_payer.to_account_info(),
                    to: ctx.accounts.vizing_pad_fee_collector.to_account_info(),
                },
            ),
            fee,
        )?;
        
        msg!("fee:{} to fee_collector", fee);

        emit!(SuccessfulLaunchMessage {
            erliest_arrival_timestamp: params.erliest_arrival_timestamp,
            latest_arrival_timestamp: params.latest_arrival_timestamp,
            relayer: params.relayer,
            sender: params.sender,
            src_contract: ctx.accounts.vizing_app_message_authority.key(),
            value: dest_value,
            fee: fee,
            dest_chainid: params.dest_chainid,
            addition_params: params.addition_params,
            message: params.message,
            vizing_pad_config: ctx.accounts.vizing_pad_config.key(),
            vizing_gas_system_config: ctx.accounts.vizing_gas_system.key(),
        });

        Ok(
            VizingReceipt{
                fee
        })
    }
}

#[derive(Accounts)]
#[instruction(params: LandingParams)]
pub struct LandingOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(mut, signer)]
    pub relayer: AccountInfo<'info>,

    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump,
        constraint = vizing_pad_config.trusted_relayers.contains(&relayer.key()) @VizingError::NotRelayer, 
        constraint = vizing_pad_config.is_paused != true @VizingError::VizingNotActivated
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    /// CHECK: We need this PDA as a signer
    #[account(seeds = [VIZING_AUTHORITY_SEED, vizing_pad_config.key().as_ref()],bump = vizing_authority.bump)]
    pub vizing_authority: Account<'info, VizingAuthorityParams>,

    /// CHECK: target contract
    #[account(mut, address = params.message.target_program @VizingError::TargetContractInvalid)]
    pub target_program: AccountInfo<'info>,

    #[account(
        seeds = [VIZING_APP_CONFIG_SEED, &vizing_app_configs.vizing_app_program_id.to_bytes()],
        bump = vizing_app_configs.bump,
        constraint = vizing_app_configs.vizing_app_program_id == target_program.key() @VizingError::TargetContractInvalid
    )]
    pub vizing_app_configs: Option<Account<'info, VizingAppConfig>>,

    pub system_program: Program<'info, System>,
}

impl LandingOp<'_> {
    #[access_control(landing_check(&ctx))]
    pub fn vizing_landing<'info>(ctx: &mut Context<'_, '_, '_, 'info, LandingOp<'info>>, params: LandingParams) -> Result<()> {
        let balance_before = ctx.accounts.relayer.lamports();
        let mut target = ctx.accounts.target_program.to_account_info();
        if target.executable {
            let account_info = ctx
            .remaining_accounts
            .iter()
            .map(|acc| {
                let mut account = acc.to_account_metas(None)[0].clone();
                account.is_signer = account.pubkey == ctx.accounts.vizing_authority.key();
                account
            })
            .collect::<Vec<_>>();

            if params.value > 0 {
                target = ctx.remaining_accounts[1].to_account_info();
                transfer(
                    CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        Transfer {
                            from: ctx.accounts.relayer.to_account_info(),
                            to: target
                        },
                    ),
                    params.value,
                )?;
            }

            let ix = Instruction {
                program_id: ctx.accounts.target_program.key(),
                accounts: account_info,
                data: build_landing_ix_data(&params)?,
            };
    
            invoke_signed(
                &ix,
                &[ctx.remaining_accounts].concat(),
                &[&[VIZING_AUTHORITY_SEED, ctx.accounts.vizing_pad_config.key().as_ref(), &[ctx.accounts.vizing_authority.bump]]],
            )
            .map_err(|_| VizingError::CallingFailed)?;

        }else{
            if params.value > 0 {
                transfer(
                    CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        Transfer {
                            from: ctx.accounts.relayer.to_account_info(),
                            to: target
                        },
                    ),
                    params.value,
                )?;
            }
        }

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

fn landing_check(ctx: &Context<LandingOp>) -> Result<()> {
    if let Some(config) = ctx.accounts.vizing_app_configs.clone() {
        let vizing_apps = config.vizing_app_accounts.clone();
        let remaining_accounts = ctx.remaining_accounts.iter().map(|a| a.key).collect::<Vec<_>>();
        for app in vizing_apps {
            if !remaining_accounts.contains(&&app) {
                return Err(VizingError::VizingAppNotInRemainingAccounts.into());
            }
        }
    }
    Ok(())
}

fn build_landing_ix_data(params: &LandingParams) -> Result<Vec<u8>> {
    let vizing_message = VizingMessage {
        message_id: params.message_id,
        src_chainid: params.src_chainid,
        src_contract: params.src_contract,
        value: params.value,
        signature: params.message.signature.clone(),
    };
    let mut data = Vec::with_capacity(VizingMessage::INIT_SPACE);
    data.extend(RECEIVE_FROM_VIZING_DISCRIMINATOR);
    vizing_message.serialize(&mut data)?;
    Ok(data)
}



#[account]
#[derive(InitSpace)]
pub struct LandingParams {
    pub message_id: [u8; 32],
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub src_chainid: u64,
    pub src_tx_hash: [u8; 32],
    pub src_contract: [u8; 32],
    pub src_chain_nonce: u32,
    pub sender: [u8; 32],
    pub value: u64,
    pub addition_params: AdditionalParams,
    pub message: LandingMessage,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LandingMessage {
    pub mode: u8,
    pub target_program: Pubkey,
    pub execute_gas_limit: u32,
    pub max_fee_per_gas: u64,
    #[max_len(1024)]
    pub signature: Vec<u8>,
}


#[account]
#[derive(InitSpace)]
pub struct CurrentRecordMessage {
    pub compute_trade_fee1: Uint256,
    pub compute_trade_fee2: Uint256,
    pub estimate_price1: u64,
    pub estimate_price2: u64,
    pub estimate_gas: u64,
    pub estimate_total_fee: u64,
    pub exact_output: Uint256,
    pub exact_input: Uint256,
    pub estimate_vizing_gas_fee: u64,
    pub init_state: bool
}

#[derive(Accounts)]
pub struct InitCurrentRecordMessage<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + CurrentRecordMessage::INIT_SPACE,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
impl InitCurrentRecordMessage<'_> {
    pub fn init_current_record_message(
        ctx: Context<InitCurrentRecordMessage>
    ) ->Result<()> {
        let current_record_message = &mut ctx.accounts.current_record_message;
        current_record_message.init_state = true;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct ComputeTradeFee1<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl ComputeTradeFee1<'_> {
    pub fn get_compute_trade_fee1(
        ctx: Context<ComputeTradeFee1>,
        dest_chain_id: u64,
        amount_out: Uint256,
    ) -> Result<Uint256> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let current_record_message = &mut ctx.accounts.current_record_message;
        let gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let fee = vizing_gas_system::compute_trade_fee1(
            trade_fee.molecular,
            trade_fee.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            dest_chain_id,
            amount_out,
        ).ok_or(errors::ErrorCode::ComputeTradeFee1NotFound)?;
        current_record_message.compute_trade_fee1=fee;
        //set return fee
        let mut return_data = Vec::new();
        return_data.extend_from_slice(&fee.high.to_le_bytes()); 
        return_data.extend_from_slice(&fee.low.to_le_bytes()); 

        set_return_data(&return_data);
        Ok(fee)
    }
}

#[derive(Accounts)]
pub struct ComputeTradeFee2<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl ComputeTradeFee2<'_> {
    pub fn get_compute_trade_fee2(
        ctx: Context<ComputeTradeFee2>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
        amount_out: Uint256,
    ) -> Result<Uint256> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id,target_contract);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let current_record_message = &mut ctx.accounts.current_record_message;
        let fee = vizing_gas_system::compute_trade_fee2(
            trade_fee.molecular,
            trade_fee.denominator,
            trade_fee_config.molecular,
            trade_fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            target_contract,
            dest_chain_id,
            amount_out,
        ).ok_or(errors::ErrorCode::ComputeTradeFee2NotFound)?;
        current_record_message.compute_trade_fee2=fee;
        //set return fee
        let mut return_data = Vec::new();
        return_data.extend_from_slice(&fee.high.to_le_bytes()); 
        return_data.extend_from_slice(&fee.low.to_le_bytes()); 

        set_return_data(&return_data);
        Ok(fee)
    }
}

#[derive(Accounts)]
pub struct EstimatePrice1<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl EstimatePrice1<'_> {
    pub fn get_estimate_price1(
        ctx: Context<EstimatePrice1>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
    ) -> Result<u64> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id,target_contract);
        let fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let current_record_message = &mut ctx.accounts.current_record_message;
        let dapp_config_value=trade_fee_config.value;

        let dapp_base_price: u64 = vizing_gas_system::estimate_price1(
            fee_config.base_price,
            gas_system_global.global_base_price,
            dapp_config_value,
            target_contract,
            dest_chain_id,
        ).ok_or(errors::ErrorCode::EstimatePrice2NotFound)?;
        current_record_message.estimate_price1=dapp_base_price;
        //set return dapp_base_price

        set_return_data(&dapp_base_price.to_le_bytes());
        Ok(dapp_base_price)
    }
}

#[derive(Accounts)]
pub struct EstimatePrice2<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl EstimatePrice2<'_> {
    pub fn get_estimate_price2(
        ctx: Context<EstimatePrice2>,
        dest_chain_id: u64,
    ) -> Result<u64> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let current_record_message = &mut ctx.accounts.current_record_message;

        let base_price: u64 = vizing_gas_system::estimate_price2(
            gas_system_global.global_base_price,
            fee_config.base_price,
            dest_chain_id,
        ).ok_or(errors::ErrorCode::EstimatePrice2NotFound)?;
        current_record_message.estimate_price2=base_price;
        //set return base_price

        set_return_data(&base_price.to_le_bytes());
        Ok(base_price)
    }
}

#[derive(Accounts)]
pub struct EstimateGas<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl EstimateGas<'_> {
    pub fn get_estimate_gas(
        ctx: Context<EstimateGas>,
        amount_out: Uint256,
        dest_chain_id: u64,
        message: Message,
    ) -> Result<u64> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let current_record_message = &mut ctx.accounts.current_record_message;
        let gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);

        let serialized_data: Vec<u8> = message.try_to_vec()?;
        let Some((_, dapp, _, _, _))=message_monitor::slice_message(&serialized_data) else { todo!() };

        let fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id,dapp);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let dapp_config_value = trade_fee_config.value;

        let fee = vizing_gas_system::estimate_gas(
            gas_system_global.global_base_price,
            fee_config.base_price,
            dapp_config_value,
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            fee_config.molecular,
            fee_config.denominator,
            trade_fee.molecular,
            trade_fee.denominator,
            trade_fee_config.molecular,
            trade_fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            gas_system_global.default_gas_limit,
            amount_out,
            dest_chain_id,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateGasNotFound)?;
        current_record_message.estimate_gas=fee;
        //set return fee
        set_return_data(&fee.to_le_bytes());
        Ok(fee)
    }
}

#[derive(Accounts)]
pub struct EstimateTotalFee<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl EstimateTotalFee<'_> {
    pub fn get_estimate_total_fee(
        ctx: Context<EstimateTotalFee>,
        dest_chain_id: u64,
        amount_out: Uint256,
        message: Message,
    ) -> Result<u64> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let current_record_message = &mut ctx.accounts.current_record_message;

        let serialized_data: Vec<u8> = message.try_to_vec()?;
        let Some((_, dapp, _, _, _))=message_monitor::slice_message(&serialized_data) else { todo!() };

        let gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id,dapp);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let dapp_config_value = trade_fee_config.value;

        let fee = vizing_gas_system::estimate_total_fee(
            gas_system_global.amount_in_threshold,
            trade_fee.molecular,
            trade_fee.denominator,
            trade_fee_config.molecular,
            trade_fee_config.denominator,
            gas_system_global.molecular,
            gas_system_global.denominator,
            dapp_config_value,
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            fee_config.molecular,
            fee_config.denominator,
            gas_system_global.default_gas_limit,
            gas_system_global.global_base_price,
            fee_config.base_price,
            dest_chain_id,
            amount_out,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateTotalFeeNotFound)?;
        current_record_message.estimate_total_fee=fee;
        //set return fee
        set_return_data(&fee.to_le_bytes());
        Ok(fee)
    }
}

#[derive(Accounts)]
pub struct ExactOutput<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl ExactOutput<'_> {
    pub fn get_exact_output(
        ctx: Context<ExactOutput>,
        dest_chain_id: u64,
        amount_out: Uint256,
    ) -> Result<Uint256> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let current_record_message = &mut ctx.accounts.current_record_message;

        let amount_in = vizing_gas_system::exact_output(
            fee_config.molecular,
            fee_config.denominator,
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            dest_chain_id,
            amount_out,
        ).ok_or(errors::ErrorCode::ExactOutputNotFound)?;
        current_record_message.exact_output=amount_in;
        //set return amount_in
        let mut return_data = Vec::new();
        return_data.extend_from_slice(&amount_in.high.to_le_bytes()); 
        return_data.extend_from_slice(&amount_in.low.to_le_bytes()); 
        set_return_data(&return_data);
        Ok(amount_in)
    }
}

#[derive(Accounts)]
pub struct ExactInput<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl ExactInput<'_> {
    pub fn get_exact_input(
        ctx: Context<ExactInput>,
        dest_chain_id: u64,
        amount_in: u64,
    ) -> Result<Uint256> {
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let current_record_message = &mut ctx.accounts.current_record_message;

        let amount_out = vizing_gas_system::exact_input(
            fee_config.molecular,
            fee_config.denominator,
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
            dest_chain_id,
            amount_in,
        ).ok_or(errors::ErrorCode::ExactInputputNotFound)?;
        current_record_message.exact_input=amount_out;
        //set return amount_out
        let mut return_data = Vec::new();
        return_data.extend_from_slice(&amount_out.high.to_le_bytes()); 
        return_data.extend_from_slice(&amount_out.low.to_le_bytes()); 

        set_return_data(&return_data);
        Ok(amount_out)
    }
}

#[derive(Accounts)]
pub struct EstimateVizingGasFee1<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}
impl EstimateVizingGasFee1<'_>{
    pub fn get_estimate_vizing_gas_fee (
        ctx: Context<EstimateVizingGasFee1>,
        value: Uint256,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message:Vec<u8>
    ) -> Result<u64> {  

        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let current_record_message = &mut ctx.accounts.current_record_message;
        msg!("message: {:?}",message);
        let Some((_, dapp, _, _, _))=message_monitor::slice_message(&message) else { todo!() };
        
        let get_gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let get_fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let get_trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id, dapp);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let dapp_config_value = get_trade_fee_config.value;
        
        let vizing_gas_fee = vizing_gas_system::estimate_gas(
            get_gas_system_global.global_base_price,
            get_fee_config.base_price,
            dapp_config_value,
            get_fee_config.molecular_decimal,
            get_fee_config.denominator_decimal,
            get_fee_config.molecular,
            get_fee_config.denominator,
            trade_fee.molecular,
            trade_fee.denominator,
            get_trade_fee_config.molecular,
            get_trade_fee_config.denominator,
            get_gas_system_global.molecular,
            get_gas_system_global.denominator,
            get_gas_system_global.default_gas_limit,
            value,
            dest_chain_id,
            &message,
        ).ok_or(errors::ErrorCode::EstimateGasNotFound)?;
        current_record_message.estimate_vizing_gas_fee=vizing_gas_fee;
        //set return vizing_gas_fee
        set_return_data(&vizing_gas_fee.to_le_bytes());
        Ok(vizing_gas_fee)
    }
}

#[derive(Accounts)]
pub struct EstimateVizingGasFee<'info> {
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
}

impl EstimateVizingGasFee<'_>{
    pub fn get_estimate_vizing_gas_fee (
        ctx: Context<EstimateVizingGasFee>,
        value: Uint256,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Message
    ) -> Result<u64> {  

        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;


        let serialized_data: Vec<u8> = message.try_to_vec()?;
        let Some((_, dapp, _, _, _))=message_monitor::slice_message(&serialized_data) else { todo!() };

        let get_gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let get_fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let get_trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id, dapp);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let dapp_config_value = get_trade_fee_config.value;
        
        let vizing_gas_fee = vizing_gas_system::estimate_gas(
            get_gas_system_global.global_base_price,
            get_fee_config.base_price,
            dapp_config_value,
            get_fee_config.molecular_decimal,
            get_fee_config.denominator_decimal,
            get_fee_config.molecular,
            get_fee_config.denominator,
            trade_fee.molecular,
            trade_fee.denominator,
            get_trade_fee_config.molecular,
            get_trade_fee_config.denominator,
            get_gas_system_global.molecular,
            get_gas_system_global.denominator,
            get_gas_system_global.default_gas_limit,
            value,
            dest_chain_id,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateGasNotFound)?;

        Ok(vizing_gas_fee)
    }
    
}

#[derive(Accounts)]
pub struct EstimateVizingGasFee2<'info> {
    pub vizing_gas_system: Account<'info, VizingGasSystem>,

    #[account(
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Option<Account<'info, CurrentRecordMessage>>,
}

impl EstimateVizingGasFee2<'_>{
    pub fn get_estimate_vizing_gas_fee (
        ctx: Context<EstimateVizingGasFee2>,
        value: Uint256,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Message
    ) -> Result<u64> {  

        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;


        let serialized_data: Vec<u8> = message.try_to_vec()?;
        let Some((_, dapp, _, _, _))=message_monitor::slice_message(&serialized_data) else { todo!() };

        msg!("dapp: {:?}",dapp);
        let get_gas_system_global = vizing_gas_system.get_gas_system_global(dest_chain_id);
        let get_fee_config = vizing_gas_system.get_fee_config(dest_chain_id);
        let get_trade_fee_config = vizing_gas_system.get_trade_fee_config(dest_chain_id, dapp);
        let trade_fee = vizing_gas_system.get_trade_fee(dest_chain_id);
        let dapp_config_value = get_trade_fee_config.value;
        
        let vizing_gas_fee = vizing_gas_system::estimate_gas(
            get_gas_system_global.global_base_price,
            get_fee_config.base_price,
            dapp_config_value,
            get_fee_config.molecular_decimal,
            get_fee_config.denominator_decimal,
            get_fee_config.molecular,
            get_fee_config.denominator,
            trade_fee.molecular,
            trade_fee.denominator,
            get_trade_fee_config.molecular,
            get_trade_fee_config.denominator,
            get_gas_system_global.molecular,
            get_gas_system_global.denominator,
            get_gas_system_global.default_gas_limit,
            value,
            dest_chain_id,
            &serialized_data,
        ).ok_or(errors::ErrorCode::EstimateGasNotFound)?;

        if let Some(current_record_message) = &mut ctx.accounts.current_record_message {
            current_record_message.estimate_vizing_gas_fee=vizing_gas_fee;
        }
        //set return vizing_gas_fee
        set_return_data(&vizing_gas_fee.to_le_bytes());
        Ok(vizing_gas_fee)
    }
    
}








