pub mod governance;
pub mod library;
pub mod vizing_channel;
pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_channel::*;
use vizing_omni::*;

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
        _dest_chain_id: u64,
        _amount_out: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn compute_trade_fee2(
        mut _ctx: Context<ComputeTradeFee2>,
        _target_contract: [u8; 32],
        _dest_chain_id: u64,
        _amount_out: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn estimate_price1(
        mut _ctx: Context<EstimatePrice1>,
        _target_contract: [u8; 32],
        _dest_chain_id: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn estimate_price2(mut _ctx: Context<EstimatePrice2>, _dest_chain_id: u64) -> Result<()> {
        Ok(())
    }

    pub fn estimate_gas(
        mut _ctx: Context<EstimateGas>,
        _amount_out: u64,
        _dest_chain_id: u64,
        _message: Message,
    ) -> Result<()> {
        Ok(())
    }

    pub fn estimate_total_fee(
        mut _ctx: Context<EstimateTotalFee>,
        _dest_chain_id: u64,
        _amount_out: u64,
        _message: Message,
    ) -> Result<()> {
        Ok(())
    }

    pub fn exact_output(
        mut _ctx: Context<ExactOutput>,
        _dest_chain_id: u64,
        _amount_out: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn exact_input(
        mut _ctx: Context<ExactInput>,
        _dest_chain_id: u64,
        _amount_in: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn estimate_vizing_gas_fee1(
        mut _ctx: Context<EstimateVizingGasFee1>,
        _value: u64,
        _dest_chain_id: u64,
        _addition_params: Vec<u8>,
        _message: Vec<u8>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn estimate_vizing_gas_fee2(
        mut _ctx: Context<EstimateVizingGasFee2>,
        _value: u64,
        _dest_chain_id: u64,
        _addition_params: Vec<u8>,
        _message: Message,
    ) -> Result<()> {
        Ok(())
    }
}
