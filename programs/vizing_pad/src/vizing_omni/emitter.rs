use crate::library::*;
use crate::governance::*;
use anchor_lang::prelude::*;

pub const VIZING_MESSAGE_AUTHORITY_SEED: &[u8] = b"Vizing_Message_Authority_Seed";
pub const VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_RELAYER_DEFAULT: Pubkey = Pubkey::new_from_array([
    137, 71, 54, 167, 199, 236, 39, 80, 113, 216, 76, 7, 85, 39, 112, 180, 125, 214, 156, 170, 202,
    74, 57, 119, 4, 40, 1, 88, 236, 158, 120, 105,
]);
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
        let (_, message_pda_authority_bump) =
            Pubkey::find_program_address(&[b"message_pda_authority"], ctx.program_id);
        ctx.accounts.message_pda_authority.bump = message_pda_authority_bump;
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
    pub value: Uint256,
    pub dest_chainid: u64,
    pub addition_params: AdditionalParams,
    pub message: Message,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Message {
    pub mode: u8,
    pub target_contract: [u8; 32],
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

//get 
//48 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone ,InitSpace)]
pub struct GasSystemGlobal {
    pub key: u64,
    pub global_base_price: u64,
    pub default_gas_limit: u64,
    pub amount_in_threshold: u64,
    pub molecular: u64,
    pub denominator: u64,
}

//24 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone ,InitSpace)]
pub struct NativeTokenTradeFeeConfig {
    pub key: u64,
    pub molecular: u64,
    pub denominator: u64,
}

//42 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone ,InitSpace)]
pub struct FeeConfig {
    pub key: u64,
    pub base_price: u64,
    pub reserve: u64,
    pub molecular: u64,
    pub denominator: u64,
    pub molecular_decimal: u8,
    pub denominator_decimal: u8,
}
//24 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TradeFee {
    pub key: u64,
    pub molecular: u64,
    pub denominator: u64,
}

//352 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TradeFeeAndDappConfig {
    pub key: u64,
    #[max_len(10)]
    pub dapps: Vec<[u8; 32]>, //address group
    pub molecular: u64,
    pub denominator: u64,
    pub value: u64,
}

#[account]
#[derive(InitSpace)]
pub struct MappingFeeConfig {
    #[max_len(20)]
    pub gas_system_global_mappings: Vec<GasSystemGlobal>,
    #[max_len(20)]
    pub fee_config_mappings: Vec<FeeConfig>,
    #[max_len(20)]
    pub trade_fee_mappings: Vec<TradeFee>,
    #[max_len(20)]
    pub trade_fee_config_mappings: Vec<TradeFeeAndDappConfig>,
    #[max_len(20)]
    pub native_token_trade_fee_config_mappings: Vec<NativeTokenTradeFeeConfig>,
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
pub struct ComputeTradeFee1<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct ComputeTradeFee2<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct EstimatePrice1<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct EstimatePrice2<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct EstimateGas<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct EstimateTotalFee<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct ExactOutput<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct ExactInput<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct EstimateVizingGasFee1<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}

#[derive(Accounts)]
pub struct EstimateVizingGasFee2<'info> {
    #[account(
        seeds = [contants::VIZING_PAD_CONFIG_SEED], 
        bump = vizing_pad_config.bump
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [contants::VIZING_RECORD_SEED.as_ref()],
        bump
    )]
    pub current_record_message: Account<'info, CurrentRecordMessage>,
}




