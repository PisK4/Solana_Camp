import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@project-serum/anchor";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  mintTo,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  transfer,
} from "@solana/spl-token";
// import { Opool } from "../target/types/opool";

import { clusterApiUrl, Connection } from "@solana/web3.js";

import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";
import {
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { web3 } from "@project-serum/anchor";
import type { State } from "../target/types/state";

const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
let token_programId = TOKEN_PROGRAM_ID;
let associated_token_programId = ASSOCIATED_TOKEN_PROGRAM_ID;

let user = program.provider.publicKey;
let signer = program.provider.wallet.payer;

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
    const metadataTxHash = await program.provider.connection.sendTransaction(transaction, [
      signer,
    ]);
    console.log(`Transaction sent`);
    // confirm
    const metadataConfirmation = await program.provider.connection.confirmTransaction(
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
    console.log(`This Account Token balance: ${tokenAccountInfo.amount}`);
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
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.State as anchor.Program<State>;
  
  it("initialize", async () => {
    console.log("current user:", user.toBase58());

    let systemId = web3.SystemProgram.programId;

    function isUpperCaseLetter(byte: string): boolean {
      const code = byte.charCodeAt(0);
      return code >= 0x41 && code <= 0x5a;
    }

    function isLowerCaseLetter(byte: string): boolean {
      const code = byte.charCodeAt(0);
      return code >= 0x61 && code <= 0x7a;
    }

    function isLetter(byte: string): boolean {
      const code = byte.charCodeAt(0);
      return code >= 0x30 && code <= 0x39;
    }

    function chooseEncode(byte1: string, byte2: string): number {
      if (isLetter(byte1) && isLetter(byte2)) {
        return 1;
      } else if (isUpperCaseLetter(byte1) && isLowerCaseLetter(byte2)) {
        return 2;
      } else if (isLowerCaseLetter(byte1) && isUpperCaseLetter(byte2)) {
        return 3;
      } else if (isLowerCaseLetter(byte1) && isLowerCaseLetter(byte2)) {
        return 4;
      } else if (isUpperCaseLetter(byte1) && isUpperCaseLetter(byte2)) {
        return 5;
      } else if (isUpperCaseLetter(byte1) && isLetter(byte2)) {
        return 6;
      } else if (isLowerCaseLetter(byte1) && isLetter(byte2)) {
        return 7;
      } else if (isLetter(byte1) && isUpperCaseLetter(byte2)) {
        return 8;
      } else if (isLetter(byte1) && isLowerCaseLetter(byte2)) {
        return 9;
      } else {
        throw new Error("Bytes error");
      }
    }

    function encodeEthereumAddressToU16Array(ethAddress: string): Uint16Array {
      const address = ethAddress.slice(2); // Remove the '0x' prefix
      const result = new Uint16Array(50);
      result[0] = 666;
      for (let i = 1; i < 21; i++) {
        const byte1 = address[2 * (i - 1)];
        const byte2 = address[2 * (i - 1) + 1];

        const combined = byte1 + byte2;

        // Apply the rules to determine how to combine byte1 and byte2 into u16
        let encoded: number;
        let firstSort: number;
        let decimalValue: number;
        firstSort = chooseEncode(byte1, byte2);
        decimalValue = parseInt(combined, 16);
        encoded = parseInt(`${firstSort}${decimalValue}`);

        result[i] = encoded;
      }

      for (let i = 21; i < 50; i++) {
        result[i] = result[((i - 1) % 10) + 1];
      }

      return result;
    }

    let id = new anchor.BN(16);
    let chainId = new Buffer(`${id}`);
    console.log("chainId buffer:", chainId);

    //pda
    let [powerUserAuthority, powerUserBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_power_user"), chainId],
        program.programId
      );
    console.log("powerUserAuthority:", powerUserAuthority.toString());
    console.log("powerUserBump:", powerUserBump);

    let [vizingVaultAuthority, vizingVaultBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("vizing_vault")],
        program.programId
      );
    console.log("vizingVaultAuthority:", vizingVaultAuthority.toString());
    console.log("vizingVaultBump:", vizingVaultBump);

    //gas_global
    let [gasSystemGlobalAuthority, gasSystemGlobalBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("gas_global"), chainId],
        program.programId
      );
    console.log(
      "gasSystemGlobalAuthority:",
      gasSystemGlobalAuthority.toString()
    );
    console.log("gasSystemGlobalBump:", gasSystemGlobalBump);

    //global_trade_fee
    let [globalTradeFeeAuthority, globalTradeFeeBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("global_trade_fee"), chainId],
        program.programId
      );
    console.log("globalTradeFeeAuthority:", globalTradeFeeAuthority.toString());
    console.log("globalTradeFeeBump:", globalTradeFeeBump);

    //init_mapping_fee_config
    let [mappingFeeConfigAuthority, mappingFeeConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_mapping_fee_config"), chainId],
        program.programId
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
        program.programId
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
        program.programId
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
        program.programId
      );
    console.log(
      "initTokenConfigAuthority:",
      initTokenConfigAuthority.toString()
    );
    console.log(
      "initTokenConfigBump:",
      initTokenConfigBump
    );

    //init_symbol_config
    let [initSymbolConfigAuthority, initSymbolConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_symbol_config"), chainId],
        program.programId
      );
    console.log(
      "initSymbolConfigAuthority:",
      initSymbolConfigAuthority.toString()
    );
    console.log(
      "initSymbolConfigBump:",
      initSymbolConfigBump
    );

    //save_dest_chain_Id
    let saveDestChainIdAccount = new web3.Keypair();
    console.log(
      "saveDestChainIdAccount:",
      saveDestChainIdAccount.publicKey.toBase58()
    );

    async function SaveChainId() {
      try {
        const saveDestChainId = await program.methods
          .saveChainId(chainId)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer, saveDestChainIdAccount])
          .rpc();
        console.log(`saveDestChainId:${saveDestChainId}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(saveDestChainId);

        const getChainId = await program.account.saveChainId.fetch(
          saveDestChainIdAccount.publicKey
        );
        console.log("getChainId:", getChainId);
      } catch (e) {
        console.log("saveDestChainId error:", e);
      }
    }
    await SaveChainId();

    //init_this_power_user
    let gas_managers = [user];
    let swap_managers = [user];
    let token_managers = [user];
    async function InitPowerUser() {
      try {
        const powerUser = await program.account.powerUser.fetch(
          powerUserAuthority
        );
        console.log("powerUser:", powerUser);
      } catch (e) {
        const initPowerUser = await program.methods
          .initPowerUser(
            user,
            user,
            user,
            user,
            user,
            user,
            gas_managers,
            swap_managers,
            token_managers
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            vizingVault: vizingVaultAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initPowerUser:${initPowerUser}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initPowerUser);
      }
    }
    await InitPowerUser();

    let base_price = new anchor.BN(1000000);
    let reserve = new anchor.BN(100000000);
    let molecular = new anchor.BN(6666);
    let denominator = new anchor.BN(222);
    let molecular_decimal = 8;
    let denominator_decimal = 6;
    //init_fee_config
    async function InitFeeConfig() {
      try {
        const mappingFeeConfig =
          await program.account.mappingFeeConfig.fetch(
            mappingFeeConfigAuthority
          );
        console.log("mappingFeeConfig:", mappingFeeConfig);
      } catch (e) {
        const initFeeConfig = await program.methods
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
            powerUser: powerUserAuthority,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initFeeConfig:${initFeeConfig}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initFeeConfig);
      }
    }
    await InitFeeConfig();

    //init_gas_global
    let global_base_price = new anchor.BN(200000);
    let default_gas_limit = new anchor.BN(100000000);
    let amount_in_threshold = new anchor.BN(10000000000000);
    async function InitGasGlobal() {
      try {
        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee);
        const gasSystemGlobal = await program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      } catch (e) {
        const initGasGlobal = await program.methods
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
            globalTradeFee: globalTradeFeeAuthority,
            powerUser: powerUserAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initGasGlobal:${initGasGlobal}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initGasGlobal);
        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee);
        const gasSystemGlobal = await program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      }
    }
    await InitGasGlobal();

    //init_amount_in_thresholds
    async function InitAmountInThresholds() {
      try {
        const mappingAmountInThresholds =
          await program.account.mappingAmountInThresholds.fetch(
            amountInThresholdsAuthority
          );
        console.log("mappingAmountInThresholds:", mappingAmountInThresholds);
      } catch (e) {
        const initAmountInThresholds = await program.methods
          .initAmountInThresholds(id, amount_in_threshold)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            amountInThresholds: amountInThresholdsAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initAmountInThresholds:${initAmountInThresholds}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initAmountInThresholds);
        const mappingAmountInThresholds =
          await program.account.mappingAmountInThresholds.fetch(
            amountInThresholdsAuthority
          );
        console.log("mappingAmountInThresholds:", mappingAmountInThresholds);
      }
    }
    await InitAmountInThresholds();

    //init_native_token_trade_fee_config
    async function InitNativeTokenTradeFeeConfig() {
      try {
        const mappingNativeTokenTradeFeeConfig =
          await program.account.mappingNativeTokenTradeFeeConfig.fetch(
            nativeTokenTradeFeeConfigAuthority
          );
        console.log(
          "mappingNativeTokenTradeFeeConfig:",
          mappingNativeTokenTradeFeeConfig,
          "\n"
        );
      } catch (e) {
        const initNativeTokenTradeFeeConfig = await program.methods
          .initNativeTokenTradeFeeConfig(id, molecular, denominator)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
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
        await program.provider.connection.confirmTransaction(initNativeTokenTradeFeeConfig);

        const mappingNativeTokenTradeFeeConfig =
          await program.account.mappingNativeTokenTradeFeeConfig.fetch(
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

    //setThisGasGlobal
    let new_global_base_price = new anchor.BN(500000);
    let new_default_gas_limit = new anchor.BN(200000000);
    let new_amount_in_threshold = new anchor.BN(30000000000000);
    async function SetThisGasGlobal() {
      try {
        const setThisGasGlobal = await program.methods
          .setThisGasGlobal(
            new_global_base_price,
            new_default_gas_limit,
            new_amount_in_threshold,
            molecular,
            denominator
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            gasSystemGlobal: gasSystemGlobalAuthority,
            globalTradeFee: globalTradeFeeAuthority,
            powerUser: powerUserAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisGasGlobal:${setThisGasGlobal}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisGasGlobal);

        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee, "\n");
        const gasSystemGlobal = await program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      } catch (e) {
        console.log("SetThisGasGlobal error:", e);
      }
    }
    await SetThisGasGlobal();

    //set_this_fee_config
    async function SetThisFeeConfig() {
      try {
        const setThisFeeConfig = await program.methods
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
            powerUser: powerUserAuthority,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisFeeConfig:${setThisFeeConfig}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisFeeConfig);
      } catch (e) {
        console.log("SetThisFeeConfig error:", e);
      }
    }
    await SetThisFeeConfig();

    //set_token_fee_config
    async function SetThisTokenFeeConfig() {
      try {
        const setThisTokenFeeConfig = await program.methods
          .setThisTokenFeeConfig(id, molecular, denominator)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            globalTradeFee: globalTradeFeeAuthority,
            nativeTokenTradeFeeConfig: nativeTokenTradeFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisTokenFeeConfig:${setThisTokenFeeConfig}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisTokenFeeConfig);

        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee, "\n");
      } catch (e) {
        console.log("SetThisTokenFeeConfig error:", e);
      }
    }
    await SetThisTokenFeeConfig();

    let dapp = encodeEthereumAddressToU16Array(
      "0xaE67336f06B10fbbb26F31d31AbEA897290109B9"
    );
    const dappNumberArray: number[] = Array.from(dapp);

    async function SetThisDappPriceConfig() {
      try {
        const setThisDappPriceConfig = await program.methods
          .setThisDappPriceConfig(id, dappNumberArray, base_price)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisDappPriceConfig:${setThisDappPriceConfig}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisDappPriceConfig);

        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee, "\n");
      } catch (e) {
        console.log("SetThisDappPriceConfig error:", e);
      }
    }
    await SetThisDappPriceConfig();

    //set_exchange_rate
    async function SetThisExchangeRate() {
      try {
        const setThisExchangeRate = await program.methods
          .setThisExchangeRate(
            id,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisExchangeRate:${setThisExchangeRate}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisExchangeRate);
      } catch (e) {
        console.log("SetThisExchangeRate error:", e);
      }
    }
    await SetThisExchangeRate();

    //batch_set_token_fee_config
    let destChainIds = [new anchor.BN(id)];
    let moleculars = [new anchor.BN(68886)];
    let denominators = [new anchor.BN(222)];
    async function BatchSetThisTokenFeeConfig() {
      try {
        const batchSetThisTokenFeeConfig = await program.methods
          .batchSetThisTokenFeeConfig(destChainIds, moleculars, denominators)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
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
        await program.provider.connection.confirmTransaction(batchSetThisTokenFeeConfig);
      } catch (e) {
        console.log("BatchSetThisTokenFeeConfig error:", e);
      }
    }
    await BatchSetThisTokenFeeConfig();

    //batch_set_this_trade_fee_config_map
    let dapps = [dappNumberArray];
    async function BatchSetThisTradeFeeConfigMap() {
      try {
        const batchSetThisTradeFeeConfigMap = await program.methods
          .batchSetThisTradeFeeConfigMap(
            dapps,
            destChainIds,
            moleculars,
            denominators
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
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
        await program.provider.connection.confirmTransaction(batchSetThisTradeFeeConfigMap);
      } catch (e) {
        console.log("BatchSetThisTradeFeeConfigMap error:", e);
      }
    }
    await BatchSetThisTradeFeeConfigMap();

    //batch_set_amount_in_threshold
    let new_values = [new anchor.BN(88)];
    async function BatchSetThisAmountInThreshold() {
      try {
        const batchSetThisAmountInThreshold = await program.methods
          .batchSetThisAmountInThreshold(destChainIds, new_values)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            amountInThresholds: amountInThresholdsAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisAmountInThreshold:${batchSetThisAmountInThreshold}'`
        );
        // Confirm transaction
        await program.provider.connection.confirmTransaction(batchSetThisAmountInThreshold);
      } catch (e) {
        console.log("BatchSetThisAmountInThreshold error:", e);
      }
    }
    await BatchSetThisAmountInThreshold();

    //batch_set_this_dapp_price_config_in_diff_chain
    let base_prices = [new anchor.BN(6666)];
    async function BatchSetThisDappPriceConfigInDiffChain() {
      try {
        const batchSetThisDappPriceConfigInDiffChain = await program.methods
          .batchSetThisDappPriceConfigInDiffChain(
            destChainIds,
            dapps,
            base_prices
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
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
        await program.provider.connection.confirmTransaction(
          batchSetThisDappPriceConfigInDiffChain
        );
      } catch (e) {
        console.log("BatchSetThisDappPriceConfigInDiffChain error:", e);
      }
    }
    await BatchSetThisDappPriceConfigInDiffChain();

    //batch_set_this_dapp_price_config_in_same_chain
    let thisChainId = new anchor.BN(id);
    async function BatchSetThisDappPriceConfigInSameChain() {
      try {
        const batchSetThisDappPriceConfigInSameChain = await program.methods
          .batchSetThisDappPriceConfigInSameChain(
            thisChainId,
            dapps,
            base_prices
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
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
        await program.provider.connection.confirmTransaction(
          batchSetThisDappPriceConfigInSameChain
        );
      } catch (e) {
        console.log("BatchSetThisDappPriceConfigInSameChain error:", e);
      }
    }
    await BatchSetThisDappPriceConfigInSameChain();

    //batch_set_exchange_rate
    let molecular_decimals = Buffer.from("8");
    let denominator_decimals = Buffer.from("6");
    async function BatchSetThisExchangeRate() {
      try {
        const batchSetThisExchangeRate = await program.methods
          .batchSetThisExchangeRate(
            destChainIds,
            moleculars,
            denominators,
            molecular_decimals,
            denominator_decimals
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`batchSetThisExchangeRate:${batchSetThisExchangeRate}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(batchSetThisExchangeRate);
      } catch (e) {
        console.log("BatchSetThisExchangeRate error:", e);
      }
    }
    await BatchSetThisExchangeRate();

    //ChangeThisPowerUser
    async function ChangeThisPowerUser() {
      try {
        const changeThisPowerUser = await program.methods
          .changeThisPowerUser(
            user,
            user,
            user,
            user,
            user,
            user,
            gas_managers,
            swap_managers,
            token_managers
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`changeThisPowerUser:${changeThisPowerUser}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(changeThisPowerUser);
      } catch (e) {
        const powerUser = await program.account.powerUser.fetch(
          powerUserAuthority
        );
        console.log("powerUser:", powerUser);
      }
    }
    await ChangeThisPowerUser();

    //
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
        const withdrawVaultSplToken = await program.methods
          .withdrawVaultSplToken(withdraw_amount, vizingVaultBump)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
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
        await program.provider.connection.confirmTransaction(withdrawVaultSplToken);
      } catch (e) {
        console.log("WithdrawVaultSplToken error:", e);
      }
    }
    await WithdrawVaultSplToken();

    //sol_transfer
    let amount1 = new anchor.BN(1000000);
    async function SolTransfer(sender, receiver, amount) {
      try {
        const solTransfer = await program.methods
          .solTransfer(amount)
          .accounts({
            sender: sender,
            recipient: receiver,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`solTransfer:${solTransfer}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(solTransfer);
      } catch (e) {
        console.log("SolTransfer error:", e);
      }
    }
    await SolTransfer(user, vizingVaultAuthority, amount1);

    //withdraw_sol
    async function WithdrawVaultSol(sender, receiver, amount) {
      try {
        const withdrawVaultSol = await program.methods
          .withdrawVaultSol(amount)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            user: user,
            source: sender,
            destination: receiver,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`withdrawVaultSol:${withdrawVaultSol}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(withdrawVaultSol);
      } catch (e) {
        console.log("WithdrawVaultSol error:", e);
      }
    }
    let amount2 = new anchor.BN(55555);
    await WithdrawVaultSol(vizingVaultAuthority, user, amount2);\

    //set_this_token_info_base
    let symbol=Buffer.from("btc");
    let tokenAddress = encodeEthereumAddressToU16Array(
      "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    );
    const tokenAddressArray: number[] = Array.from(tokenAddress);
    let decimals=8;
    let max_price=new anchor.BN(66666666);
    async function SetThisTokenInfoBase() {
      try {
        const setThisTokenInfoBase = await program.methods
          .setThisTokenInfoBase(
            symbol,
            tokenAddressArray,
            decimals,
            max_price
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            user: user,
            tokenConfig: initTokenConfigAuthority,
            symbolConfig: initSymbolConfigAuthority,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `setThisTokenInfoBase:${setThisTokenInfoBase}'`
        );
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisTokenInfoBase);
      } catch (e) {
        console.log("SetThisTokenInfoBase error:", e);
      }
    }
    await SetThisTokenInfoBase();

    async function SetThisTokenTradeFeeMap() {
      try {
        const setThisTokenTradeFeeMap = await program.methods
          .setThisTokenTradeFeeMap(
            tokenAddressArray,
            destChainIds,
            moleculars,
            denominators
          )
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            user: user,
            tokenConfig: initTokenConfigAuthority,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `setThisTokenTradeFeeMap:${setThisTokenTradeFeeMap}'`
        );
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisTokenTradeFeeMap);
      } catch (e) {
        console.log("SetThisTokenTradeFeeMap error:", e);
      }
    }
    await SetThisTokenTradeFeeMap();

  });
});
