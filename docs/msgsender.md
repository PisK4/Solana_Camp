在 Solana 中，跨程序调用（CPI）允许一个智能合约（源程序）调用另一个智能合约（目标程序）。在目标程序中，有时需要知道调用它的源程序的地址。Solana 的系统账户（Sysvar）提供了一种机制，可以在目标程序中获取发起调用的源程序地址。

使用 Sysvar 来获取源程序地址
Solana 提供了一个叫做 SysvarC1 的系统账户，可以用来获取调用 CPI 的程序的地址。在 Anchor 框架中，可以通过在程序上下文中包含这个系统账户来实现这一点。

示例：获取 CPI 调用的源程序地址
下面是一个示例，展示如何在目标程序中获取源程序的地址。

1. 配置目标程序
在 programs/target_program/src/lib.rs 中编写以下代码：

rust
复制代码
use anchor_lang::prelude::*;

declare_id!("TargetProgramIDHere");

#[program]
mod target_program {
    use super::*;

    pub fn receive_cpi(ctx: Context<ReceiveCpi>) -> Result<()> {
        let source_program_id = ctx.accounts.cpi_program.key();
        msg!("CPI called from program: {}", source_program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReceiveCpi<'info> {
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: AccountInfo<'info>,
    #[account(signer)]
    pub cpi_program: AccountInfo<'info>,
}
2. 调整 Anchor.toml
确保你的 Anchor.toml 文件中包含正确的依赖：

toml
复制代码
[dependencies]
anchor-lang = "0.18.0"
3. 编译和部署目标程序
编译并部署目标程序：

bash
复制代码
anchor build
anchor deploy
4. 源程序调用目标程序
在源程序中调用目标程序的 CPI 方法。

在 programs/source_program/src/lib.rs 中编写以下代码：

rust
复制代码
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};

declare_id!("SourceProgramIDHere");

#[program]
mod source_program {
    use super::*;

    pub fn call_cpi(ctx: Context<CallCpi>) -> Result<()> {
        let cpi_program_id = ctx.accounts.target_program.key();
        let instructions = anchor_lang::solana_program::sysvar::instructions::ID;
        let accounts = vec![
            AccountMeta::new_readonly(cpi_program_id, false),
            AccountMeta::new_readonly(instructions, false),
        ];

        let ix = Instruction {
            program_id: cpi_program_id,
            accounts,
            data: vec![0], // 调用目标程序中的 `receive_cpi` 方法
        };

        let account_infos = vec![
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.target_program.to_account_info(),
            ctx.accounts.instructions.to_account_info(),
        ];

        anchor_lang::solana_program::program::invoke(
            &ix,
            &account_infos,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CallCpi<'info> {
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    #[account(address = target_program::ID)]
    pub target_program: AccountInfo<'info>,
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: AccountInfo<'info>,
}
5. 编译和部署源程序
编译并部署源程序：

bash
复制代码
anchor build
anchor deploy
解释
目标程序：

ReceiveCpi 上下文结构中包含 instructions 和 cpi_program。
instructions 是 SysvarC1 系统账户，用于获取 CPI 调用的源程序的地址。
在 receive_cpi 方法中，使用 ctx.accounts.cpi_program.key() 获取源程序地址并打印。
源程序：

CallCpi 上下文结构中包含 target_program 和 instructions。
使用 anchor_lang::solana_program::program::invoke 方法调用目标程序的 CPI 方法。
CPI 调用：

在源程序中构造 CPI 调用目标程序的指令，并传递所需的账户信息。
总结
通过使用 SysvarC1 系统账户，可以在目标程序中获取发起 CPI 调用的源程序地址。在 Anchor 框架中，通过在上下文结构中包含相应的账户信息，可以实现这一功能。这使得程序之间的交互更加灵活和强大，有助于构建复杂的应用场景。