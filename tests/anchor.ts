import * as anchor from "@project-serum/anchor";
import * as borsh from "borsh";

import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  mintTo,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  transfer,
} from "@solana/spl-token";

import { clusterApiUrl, Connection } from "@solana/web3.js";

import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";
import {
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { web3 } from "@project-serum/anchor";

import assert from "assert";

const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
let token_programId = TOKEN_PROGRAM_ID;
let associated_token_programId = ASSOCIATED_TOKEN_PROGRAM_ID;

let user = pg.wallet.publicKey;
let signer = pg.wallet.keypair;

let newTokenMes = new web3.Keypair();

let testReceiver = new web3.Keypair();
console.log("testReceiver:", testReceiver.publicKey.toBase58());

const metadataData = {
  name: "Jump Sea Token",
  symbol: "JST",
  uri: "https://arweave.net/1234",
  sellerFeeBasisPoints: 0,
  creators: null,
  collection: null,
  uses: null,
};

function padStringTo32Bytes(str: string): Buffer {
  const buffer = Buffer.alloc(32);
  buffer.write(str);
  return buffer;
}

const vizingPadSettingsSeed = Buffer.from("Vizing_Pad_Settings_Seed");
const vizingAuthoritySeed = Buffer.from("Vizing_Authority_Seed");
const feeReceiverKeyPair = anchor.web3.Keypair.fromSeed(
  Buffer.from(padStringTo32Bytes("fee_receiver"))
);

const engineAdminKeyPairs = [
  anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("engine_admin_1"))
  ),
  anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("engine_admin_2"))
  ),
];

const stationAdminKeyPair = anchor.web3.Keypair.fromSeed(
  Buffer.from(padStringTo32Bytes("station_admim"))
);

const gasPoolAdminKeyPair = anchor.web3.Keypair.fromSeed(
  Buffer.from(padStringTo32Bytes("gas_pool_admin"))
);

const trustedRelayerKeyPairs = [
  anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("trusted_relayer_1"))
  ),
  anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("trusted_relayer_2"))
  ),
];

const registeredValidatorKeyPairs = [
  anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("registered_validator_1"))
  ),
  anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("registered_validator_2"))
  ),
];

// async function requestAirdrop(user: PublicKey) {
//   try {
//     const airdropSignature = await connection.requestAirdrop(
//       user,
//       5 * web3.LAMPORTS_PER_SOL // Requesting 2 SOL
//     );

//     // Confirm the transaction
//     await connection.confirmTransaction(airdropSignature);
//     console.log("Airdrop successful!");
//   } catch (err) {
//     console.error("Airdrop failed:", err);
//   }
// }
// await requestAirdrop(user);

async function getSolBalance(checkAddress: PublicKey) {
  try {
    const balance = await connection.getBalance(checkAddress);
    console.log("sol balance:",balance);
    return balance;
  } catch (err) {
    console.error("Failed to get balance:", err);
  }
}

//createMint
async function create_mint() {
  const tokenMint = await createMint(
    connection,
    signer,
    user,
    null,
    6,
    newTokenMes,
    null,
    token_programId
  );
  console.log("tokenMint:", tokenMint.toString());
}
await create_mint();

//Make some token metadata
// Generate a new keypair for the mint
async function make_metadata() {
  const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const metadataPDAAndBump = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      newTokenMes.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const metadataPDA = metadataPDAAndBump[0];
  console.log("metadataPDA success");
  const transaction = new Transaction();

  const createMetadataAccountInstruction =
    createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataPDA,
        mint: newTokenMes.publicKey,
        mintAuthority: user,
        payer: user,
        updateAuthority: user,
      },
      {
        createMetadataAccountArgsV3: {
          collectionDetails: null,
          data: metadataData,
          isMutable: true,
        },
      }
    );

  transaction.add(createMetadataAccountInstruction);

  // send
  try {
    const metadataTxHash = await pg.connection.sendTransaction(transaction, [
      signer,
    ]);
    console.log(`Transaction sent`);
    // confirm
    const metadataConfirmation = await pg.connection.confirmTransaction(
      metadataTxHash
    );
    console.log(
      `Transaction confirmed: ${metadataTxHash}`,
      metadataConfirmation
    );
  } catch (error) {
    console.error("Error sending transaction:", error);
  }
}
await make_metadata();

let userAssociatedAccount: PublicKey;
let destinationTokenAccount: PublicKey;

async function GetOrcreateAssociatedToken(
  createAssociateAccount: PublicKey,
  newToken: PublicKey,
  this_signer
) {
  try {
    const userAssociatedTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      this_signer,
      newToken,
      createAssociateAccount,
      true,
      null,
      null,
      token_programId,
      associated_token_programId
    );
    return userAssociatedTokenAccount.address;
  } catch (e) {
    console.log("GetOrcreateAssociatedToken:", e);
  }
}
userAssociatedAccount = await GetOrcreateAssociatedToken(
  user,
  newTokenMes.publicKey,
  signer
);
console.log(
  "userAssociatedAccount:",
  userAssociatedAccount.toString(),
  "owner:",
  user.toString()
);

//
destinationTokenAccount = await GetOrcreateAssociatedToken(
  testReceiver.publicKey,
  newTokenMes.publicKey,
  signer
);
console.log(
  "destinationTokenAccountInfo:",
  destinationTokenAccount.toString(),
  "owner:",
  testReceiver.publicKey.toString()
);

//mint to
async function mint_to(newToken: PublicKey, receiver: PublicKey) {
  const init_amount = 10000000000;
  try {
    const mintToTx = await mintTo(
      connection,
      signer,
      newToken,
      receiver,
      user,
      init_amount,
      [signer],
      null,
      token_programId
    );
    console.log("mintToTx:", mintToTx);
  } catch (e) {
    console.log("Mint to error:", e);
  }
}
await mint_to(newTokenMes.publicKey, userAssociatedAccount);

//token balance
async function getTokenBalance(checkAddress: PublicKey) {
  try {
    const tokenAccountInfo = await getAccount(connection, checkAddress);
    let tokenBalance=tokenAccountInfo.amount;
    console.log(`This Account Token balance: ${tokenAccountInfo.amount}`);
    return tokenBalance;
  } catch (err) {
    console.error("Failed to get token balance:", err);
  }
}
await getTokenBalance(userAssociatedAccount);

//Transfer token to user
async function transfer_to(receiverAssociatedAccount: PublicKey) {
  try {
    const TransferTokenSignature = await transfer(
      connection,
      signer,
      userAssociatedAccount,
      receiverAssociatedAccount,
      user,
      10000,
      [signer],
      null,
      token_programId
    );
    console.log("transfer_to:", TransferTokenSignature);
  } catch (e) {
    console.log("Transfer error:", e);
  }
}
await transfer_to(destinationTokenAccount);

describe("Test", () => {
  it("initialize", async () => {
    console.log("current user:", user.toBase58());
    let systemId = web3.SystemProgram.programId;

    let vizingPadSettings: anchor.web3.PublicKey;
    let relayerSettings: anchor.web3.PublicKey;
    let vizingAuthority: anchor.web3.PublicKey;
    let vizingPadBump: number;
    let relayerBump: number;

    const seed = [vizingPadSettingsSeed];
    const [vizingPad, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seed,
      pg.PROGRAM_ID
    );

    vizingPadSettings = vizingPad;
    vizingPadBump = bump;

    console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);

    const initParams = {
      owner: user,
      feeReceiver: feeReceiverKeyPair.publicKey,
      engineAdmin: engineAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: user,
      stationAdmin: stationAdminKeyPair.publicKey,
      trustedRelayers: trustedRelayerKeyPairs.map(
        (keypair) => keypair.publicKey
      ),
      registeredValidator: registeredValidatorKeyPairs.map(
        (keypair) => keypair.publicKey
      )[0],
      relayers: trustedRelayerKeyPairs.map((keypair) => keypair.publicKey),
      isPaused: false,
    };

    {
      const seed = [vizingAuthoritySeed];
      const [authority, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        pg.PROGRAM_ID
      );

      console.log(`authority: ${authority.toBase58()}, bump: ${bump}`);
      vizingAuthority = authority;
    }

    // function encodeEthereumAddressToU8Array(ethAddress: string): number[] {
    //   const address = ethAddress.slice(2); // Remove the '0x' prefix
    //   const result = new Uint8Array(60);
    //   result[0] = 2;
    //   for (let i = 0; i < 40; i++) {
    //     let charAddressI = address[i].charCodeAt(0);
    //     result[i + 1] = charAddressI;
    //   }

    //   for (let i = 41; i < 60; i++) {
    //     result[i] = result[((i - 1) % 10) + 1];
    //   }
    //   const addressArray: number[] = Array.from(result);
    //   return addressArray;
    // }

    // function decodeU8ArrayToEthereumAddress(message) {
    //   const asciiData = message.slice(0, 40); // ASCII数据
    //   let hexAddress = "0x";
    //   for (let i = 0; i < 40; i++) {
    //     const charCode = asciiData[i];
    //     hexAddress += String.fromCharCode(charCode);
    //   }
    //   return hexAddress;
    // }

    function encodeEthereumAddressToU8Array(ethAddress: string): number[] {
      const remove0xAddress = ethAddress.slice(2);
      const address = Buffer.from(remove0xAddress);
      console.log("ethAddress:",ethAddress,"\n","Buffer address:",address);
      console.log("address length:",address.length);
      const result = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        result[i] = address[i];
      }
      const addressArray: number[] = Array.from(result);
      console.log("ethAddress:",ethAddress,"\n","addressArray:",addressArray);
      return addressArray;
    }

    function decodeU8ArrayToEthereumAddress(message) {
      const asciiData = message.slice(0, 32); // ASCII数据
      console.log("asciiData:",asciiData);
      let hexAddress = "0x";
      for (let i = 0; i < 31; i++) {
        const charCode = asciiData[i];
        hexAddress += String.fromCharCode(charCode);
      }
      return hexAddress;
    }

    let id = new anchor.BN(4);
    let chainId = Buffer.from([4]);
    console.log("chainId buffer:", chainId);

    //pda
    let [powerUserAuthority, powerUserBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_power_user")],
        pg.PROGRAM_ID
      );
    console.log("powerUserAuthority:", powerUserAuthority.toString());
    console.log("powerUserBump:", powerUserBump);

    let [vizingVaultAuthority, vizingVaultBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("vizing_vault")],
        pg.PROGRAM_ID
      );
    console.log("vizingVaultAuthority:", vizingVaultAuthority.toString());
    console.log("vizingVaultBump:", vizingVaultBump);

    //gas_global
    let [gasSystemGlobalAuthority, gasSystemGlobalBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("gas_global"), chainId],
        pg.PROGRAM_ID
      );
    console.log(
      "gasSystemGlobalAuthority:",
      gasSystemGlobalAuthority.toString()
    );
    console.log("gasSystemGlobalBump:", gasSystemGlobalBump);

    //init_mapping_fee_config
    let [mappingFeeConfigAuthority, mappingFeeConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_mapping_fee_config"), chainId],
        pg.PROGRAM_ID
      );
    console.log(
      "mappingFeeConfigAuthority:",
      mappingFeeConfigAuthority.toString()
    );
    console.log("mappingFeeConfigBump:", mappingFeeConfigBump);

    //amount_in_thresholds
    let [amountInThresholdsAuthority, amountInThresholdsBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("amount_in_thresholds"), chainId],
        pg.PROGRAM_ID
      );
    console.log(
      "amountInThresholdsAuthority:",
      amountInThresholdsAuthority.toString()
    );
    console.log("amountInThresholdsBump:", amountInThresholdsBump);

    //native_token_trade_fee_config
    let [nativeTokenTradeFeeConfigAuthority, nativeTokenTradeFeeConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("native_token_trade_fee_config"), chainId],
        pg.PROGRAM_ID
      );
    console.log(
      "nativeTokenTradeFeeConfigAuthority:",
      nativeTokenTradeFeeConfigAuthority.toString()
    );
    console.log(
      "nativeTokenTradeFeeConfigBump:",
      nativeTokenTradeFeeConfigBump
    );

    //init_token_config
    let [initTokenConfigAuthority, initTokenConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_token_config"), chainId],
        pg.PROGRAM_ID
      );
    console.log(
      "initTokenConfigAuthority:",
      initTokenConfigAuthority.toString()
    );
    console.log("initTokenConfigBump:", initTokenConfigBump);

    //init_symbol_config
    let [initSymbolConfigAuthority, initSymbolConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_symbol_config"), chainId],
        pg.PROGRAM_ID
      );
    console.log(
      "initSymbolConfigAuthority:",
      initSymbolConfigAuthority.toString()
    );
    console.log("initSymbolConfigBump:", initSymbolConfigBump);

    //save_dest_chain_Id
    let saveDestChainIdAccount = new web3.Keypair();
    console.log(
      "saveDestChainIdAccount:",
      saveDestChainIdAccount.publicKey.toBase58()
    );

    let dapp = encodeEthereumAddressToU8Array("0xaE67336f06B10fbbb26F31d31AbEA897290109B9");

    //initializeVizingPad
    async function InitializeVizingPad() {
      try {
        const vizingPadAccount =
          await pg.program.account.vizingPadSettings.fetch(vizingPadSettings);
        console.log("vizingPadAccount:", vizingPadAccount.owner.toBase58());
      } catch (e) {
        const tx = await pg.program.methods
          .initializeVizingPad(initParams)
          .accounts({
            vizing: vizingPadSettings,
            vizingAuthority: vizingAuthority,
            payer: user,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        console.log(`initializeVizingPad: ${tx}`);
      }
    }
    await InitializeVizingPad();

    //saveChainId
    async function SaveChainId() {
      try {
        const saveDestChainId = await pg.program.methods
          .saveChainId(chainId)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer, saveDestChainIdAccount])
          .rpc();
        console.log(`saveDestChainId:${saveDestChainId}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(saveDestChainId);

        const getChainId = await pg.program.account.saveChainId.fetch(
          saveDestChainIdAccount.publicKey
        );
        console.log("getChainId:", getChainId);
      } catch (e) {
        console.log("saveDestChainId error:", e);
      }
    }
    await SaveChainId();

    //initVizingVault
    async function InitVizingVault() {
      try {
        const vaultMes = await pg.program.account.vaultMes.fetch(
          vizingVaultAuthority
        );
        console.log("InitVizingVault:", vaultMes);
      } catch (e) {
        const initVizingVault = await pg.program.methods
          .initVizingVault()
          .accounts({
            vizingVault: vizingVaultAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initVizingVault:${initVizingVault}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(initVizingVault);
        const vaultMes = await pg.program.account.vaultMes.fetch(
          vizingVaultAuthority
        );
        console.log("InitVizingVault:", vaultMes);
      }
    }
    await InitVizingVault();

    let base_price = new anchor.BN(500);
    let reserve = new anchor.BN(1000);
    let molecular = new anchor.BN(5);
    let denominator = new anchor.BN(10);
    let molecular_decimal = 6;
    let denominator_decimal = 6;
    //init_fee_config
    async function InitFeeConfig() {
      try {
        const mappingFeeConfig =
          await pg.program.account.mappingFeeConfig.fetch(
            mappingFeeConfigAuthority
          );
        console.log("mappingFeeConfig:", mappingFeeConfig);
      } catch (e) {
        try {
          const initFeeConfig = await pg.program.methods
            .initFeeConfig(
              id,
              base_price,
              reserve,
              molecular,
              denominator,
              molecular_decimal,
              denominator_decimal
            )
            .accounts({
              saveChainId: saveDestChainIdAccount.publicKey,
              vizing: vizingPadSettings,
              mappingFeeConfig: mappingFeeConfigAuthority,
              user: user,
              systemProgram: systemId,
            })
            .signers([signer])
            .rpc();
          console.log(`initFeeConfig:${initFeeConfig}'`);
          // Confirm transaction
          await pg.connection.confirmTransaction(initFeeConfig);
        } catch (e) {
          console.log("initFeeConfig error:", e);
        }
      }
    }
    await InitFeeConfig();

    //init_gas_global
    let global_base_price = new anchor.BN(500);
    let default_gas_limit = new anchor.BN(1000);
    let amount_in_threshold = new anchor.BN(1000000000);
    async function InitGasGlobal() {
      try {
        const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      } catch (e) {
        const initGasGlobal = await pg.program.methods
          .initGasGlobal(
            global_base_price,
            default_gas_limit,
            amount_in_threshold,
            molecular,
            denominator
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            gasSystemGlobal: gasSystemGlobalAuthority,
            vizing: vizingPadSettings,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initGasGlobal:${initGasGlobal}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(initGasGlobal);
        const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      }
    }
    await InitGasGlobal();

    //init_native_token_trade_fee_config
    let native_molecular = new anchor.BN(5);
    let native_denominator = new anchor.BN(10);
    async function InitNativeTokenTradeFeeConfig() {
      try {
        const mappingNativeTokenTradeFeeConfig =
          await pg.program.account.mappingNativeTokenTradeFeeConfig.fetch(
            nativeTokenTradeFeeConfigAuthority
          );
        console.log(
          "mappingNativeTokenTradeFeeConfig:",
          mappingNativeTokenTradeFeeConfig,
          "\n"
        );
      } catch (e) {
        const initNativeTokenTradeFeeConfig = await pg.program.methods
          .initNativeTokenTradeFeeConfig(
            id,
            native_molecular,
            native_denominator
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            nativeTokenTradeFeeConfig: nativeTokenTradeFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `initNativeTokenTradeFeeConfig:${initNativeTokenTradeFeeConfig}'`
        );
        // Confirm transaction
        await pg.connection.confirmTransaction(initNativeTokenTradeFeeConfig);

        const mappingNativeTokenTradeFeeConfig =
          await pg.program.account.mappingNativeTokenTradeFeeConfig.fetch(
            nativeTokenTradeFeeConfigAuthority
          );
        console.log(
          "mappingNativeTokenTradeFeeConfig:",
          mappingNativeTokenTradeFeeConfig,
          "\n"
        );
      }
    }
    await InitNativeTokenTradeFeeConfig();

    let symbol = Buffer.from("usdt");
    let tokenAddress = encodeEthereumAddressToU8Array(
      "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    );

    let init_decimals = 6;
    let init_max_price = new anchor.BN(1000);
    async function InitTokenInfoBase() {
      try {
        const mappingSymbolConfig =
          await pg.program.account.mappingSymbolConfig.fetch(
            initSymbolConfigAuthority
          );
        console.log("mappingSymbolConfig:", mappingSymbolConfig);
        const mappingTokenConfig =
          await pg.program.account.mappingTokenConfig.fetch(
            initTokenConfigAuthority
          );
        console.log("mappingTokenConfig:", mappingTokenConfig);
      } catch (e) {
        const initTokenInfoBase = await pg.program.methods
          .initTokenInfoBase(
            symbol,
            tokenAddress,
            init_decimals,
            init_max_price
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            user: user,
            tokenConfig: initTokenConfigAuthority,
            symbolConfig: initSymbolConfigAuthority,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initTokenInfoBase:${initTokenInfoBase}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(initTokenInfoBase);
      }
    }
    await InitTokenInfoBase();

    //modifySettings
    const modifyParams = {
      owner: user,
      feeReceiver: feeReceiverKeyPair.publicKey,
      engineAdmin: engineAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: user,
      stationAdmin: stationAdminKeyPair.publicKey,
      trustedRelayers: trustedRelayerKeyPairs.map(
        (keypair) => keypair.publicKey
      ),
      registeredValidator: anchor.web3.Keypair.generate().publicKey,
      isPaused: false,
    };
    async function ModifySettings() {
      try {
        await pg.program.methods
          .modifySettings(modifyParams)
          .accounts({
            owner: user,
            vizing: vizingPadSettings,
          })
          .signers([signer])
          .rpc();
      } catch (e) {
        console.log("modifySettings error:", e);
      }
    }
    await ModifySettings();

    //grantFeeCollector
    async function GrantFeeCollector() {
      try {
        let vizingPadAccount = await pg.program.account.vizingPadSettings.fetch(
          vizingPadSettings
        );
        console.log(
          "vizingPadAccount:",
          vizingPadAccount.feeReceiver.toBase58()
        );
      } catch (e) {
        await pg.program.methods
          .grantFeeCollector(feeReceiverKeyPair.publicKey)
          .accounts({
            gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
            vizing: vizingPadSettings,
          })
          .signers([gasPoolAdminKeyPair])
          .rpc();
        console.log("grantFeeCollector error:", e);
      }
    }
    await GrantFeeCollector();

    //setThisGasGlobal
    let new_global_base_price = new anchor.BN(500);
    let new_default_gas_limit = new anchor.BN(1000);
    let new_amount_in_threshold = new anchor.BN(100000000);
    async function SetThisGasGlobal(thisGlobalBasePrice,thisDefaultGasLimit,thisAmountInThreshold,thisMolecular,thisDenominator) {
      try {
        const setThisGasGlobal = await pg.program.methods
          .setThisGasGlobal(
            thisGlobalBasePrice,
            thisDefaultGasLimit,
            thisAmountInThreshold,
            thisMolecular,
            thisDenominator
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            gasSystemGlobal: gasSystemGlobalAuthority,
            vizing: vizingPadSettings,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisGasGlobal:${setThisGasGlobal}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisGasGlobal);

        const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      } catch (e) {
        console.log("SetThisGasGlobal error:", e);
      }
    }
    
    await SetThisGasGlobal(new_global_base_price,new_default_gas_limit,new_amount_in_threshold,molecular,denominator);

    //set_this_fee_config
    async function SetThisFeeConfig() {
      try {
        const setThisFeeConfig = await pg.program.methods
          .setThisFeeConfig(
            id,
            base_price,
            reserve,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisFeeConfig:${setThisFeeConfig}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisFeeConfig);
      } catch (e) {
        console.log("SetThisFeeConfig error:", e);
      }
    }
    await SetThisFeeConfig();

    //set_token_fee_config
    async function SetThisTokenFeeConfig() {
      try {
        const setThisTokenFeeConfig = await pg.program.methods
          .setThisTokenFeeConfig(id, molecular, denominator)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            gasSystemGlobal: gasSystemGlobalAuthority,
            nativeTokenTradeFeeConfig: nativeTokenTradeFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisTokenFeeConfig:${setThisTokenFeeConfig}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisTokenFeeConfig);
      } catch (e) {
        console.log("SetThisTokenFeeConfig error:", e);
      }
    }
    await SetThisTokenFeeConfig();

    async function SetThisDappPriceConfig() {
      try {
        const setThisDappPriceConfig = await pg.program.methods
          .setThisDappPriceConfig(id, dapp, base_price)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisDappPriceConfig:${setThisDappPriceConfig}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisDappPriceConfig);

        const mappingFeeConfig =
          await pg.program.account.mappingFeeConfig.fetch(
            mappingFeeConfigAuthority
          );
        const this_dapp_date = mappingFeeConfig.dappConfigMappings[0].dapp;
        const this_dapp = decodeU8ArrayToEthereumAddress(this_dapp_date);
        console.log("decode dapp address:", this_dapp);
      } catch (e) {
        console.log("SetThisDappPriceConfig error:", e);
      }
    }
    await SetThisDappPriceConfig();

    //set_exchange_rate
    async function SetThisExchangeRate() {
      try {
        const setThisExchangeRate = await pg.program.methods
          .setThisExchangeRate(
            id,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisExchangeRate:${setThisExchangeRate}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisExchangeRate);
      } catch (e) {
        console.log("SetThisExchangeRate error:", e);
      }
    }
    await SetThisExchangeRate();

    //batch_set_token_fee_config
    let destChainIds = [id];
    let moleculars = [new anchor.BN(5)];
    let denominators = [new anchor.BN(10)];
    async function BatchSetThisTokenFeeConfig() {
      try {
        const batchSetThisTokenFeeConfig = await pg.program.methods
          .batchSetThisTokenFeeConfig(destChainIds, moleculars, denominators)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            nativeTokenTradeFeeConfig: nativeTokenTradeFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisTokenFeeConfig:${batchSetThisTokenFeeConfig}'`
        );
        // Confirm transaction
        await pg.connection.confirmTransaction(batchSetThisTokenFeeConfig);
      } catch (e) {
        console.log("BatchSetThisTokenFeeConfig error:", e);
      }
    }
    await BatchSetThisTokenFeeConfig();

    //batch_set_this_trade_fee_config_map
    let dapps = [dapp];
    async function BatchSetThisTradeFeeConfigMap() {
      try {
        const batchSetThisTradeFeeConfigMap = await pg.program.methods
          .batchSetThisTradeFeeConfigMap(
            dapps,
            destChainIds,
            moleculars,
            denominators
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            nativeTokenTradeFeeConfig: nativeTokenTradeFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisTradeFeeConfigMap:${batchSetThisTradeFeeConfigMap}'`
        );
        // Confirm transaction
        await pg.connection.confirmTransaction(batchSetThisTradeFeeConfigMap);
      } catch (e) {
        console.log("BatchSetThisTradeFeeConfigMap error:", e);
      }
    }
    await BatchSetThisTradeFeeConfigMap();

    //batch_set_this_dapp_price_config_in_diff_chain
    let base_prices = [new anchor.BN(1000)];
    async function BatchSetThisDappPriceConfigInDiffChain() {
      try {
        const batchSetThisDappPriceConfigInDiffChain = await pg.program.methods
          .batchSetThisDappPriceConfigInDiffChain(
            destChainIds,
            dapps,
            base_prices
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisDappPriceConfigInDiffChain:${batchSetThisDappPriceConfigInDiffChain}'`
        );
        // Confirm transaction
        await pg.connection.confirmTransaction(
          batchSetThisDappPriceConfigInDiffChain
        );
      } catch (e) {
        console.log("BatchSetThisDappPriceConfigInDiffChain error:", e);
      }
    }
    await BatchSetThisDappPriceConfigInDiffChain();

    //batch_set_this_dapp_price_config_in_same_chain
    async function BatchSetThisDappPriceConfigInSameChain() {
      try {
        const batchSetThisDappPriceConfigInSameChain = await pg.program.methods
          .batchSetThisDappPriceConfigInSameChain(id, dapps, base_prices)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisDAppPriceConfigInSameChain:${batchSetThisDappPriceConfigInSameChain}'`
        );
        // Confirm transaction
        await pg.connection.confirmTransaction(
          batchSetThisDappPriceConfigInSameChain
        );
      } catch (e) {
        console.log("BatchSetThisDappPriceConfigInSameChain error:", e);
      }
    }
    await BatchSetThisDappPriceConfigInSameChain();

    //get
    async function GetDappBasePrice(dest_chain_id, chain_base_price, dapp) {
      let dapp_base_price;
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const dappConfigMappings = await mappingFeeConfig.dappConfigMappings;
      let dapp_config_value = dappConfigMappings[0].value.toNumber();
      if (dapp_config_value > 0) {
        dapp_base_price = dapp_config_value;
      } else {
        dapp_base_price = chain_base_price;
      }
      console.log("GetDappBasePrice:", dapp_base_price);
      return dapp_base_price;
    }

    async function ExactOutput(dest_chain_id, amount_out) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const feeConfigMappings = await mappingFeeConfig.feeConfigMappings;
      const fee_config_molecular_decimal =
        feeConfigMappings[0].molecularDecimal;
      const fee_config_denominator_decimal =
        feeConfigMappings[0].denominatorDecimal;
      let this_amount_out;
      if (fee_config_molecular_decimal != fee_config_denominator_decimal) {
        if (fee_config_molecular_decimal > fee_config_denominator_decimal) {
          this_amount_out =
            (amount_out / 10) ^
            (fee_config_molecular_decimal - fee_config_denominator_decimal);
        } else {
          this_amount_out =
            (amount_out / 10) ^
            (fee_config_denominator_decimal - fee_config_molecular_decimal);
        }
      } else {
        this_amount_out = amount_out;
      }

      let amount_in =
        (this_amount_out * fee_config_denominator_decimal) /
        fee_config_denominator_decimal;
      console.log("ExactOutput:", amount_in);
      return amount_in;
    }

    async function ComputeTradeFee2(
      target_contract,
      dest_chain_id,
      amount_out
    ) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
        gasSystemGlobalAuthority
      );
      const tradeFeeConfigMappings =
        await mappingFeeConfig.tradeFeeConfigMappings;
      let trade_fee_config_molecular =
        tradeFeeConfigMappings[0].molecular.toNumber();
      let trade_fee_config_denominator =
        tradeFeeConfigMappings[0].denominator.toNumber();
      let global_trade_fee_molecular = gasSystemGlobal.molecular.toNumber();
      let global_trade_fee_denominator = gasSystemGlobal.denominator.toNumber();
      let fee;
      if (trade_fee_config_denominator > 0) {
        fee =
          (amount_out * trade_fee_config_molecular) /
          trade_fee_config_denominator;
      } else {
        fee =
          (amount_out * global_trade_fee_molecular) /
          global_trade_fee_denominator;
      }
      console.log("ComputeTradeFee2:", fee);
      return fee;
    }

    async function EstimatePrice1(target_contract, dest_chain_id) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const dappConfigMappings = await mappingFeeConfig.dappConfigMappings;
      const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
        gasSystemGlobalAuthority
      );
      let gas_system_global_base_price =
        await gasSystemGlobal[0].global_base_price.toNumber();
      let dapp_config_value = dappConfigMappings[0].value.toNumber();
      let dapp_base_price;
      if (dapp_config_value > 0) {
        dapp_base_price = dapp_config_value;
      } else {
        dapp_base_price = gas_system_global_base_price;
      }
      console.log("EstimatePrice1:", dapp_base_price);
      return dapp_base_price;
    }

    async function EstimatePrice2(dest_chain_id) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const feeConfigMappings = await mappingFeeConfig.feeConfigMappings;
      const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
        gasSystemGlobalAuthority
      );

      let gas_system_global_base_price =
        await gasSystemGlobal[0].global_base_price.toNumber();
      let fee_config_base_price =
        await feeConfigMappings[0].basePrice.toNumber();
      let base_price;
      if (fee_config_base_price > 0) {
        base_price = fee_config_base_price;
      } else {
        base_price = gas_system_global_base_price;
      }
      console.log("EstimatePrice2:", base_price);
      return base_price;
    }

    async function ExactInput(dest_chain_id, amount_in) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const feeConfigMappings = await mappingFeeConfig.feeConfigMappings;
      let fee_config_molecular_decimal = feeConfigMappings[0].molecularDecimal;
      let fee_config_denominator_decimal =
        feeConfigMappings[0].denominatorDecimal;
      let this_amount_in;
      if (fee_config_molecular_decimal != fee_config_denominator_decimal) {
        if (fee_config_molecular_decimal > fee_config_denominator_decimal) {
          this_amount_in =
            (amount_in * 10) ^
            (fee_config_molecular_decimal - fee_config_denominator_decimal);
        } else {
          this_amount_in =
            (amount_in / 10) ^
            (fee_config_denominator_decimal - fee_config_molecular_decimal);
        }
      } else {
        this_amount_in = amount_in;
      }
      let amount_out =
        (this_amount_in * fee_config_molecular_decimal) /
        fee_config_denominator_decimal;
      console.log("ExactInput:", amount_out);
      return amount_out;
    }

    let testAmountOut = new anchor.BN(1000);
    const testExecuteGasLimit = new anchor.BN(6);
    const testMaxFeePerGas = new anchor.BN(2000);
    const newMessage = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: testExecuteGasLimit,
      maxFeePerGas: testMaxFeePerGas,
      signature: Buffer.from("000000001"),
    };
    async function EstimateGas(amount_out, dest_chain_id, this_message) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const feeConfigMappings = await mappingFeeConfig.feeConfigMappings;
      const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
        gasSystemGlobalAuthority
      );
      let base_price;
      let fee;
      const feeConfigBasePrice = feeConfigMappings[0].basePrice.toNumber();
      const global_base_price = gasSystemGlobal.globalBasePrice.toNumber();
      const default_gas_limit = gasSystemGlobal.defaultGasLimit.toNumber();
      if (feeConfigBasePrice > 0) {
        base_price = feeConfigBasePrice;
      } else {
        base_price = global_base_price;
      }
      if (this_message.mode == 1 || this_message.mode == 2) {
        let dapp_base_price = await GetDappBasePrice(
          dest_chain_id,
          base_price,
          this_message.targetContract
        );
        let this_price;
        if (this_message.maxFeePerGas < dapp_base_price) {
          this_price = dapp_base_price;
        } else {
          this_price = this_message.maxFeePerGas;
        }
        fee = this_price * this_message.executeGasLimit;
      } else if (this_message.mode == 4) {
        fee = base_price * this_message.executeGasLimit;
      } else {
        fee = base_price * default_gas_limit;
      }

      if (amount_out > 0) {
        let output_amount_in;
        let fee_config_molecular = feeConfigMappings[0].molecular.toNumber();
        if (fee_config_molecular != 0) {
          output_amount_in = await ExactOutput(dest_chain_id, amount_out);
        }

        let trade_fee2 = await ComputeTradeFee2(
          this_message.targetContract,
          dest_chain_id,
          output_amount_in
        );
        fee += trade_fee2;
      }
      console.log("finally fee:", fee);
      return fee;
    }
    await EstimateGas(testAmountOut, id, newMessage);

    async function EstimateTotalFee(dest_chain_id, amount_out, this_message) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const feeConfigMappings = mappingFeeConfig.feeConfigMappings;
      const gasSystemGlobal = await pg.program.account.gasSystemGlobal.fetch(
        gasSystemGlobalAuthority
      );

      const token_amount_limit = gasSystemGlobal.amountInThreshold.toNumber();

      const feeConfigBasePrice = feeConfigMappings[0].basePrice.toNumber();
      const global_base_price = gasSystemGlobal.globalBasePrice.toNumber();
      const default_gas_limit = gasSystemGlobal.defaultGasLimit.toNumber();

      let base_price;
      let fee;
      if (feeConfigBasePrice > 0) {
        base_price = feeConfigBasePrice;
      } else {
        base_price = global_base_price;
      }

      if (this_message.mode == 1 || this_message.mode == 2) {
        let dapp_base_price = await GetDappBasePrice(
          dest_chain_id,
          base_price,
          this_message.targetContract
        );
        if (this_message.maxFeePerGas < dapp_base_price) {
          console.log("price < dapp_base_price");
          return 0;
        }
        fee = this_message.maxFeePerGas * this_message.executeGasLimit;
      } else if (this_message.mode == 4) {
        fee = base_price * this_message.executeGasLimit;
      } else {
        fee = base_price * default_gas_limit;
      }

      let output_amount_in = amount_out;
      let finalFee;
      if (amount_out.toNumber() > 0) {
        let fee_config_molecular = feeConfigMappings[0].molecular.toNumber();
        if (fee_config_molecular != 0) {
          output_amount_in = await ExactOutput(dest_chain_id, amount_out.toNumber());
        }

        let trade_fee2 = await ComputeTradeFee2(
          this_message.targetContract,
          dest_chain_id,
          output_amount_in
        );
        finalFee = trade_fee2 + output_amount_in + fee;
      }
      if (output_amount_in > token_amount_limit) {
        throw "Overflow token amount limit!";
      }
      console.log("EstimateTotalFee:", finalFee);
      return finalFee;
    }
    await EstimateTotalFee(id, testAmountOut , newMessage);

    //batch_set_exchange_rate
    let molecular_decimals = Buffer.from([6]);
    let denominator_decimals = Buffer.from([6]);
    async function BatchSetThisExchangeRate() {
      try {
        const batchSetThisExchangeRate = await pg.program.methods
          .batchSetThisExchangeRate(
            destChainIds,
            moleculars,
            denominators,
            molecular_decimals,
            denominator_decimals
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`batchSetThisExchangeRate:${batchSetThisExchangeRate}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(batchSetThisExchangeRate);
      } catch (e) {
        console.log("BatchSetThisExchangeRate error:", e);
      }
    }
    await BatchSetThisExchangeRate();

    //ChangeThisPowerUser
    // async function ChangeThisPowerUser() {
    //   try {
    //     const changeThisPowerUser = await pg.program.methods
    //       .changeThisPowerUser(
    //         user,
    //         new_engine_admins,
    //         new_station_admins,
    //         new_gas_pool_admins,
    //         new_trusted_relayers,
    //         new_registered_validators,
    //         gas_managers,
    //         swap_managers,
    //         token_managers
    //       )
    //       .accounts({
    //         saveChainId: saveDestChainIdAccount.publicKey,
    //         vizing: vizingPadSettings,
    //         user: user,
    //         systemProgram: systemId,
    //       })
    //       .signers([signer])
    //       .rpc();
    //     console.log(`changeThisPowerUser:${changeThisPowerUser}'`);
    //     // Confirm transaction
    //     await pg.connection.confirmTransaction(changeThisPowerUser);
    //   } catch (e) {
    //     const powerUser = await pg.program.account.powerUser.fetch(
    //       powerUserAuthority
    //     );
    //     console.log("powerUser:", powerUser);
    //   }
    // }
    // await ChangeThisPowerUser();

    //vizingVaultAssociatedToken
    let vizingVaultAssociatedToken = await GetOrcreateAssociatedToken(
      vizingVaultAuthority,
      newTokenMes.publicKey,
      signer
    );
    console.log(
      "vizingVaultAssociatedToken:",
      vizingVaultAssociatedToken.toBase58()
    );

    await transfer_to(vizingVaultAssociatedToken);

    //WithdrawSplToken
    let withdraw_amount = new anchor.BN(1000);
    async function WithdrawVaultSplToken() {
      try {
        const withdrawVaultSplToken = await pg.program.methods
          .withdrawVaultSplToken(withdraw_amount, vizingVaultBump)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            user: user,
            source: vizingVaultAssociatedToken,
            destination: userAssociatedAccount,
            contractAuthority: vizingVaultAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([signer])
          .rpc();
        console.log(`withdrawVaultSplToken:${withdrawVaultSplToken}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(withdrawVaultSplToken);
      } catch (e) {
        console.log("WithdrawVaultSplToken error:", e);
      }
    }
    await WithdrawVaultSplToken();

    // sol_transfer
    // let amount1 = new anchor.BN(1000000);
    // async function SolTransfer(sender, receiver, amount) {
    //   try {
    //     const solTransfer = await pg.program.methods
    //       .transferSolValut(amount)
    //       .accounts({
    //         sender: sender,
    //         vizingVault: receiver,
    //         systemProgram: systemId,
    //       })
    //       .signers([signer])
    //       .rpc();
    //     console.log(`solTransfer:${solTransfer}'`);
    //     // Confirm transaction
    //     await pg.connection.confirmTransaction(solTransfer);
    //   } catch (e) {
    //     console.log("SolTransfer error:", e);
    //   }
    // }
    // await SolTransfer(user, vizingVaultAuthority, amount1);

    //withdraw_sol
    // async function WithdrawVaultSol(sender, receiver, amount) {
    //   try {
    //     const withdrawVaultSol = await pg.program.methods
    //       .withdrawVaultSol(amount)
    //       .accounts({
    //         saveChainId: saveDestChainIdAccount.publicKey,
    //         vizing: vizingPadSettings,
    //         user: user,
    //         source: sender,
    //         destination: receiver,
    //         systemProgram: systemId,
    //       })
    //       .signers([signer])
    //       .rpc();
    //     console.log(`withdrawVaultSol:${withdrawVaultSol}'`);
    //     // Confirm transaction
    //     await pg.connection.confirmTransaction(withdrawVaultSol);
    //   } catch (e) {
    //     console.log("WithdrawVaultSol error:", e);
    //   }
    // }
    // let amount2 = new anchor.BN(55555);
    // await WithdrawVaultSol(vizingVaultAuthority, user, amount2);

    //set_this_token_info_base
    const tokenAddressArray: number[] = Array.from(tokenAddress);
    let decimals = 8;
    let max_price = new anchor.BN(1000);
    async function SetThisTokenInfoBase() {
      try {
        const setThisTokenInfoBase = await pg.program.methods
          .setThisTokenInfoBase(symbol, tokenAddress, decimals, max_price)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            user: user,
            tokenConfig: initTokenConfigAuthority,
            symbolConfig: initSymbolConfigAuthority,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisTokenInfoBase:${setThisTokenInfoBase}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisTokenInfoBase);
      } catch (e) {
        console.log("SetThisTokenInfoBase error:", e);
      }
    }
    await SetThisTokenInfoBase();

    async function SetThisTokenTradeFeeMap() {
      try {
        const setThisTokenTradeFeeMap = await pg.program.methods
          .setThisTokenTradeFeeMap(
            tokenAddress,
            destChainIds,
            moleculars,
            denominators
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            vizing: vizingPadSettings,
            user: user,
            tokenConfig: initTokenConfigAuthority,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisTokenTradeFeeMap:${setThisTokenTradeFeeMap}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisTokenTradeFeeMap);
      } catch (e) {
        console.log("SetThisTokenTradeFeeMap error:", e);
      }
    }
    await SetThisTokenTradeFeeMap();

    // launch
    const executeGasLimit = new BN(6);
    const maxFeePerGas = new BN(2000);

    const message = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("000000001"),
    };

    const launchParams = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: user,
      sender: user,
      value: new anchor.BN(1000),
      destChainid: new anchor.BN(4),
      additionParams: Buffer.alloc(0),
      message: message,
    };
    async function Launch(thisLaunchParams) {
      try {
        let launch = await pg.program.methods
          .launch(thisLaunchParams)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            feePayer: user,
            messageAuthority: user,
            vizing: vizingPadSettings,
            feeCollector: feeReceiverKeyPair.publicKey,
            mappingFeeConfig: mappingFeeConfigAuthority,
            gasSystemGlobal: gasSystemGlobalAuthority,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();

        console.log(`Launch tx:${launch}'`);
        await pg.connection.confirmTransaction(launch);
      } catch (e) {
        console.log("launch error:", e);
      }
    }
    //success launch
    let thisTestValue=new anchor.BN(1000);
    let thisFee1=await EstimateTotalFee(id, thisTestValue , message);
    let solBefore1=await getSolBalance(user);
    await Launch(launchParams);
    let solAfter1=await getSolBalance(user);
    let differ1=solBefore1-solAfter1;
    if(differ1>=thisFee1){
      console.log("launch1 success",differ1);
    }else{
      console.log("launch1 amount error",differ1);
    }

    //big number value launch
    let thisTestValue2=new anchor.BN(1000000);
    let thisFee2=await EstimateTotalFee(id, thisTestValue2 , message);
    let solBefore2=await getSolBalance(user);
    const newLaunchParams1 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: user,
      sender: user,
      value: thisTestValue2,
      destChainid: new anchor.BN(4),
      additionParams: Buffer.alloc(0),
      message: message,
    };
    await Launch(newLaunchParams1);
    let solafter2=await getSolBalance(user);
    let differ2=solBefore2-solafter2;
    if(differ2>=thisFee2){
      console.log("launch2 success",differ2);
    }else{
      console.log("launch2 amount error",differ2);
    }

    // expect(vizingPadAccount.owner.toBase58()).to.equal(
    //     provider.wallet.publicKey.toBase58()
    // );


    //error amount_in_threshold
    let this_amount_in_threshold=new anchor.BN(100);
    await SetThisGasGlobal(new_global_base_price,new_default_gas_limit,this_amount_in_threshold,molecular,denominator);
    await Launch(launchParams);
    await SetThisGasGlobal(new_global_base_price,new_default_gas_limit,amount_in_threshold,molecular,denominator);

    //error not relayer
    let newRelayer=new web3.Keypair();
    const newLaunchParams2 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: newRelayer.publicKey,
      sender: user,
      value: thisTestValue2,
      destChainid: new anchor.BN(4),
      additionParams: Buffer.alloc(0),
      message: message,
    };
    await Launch(newLaunchParams2);

    //error message
    function encodeEthereumAddressTo40U8Array(ethAddress: string): number[] {
      const address = ethAddress.slice(2); // Remove the '0x' prefix
      const result = new Uint8Array(40);
      for (let i = 0; i < 40; i++) {
        let charAddressI = address[i].charCodeAt(0);
        result[i] = charAddressI;
      }

      const addressArray: number[] = Array.from(result);
      return addressArray;
    }
    let by40Dapp = encodeEthereumAddressTo40U8Array("0xaE67336f06B10fbbb26F31d31AbEA897290109B9");
    const errorDappMessage = {
      mode: 1,
      targetContract: by40Dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("000000001"),
    };
    const newLaunchParams3 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: newRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: new anchor.BN(4),
      additionParams: Buffer.alloc(0),
      message: errorDappMessage,
    };
    await Launch(newLaunchParams3);

    //error invalid eth address
    let invalidDapp = encodeEthereumAddressTo40U8Array("0xAAA777733332222bb26F31d31AbEA897290109B9");
    const errorDappMessage2= {
      mode: 1,
      targetContract: invalidDapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("000000001"),
    };
    const newLaunchParams4 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: newRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: new anchor.BN(4),
      additionParams: Buffer.alloc(0),
      message: errorDappMessage2,
    };
    await Launch(newLaunchParams4);

    //get
    // async function GetEstimateGas(amount_out, dest_chain_id, this_message) {
    //   try {
    //     const estimateGas = await pg.program.methods
    //       .estimateGas(amount_out, dest_chain_id, this_message)
    //       .accounts({
    //         saveChainId: saveDestChainIdAccount.publicKey,
    //         mappingFeeConfig: mappingFeeConfigAuthority,
    //         gasSystemGlobal: gasSystemGlobalAuthority,
    //       })
    //       .signers([signer])
    //       .rpc();
    //     console.log(`estimateGas:${estimateGas}'`);
    //     // Confirm transaction
    //     await pg.connection.confirmTransaction(estimateGas);
    //   } catch (e) {
    //     console.log("estimateGas error:", e);
    //   }
    // }
    // await GetEstimateGas(testAmountOut, id, newMessage);


  });
});



