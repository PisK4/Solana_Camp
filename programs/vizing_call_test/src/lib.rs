use anchor_lang::prelude::*;
use solana_program::program::{invoke, get_return_data};
use vizing_core::{
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

use vizing_core::program::VizingCore;

declare_id!("D4RmkYAYx96Uj9Pej9eCEvMp2n1d5NwABz4Efhj6cXMv");

#[program]
pub mod vizing_call_test {

    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()>{
        let record_message = &mut ctx.accounts.record_message;
        record_message.get_record_number = 0;
        Ok(())
    }

    //get
    /*
    /// @notice Calculate the fee for the native token transfer
    /// @param destChainid The chain id of the destination chain
    /// @param amountOut The value we spent in the source chain
     */
    pub fn get_compute_trade_fee1(
        ctx: Context<GetComputeTradeFee1>,
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            ComputeTradeFee1 {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        compute_trade_fee1(cpi_ctx, dest_chain_id, amount_out);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice Calculate the fee for the native token transfer
    /// @param targetContract contract address in the destination chain
    /// @param destChainid The chain id of the destination chain
    /// @param amountOut The value we spent in the source chain
     */
    pub fn get_compute_trade_fee2(
        ctx: Context<GetComputeTradeFee2>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            ComputeTradeFee2 {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        compute_trade_fee2(cpi_ctx, target_contract, dest_chain_id, amount_out);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice Estimate the gas price we need to encode in message
    /// @param targetContract evm address
    /// @param destChainid The chain id of the destination chain
     */
    pub fn get_estimate_price1(
        ctx: Context<GetEstimatePrice1>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            EstimatePrice1 {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        estimate_price1(cpi_ctx, target_contract, dest_chain_id);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice Estimate the gas price we need to encode in message
    /// @param destChainid The chain id of the destination chain
     */
    pub fn get_estimate_price2(ctx: Context<GetEstimatePrice2>, dest_chain_id: u64) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            EstimatePrice2 {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        estimate_price2(cpi_ctx, dest_chain_id);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice Estimate the gas fee we should pay to vizing
    /// @param amountOut amountOut in the destination chain
    /// @param destChainid The chain id of the destination chain
    /// @param message The message we want to send to the destination chain
     */
    pub fn get_estimate_gas(
        ctx: Context<GetEstimateGas>,
        amount_out: u64,
        dest_chain_id: u64,
        message: Message,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            EstimateGas {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        estimate_gas(cpi_ctx, amount_out, dest_chain_id, message);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice Estimate the total fee we should pay to vizing
    /// @param amountOut amountOut in the destination chain
    /// @param destChainid The chain id of the destination chain
    /// @param message The message we want to send to the destination chain
     */
    pub fn get_estimate_total_fee(
        ctx: Context<GetEstimateTotalFee>,
        dest_chain_id: u64,
        amount_out: u64,
        message: Message,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            EstimateTotalFee {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        estimate_total_fee(cpi_ctx, dest_chain_id, amount_out, message);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice similar to uniswap Swap Router
    /// @notice Estimate how many native token we should spend to exchange the amountOut in the destChainid
    /// @param destChainid The chain id of the destination chain
    /// @param amountOut The value we want to receive in the destination chain
     */
    pub fn get_exact_output(
        ctx: Context<GetExactOutput>,
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            ExactOutput {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        exact_output(cpi_ctx, dest_chain_id, amount_out);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice similar to uniswap Swap Router
    /// @notice Estimate how many native token we could get in the destChainid if we input the amountIn
    /// @param destChainid The chain id of the destination chain
    /// @param amountIn The value we spent in the source chain
     */
    pub fn get_exact_input(
        ctx: Context<GetExactInput>,
        dest_chain_id: u64,
        amount_in: u64,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            ExactInput {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        exact_input(cpi_ctx, dest_chain_id, amount_in);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    /*
    /// @notice Estimate the gas price we need to encode in message
    /// @param value The native token that value target address will receive in the destination chain
    /// @param destChainid The chain id of the destination chain
    /// @param additionParams The addition params for the message
    ///        if not in expert mode, set to 0 (`new bytes(0)`)
    /// @param message The message we want to send to the destination chain
     */

    pub fn get_estimate_vizing_gas_fee1(
        ctx: Context<GetEstimateVizingGasFee1>,
        value: u64,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Vec<u8>,
    ) -> Result<()> {
        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            EstimateVizingGasFee1 {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        estimate_vizing_gas_fee1(cpi_ctx, value, dest_chain_id, _addition_params, message);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
    }

    pub fn get_estimate_vizing_gas_fee2(
        ctx: Context<GetEstimateVizingGasFee2>,
        value: u64,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Message,
    ) -> Result<()> {

        let record_message = &mut ctx.accounts.record_message;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.vizing_pad_program.clone(), // Ensure this is a clone
            EstimateVizingGasFee2 {
                mapping_fee_config: ctx.accounts.mapping_fee_config.clone(), // Ensure this is a clone
                current_record_message: ctx.accounts.current_record_message.clone(), // Ensure this is a clone
            },
        );

        estimate_vizing_gas_fee2(cpi_ctx, value, dest_chain_id, _addition_params, message);

        if let Some((_, return_data)) = get_return_data() {
            let returned_vizing_gas_fee = u64::from_le_bytes(
                return_data.try_into().expect("Failed to parse return data")
            );
            msg!("Returned vizing gas fee: {}", returned_vizing_gas_fee);
            record_message.get_record_number = returned_vizing_gas_fee;
        } else {
            err!(AppErrors::VizingCallFailed);
        }

        Ok(())
        
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + TestRecordAccount::INIT_SPACE
    )]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetComputeTradeFee1<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetComputeTradeFee2<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetEstimatePrice1<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetEstimatePrice2<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetEstimateGas<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetEstimateTotalFee<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct GetExactOutput<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetExactInput<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct GetEstimateVizingGasFee1<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetEstimateVizingGasFee2<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"init_current_record_message".as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
    #[account(mut)]
    pub record_message: Account<'info, TestRecordAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub vizing_pad_program: Program<'info, VizingCore>,
    pub system_program: Program<'info, System>,
}



#[account]
#[derive(InitSpace)]
pub struct TestRecordAccount {
    pub get_record_number: u64,
}

#[error_code]
pub enum AppErrors {
    #[msg("vizing call failed")]
    VizingCallFailed,
}
