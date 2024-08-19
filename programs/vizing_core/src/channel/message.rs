use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LaunchOp<'info> {
    #[account(signer)]
    pub fee_payer: AccountInfo<'info>,
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub message_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl LaunchOp<'_> {
    pub fn execute(ctx: &mut Context<LaunchOp>, params: LaunchParams) -> Result<()> {
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

        msg!("target_contract: {}", params.message.target_contract);

        msg!("execute_gas_limit: {}", params.message.execute_gas_limit);

        msg!("max_fee_per_gas: {}", params.message.max_fee_per_gas);

        msg!("signature: {:?}", params.message.signature);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct LaunchParams {
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub relayer: Pubkey,
    pub sender: Pubkey,
    pub value: u64,
    pub dest_chainid: u64,
    #[max_len(256)]
    pub addition_params: Vec<u8>,
    pub message: Message,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Message {
    pub mode: u8,
    pub target_contract: Pubkey,
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(256)]
    pub signature: Vec<u8>,
}
