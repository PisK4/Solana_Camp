pub mod governance;
pub mod library;
pub mod vizing_channel;
pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_channel::*;
use vizing_omni::*;
use library::Uint256;

declare_id!("vizngM8xTgmP15xuxpUZHbdec3LBG7bnTe9j1BtaqsE");

#[program]
pub mod vizing_pad {

    use super::*;

    // **********  channel start ************

    pub fn launch(mut _ctx: Context<LaunchOp>, _params: LaunchParams) -> Result<()> {
        Ok(())
    }

    // **********  channel end ************

    // **********  get output data ***********
    pub fn compute_trade_fee1(
        mut _ctx: Context<ComputeTradeFee1>, 
        dest_chain_id: u64,
        amount_out: Uint256
    )-> Result<(Uint256)> {
        Ok(())
    }

    pub fn compute_trade_fee2(
        mut _ctx: Context<ComputeTradeFee2>, 
        target_contract: [u8; 32],
        dest_chain_id: u64,
        amount_out: Uint256
    )-> Result<(Uint256)> {
        Ok(())
    }

    pub fn estimate_price1(
        mut _ctx: Context<EstimatePrice1>, 
        target_contract: [u8; 32],
        dest_chain_id: u64,
    )-> Result<(u64)> {
        Ok(())
    }

    pub fn estimate_price2(
        mut _ctx: Context<EstimatePrice2>, 
        dest_chain_id: u64,
    )-> Result<(u64)> {
        Ok(())
    }

    pub fn estimate_gas(
        mut _ctx: Context<EstimateGas>, 
        amount_out: Uint256,
        dest_chain_id: u64,
        message: Message,
    )-> Result<(u64)> {
        Ok(())
    }

    pub fn estimate_total_fee(
        mut _ctx: Context<EstimateTotalFee>, 
        dest_chain_id: u64,
        amount_out: Uint256,
        message: Message,
    )-> Result<(u64)> {
        Ok(())
    }

    pub fn exact_output(
        mut _ctx: Context<ExactOutput>,
        dest_chain_id: u64,
        amount_out: Uint256,
    )-> Result<(Uint256)> {
        Ok(())
    }

    pub fn exact_input(
        mut _ctx: Context<ExactInput>,
        dest_chain_id: u64,
        amount_in: Uint256,
    )-> Result<(Uint256)> {
        Ok(())
    }

    pub fn estimate_vizing_gas_fee1(
        mut _ctx: Context<EstimateVizingGasFee1>,
        value: Uint256,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Vec<u8>
    )-> Result<(u64)> {
        Ok(())
    }

    pub fn estimate_vizing_gas_fee2(
        mut _ctx: Context<EstimateVizingGasFee2>,
        value: Uint256,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Message
    )-> Result<(u64)> {
        Ok(())
    }

}
