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
    console.log("sol balance:", balance);
    return balance;
  } catch (err) {
    console.error("Failed to get balance:", err);
  }
}

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
      console.log("ethAddress:", ethAddress, "\n", "Buffer address:", address);
      console.log("address length:", address.length);
      const result = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        result[i] = address[i];
      }
      const addressArray: number[] = Array.from(result);
      return addressArray;
    }

    function ethereumAddressToU8Array(address: string): number[] {
      const cleanAddress = address.startsWith("0x")
        ? address.slice(2)
        : address;
      const bytes = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        const byte = parseInt(cleanAddress.substr(i * 2, 2), 16);
        bytes[31 - i] = byte;
      }
      const addressArray: number[] = Array.from(bytes);
      return addressArray;
    }

    let id = new anchor.BN(4);
    let chainId = Buffer.from([4]);
    console.log("chainId buffer:", chainId);

    //pda
    //init_mapping_fee_config
    let [mappingFeeConfigAuthority, mappingFeeConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_mapping_fee_config")],
        pg.PROGRAM_ID
      );
    console.log(
      "mappingFeeConfigAuthority:",
      mappingFeeConfigAuthority.toString()
    );
    console.log("mappingFeeConfigBump:", mappingFeeConfigBump);

    //init_current_record_message
    let [recordMessageAuthority, recordMessageBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_current_record_message")],
        pg.PROGRAM_ID
      );
    console.log("recordMessageAuthority:", recordMessageAuthority.toString());
    console.log("recordMessageBump:", recordMessageBump);

    //save_dest_chain_Id
    let saveDestChainIdAccount = new web3.Keypair();
    console.log(
      "saveDestChainIdAccount:",
      saveDestChainIdAccount.publicKey.toBase58()
    );

    let dapp = ethereumAddressToU8Array(
      "0xaE67336f06B10fbbb26F31d31AbEA897290109B9"
    );
    let dapp2 = ethereumAddressToU8Array(
      "0xE3020Ac60f45842A747F6008390d0D28dDbBD98D"
    );
    let dapp3 = ethereumAddressToU8Array(
      "0xd1A48613D41E7BB2C68aD90D5fE5e7041ebA5111"
    );

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

    //init_native_token_trade_fee_config
    let native_molecular = new anchor.BN(5);
    let native_denominator = new anchor.BN(10);

    let symbol = "usdt";
    let tokenAddress = ethereumAddressToU8Array(
      "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    );

    let init_decimals = 6;
    let init_max_price = new anchor.BN(1000);

    //init_record_message
    async function InitCurrentRecordMessage() {
      try {
        const recordValid = await pg.program.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
        const valid = await recordValid.initState;
        console.log("valid:", valid);
      } catch (e) {
        const initRecordMessage = await pg.program.methods
          .initRecordMessage()
          .accounts({
            currentRecordMessage: recordMessageAuthority,
            vizing: vizingPadSettings,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initRecordMessage:${initRecordMessage}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(initRecordMessage);
      }
    }
    await InitCurrentRecordMessage();

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
    async function SetThisGasGlobal(
      thisGlobalBasePrice,
      thisDefaultGasLimit,
      thisAmountInThreshold,
      thisMolecular,
      thisDenominator
    ) {
      try {
        const setThisGasGlobal = await pg.program.methods
          .setThisGasGlobal(
            id,
            thisGlobalBasePrice,
            thisDefaultGasLimit,
            thisAmountInThreshold,
            thisMolecular,
            thisDenominator
          )
          .accounts({
            mappingFeeConfig: mappingFeeConfigAuthority,
            vizing: vizingPadSettings,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisGasGlobal:${setThisGasGlobal}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisGasGlobal);
      } catch (e) {
        console.log("SetThisGasGlobal error:", e);
      }
    }

    await SetThisGasGlobal(
      new_global_base_price,
      new_default_gas_limit,
      new_amount_in_threshold,
      molecular,
      denominator
    );

    //set_this_fee_config
    async function SetThisFeeConfig(
      thisChainId,
      thisBasePrice,
      thisReserve,
      thisMolecular,
      thisDenominator,
      thisMolecularDecimal,
      thisDenominatorDecimal
    ) {
      try {
        const setThisFeeConfig = await pg.program.methods
          .setThisFeeConfig(
            thisChainId,
            thisBasePrice,
            thisReserve,
            thisMolecular,
            thisDenominator,
            thisMolecularDecimal,
            thisDenominatorDecimal
          )
          .accounts({
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisFeeConfig tx:${setThisFeeConfig}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisFeeConfig);
      } catch (e) {
        console.log("SetThisFeeConfig error:", e);
      }
    }
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      molecular_decimal,
      denominator_decimal
    );

    //set_token_fee_config
    async function SetThisTokenFeeConfig() {
      try {
        const setThisTokenFeeConfig = await pg.program.methods
          .setThisTokenFeeConfig(id, molecular, denominator)
          .accounts({
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
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
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisDappPriceConfig tx:${setThisDappPriceConfig}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(setThisDappPriceConfig);

        const mappingFeeConfig =
          await pg.program.account.mappingFeeConfig.fetch(
            mappingFeeConfigAuthority
          );
        const this_dapp_date = mappingFeeConfig.dappConfigMappings[0].dapp;
        console.log("this_dapp_date:", this_dapp_date);
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
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisExchangeRate tx:${setThisExchangeRate}'`);
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
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
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
    let dapps = [dapp, dapp2];
    let tradeFeeConfig_dapps = [dapp, dapp];
    let tradeFeeConfig_destChainIds = [new anchor.BN(4), new anchor.BN(5)];
    let tradeFeeConfig_moleculars = [new anchor.BN(5), new anchor.BN(5)];
    let tradeFeeConfig_denominators = [new anchor.BN(10), new anchor.BN(10)];
    async function BatchSetThisTradeFeeConfigMap() {
      try {
        const batchSetThisTradeFeeConfigMap = await pg.program.methods
          .batchSetThisTradeFeeConfigMap(
            tradeFeeConfig_dapps,
            tradeFeeConfig_destChainIds,
            tradeFeeConfig_moleculars,
            tradeFeeConfig_denominators
          )
          .accounts({
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisTradeFeeConfigMap tx:${batchSetThisTradeFeeConfigMap}'`
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
    let diff_destChainIds = [new anchor.BN(4), new anchor.BN(5)];
    let diff_dapps = [dapp, dapp];
    let diff_base_prices = [new anchor.BN(1000), new anchor.BN(2000)];
    async function BatchSetThisDappPriceConfigInDiffChain() {
      try {
        const batchSetThisDappPriceConfigInDiffChain = await pg.program.methods
          .batchSetThisDappPriceConfigInDiffChain(
            diff_destChainIds,
            diff_dapps,
            diff_base_prices
          )
          .accounts({
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisDappPriceConfigInDiffChain tx:${batchSetThisDappPriceConfigInDiffChain}'`
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
    let DappPriceConfig_dapps = [dapp, dapp];
    let DappPriceConfig_base_prices = [
      new anchor.BN(1000),
      new anchor.BN(1000),
    ];
    async function BatchSetThisDappPriceConfigInSameChain() {
      try {
        const batchSetThisDappPriceConfigInSameChain = await pg.program.methods
          .batchSetThisDappPriceConfigInSameChain(
            id,
            DappPriceConfig_dapps,
            DappPriceConfig_base_prices
          )
          .accounts({
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(
          `batchSetThisDAppPriceConfigInSameChain tx:${batchSetThisDappPriceConfigInSameChain}'`
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
      const gasSystemGlobal = mappingFeeConfig.gasSystemGlobalMappings;
      const tradeFeeConfigMappings =
        await mappingFeeConfig.tradeFeeConfigMappings;
      let trade_fee_config_molecular =
        tradeFeeConfigMappings[0].molecular.toNumber();
      let trade_fee_config_denominator =
        tradeFeeConfigMappings[0].denominator.toNumber();
      let global_trade_fee_molecular = gasSystemGlobal[0].molecular.toNumber();
      let global_trade_fee_denominator =
        gasSystemGlobal[0].denominator.toNumber();
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
      const dappConfigMappings = mappingFeeConfig.dappConfigMappings;
      const gasSystemGlobal = mappingFeeConfig.gasSystemGlobalMappings;

      let gas_system_global_base_price =
        await gasSystemGlobal[0].globalBasePrice.toNumber();
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
      const gasSystemGlobal = mappingFeeConfig.gasSystemGlobalMappings;

      let gas_system_global_base_price =
        await gasSystemGlobal[0].globalBasePrice.toNumber();
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
      const feeConfigMappings = mappingFeeConfig.feeConfigMappings;
      const gasSystemGlobal = mappingFeeConfig.gasSystemGlobalMappings;
      let base_price;
      let fee;
      const feeConfigBasePrice = feeConfigMappings[0].basePrice.toNumber();
      const global_base_price = gasSystemGlobal[0].globalBasePrice.toNumber();
      const default_gas_limit = gasSystemGlobal[0].defaultGasLimit.toNumber();
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
      const gasSystemGlobal = mappingFeeConfig.gasSystemGlobalMappings;

      const token_amount_limit =
        gasSystemGlobal[0].amountInThreshold.toNumber();

      const feeConfigBasePrice = feeConfigMappings[0].basePrice.toNumber();
      const global_base_price = gasSystemGlobal[0].globalBasePrice.toNumber();
      const default_gas_limit = gasSystemGlobal[0].defaultGasLimit.toNumber();

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
          throw "price < dapp_base_price";
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
          output_amount_in = await ExactOutput(
            dest_chain_id,
            amount_out.toNumber()
          );
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
    await EstimateTotalFee(id, testAmountOut, newMessage);

    await SetThisFeeConfig(
      new anchor.BN(5),
      base_price,
      reserve,
      molecular,
      denominator,
      molecular_decimal,
      denominator_decimal
    );

    //batch_set_exchange_rate
    let batchExchangeRate_destChainIds = [new anchor.BN(4), new anchor.BN(5)];
    let batchExchangeRate_moleculars = [new anchor.BN(10), new anchor.BN(20)];
    let batchExchangeRate_denominators = [
      new anchor.BN(50),
      new anchor.BN(100),
    ];
    let molecular_decimals = Buffer.from([6, 6]);
    let denominator_decimals = Buffer.from([6, 6]);
    async function BatchSetThisExchangeRate() {
      try {
        const batchSetThisExchangeRate = await pg.program.methods
          .batchSetThisExchangeRate(
            batchExchangeRate_destChainIds,
            batchExchangeRate_moleculars,
            batchExchangeRate_denominators,
            molecular_decimals,
            denominator_decimals
          )
          .accounts({
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
      destChainid: id,
      additionParams: Buffer.alloc(0),
      message: message,
    };
    async function Launch(thisLaunchParams) {
      try {
        let launch = await pg.program.methods
          .launch(thisLaunchParams)
          .accounts({
            feePayer: user,
            messageAuthority: user,
            vizing: vizingPadSettings,
            feeCollector: feeReceiverKeyPair.publicKey,
            mappingFeeConfig: mappingFeeConfigAuthority,
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
    let thisTestValue = new anchor.BN(1000);
    let thisFee1 = await EstimateTotalFee(id, thisTestValue, message);
    let solBefore1 = await getSolBalance(user);
    await Launch(launchParams);
    let solAfter1 = await getSolBalance(user);
    let differ1 = solBefore1 - solAfter1;
    if (differ1 >= thisFee1) {
      console.log("launch1 success", differ1);
    } else {
      console.log("launch1 amount error", differ1);
    }

    //big number value launch
    let thisTestValue2 = new anchor.BN(1000000);
    let thisFee2 = await EstimateTotalFee(id, thisTestValue2, message);
    let solBefore2 = await getSolBalance(user);
    const newLaunchParams1 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: user,
      sender: user,
      value: thisTestValue2,
      destChainid: id,
      additionParams: Buffer.alloc(0),
      message: message,
    };
    await Launch(newLaunchParams1);
    let solafter2 = await getSolBalance(user);
    let differ2 = solBefore2 - solafter2;
    if (differ2 >= thisFee2) {
      console.log("launch2 success", differ2);
    } else {
      console.log("launch2 amount error", differ2);
    }

    //success random relayer
    // let newRelayer = new web3.Keypair();
    // console.log("newRelayer:", newRelayer.publicKey.toBase58());
    // const newLaunchParams2 = {
    //   erliestArrivalTimestamp: new anchor.BN(1),
    //   latestArrivalTimestamp: new anchor.BN(2),
    //   relayer: newRelayer.publicKey,
    //   sender: user,
    //   value: thisTestValue2,
    //   destChainid: id,
    //   additionParams: Buffer.alloc(0),
    //   message: message,
    // };
    // await Launch(newLaunchParams2);

    //error amount_in_threshold
    // let this_amount_in_threshold = new anchor.BN(100);
    // await SetThisGasGlobal(
    //   new_global_base_price,
    //   new_default_gas_limit,
    //   this_amount_in_threshold,
    //   molecular,
    //   denominator
    // );
    // await Launch(launchParams);
    // await SetThisGasGlobal(
    //   new_global_base_price,
    //   new_default_gas_limit,
    //   amount_in_threshold,
    //   molecular,
    //   denominator
    // );

    //error message
    // function encodeEthereumAddressTo40U8Array(ethAddress: string): number[] {
    //   const address = ethAddress.slice(2); // Remove the '0x' prefix
    //   const result = new Uint8Array(40);
    //   for (let i = 0; i < 40; i++) {
    //     let charAddressI = address[i].charCodeAt(0);
    //     result[i] = charAddressI;
    //   }

    //   const addressArray: number[] = Array.from(result);
    //   return addressArray;
    // }
    // let by40Dapp = encodeEthereumAddressTo40U8Array(
    //   "0xaE67336f06B10fbbb26F31d31AbEA897290109B9"
    // );
    // const errorDappMessage = {
    //   mode: 1,
    //   targetContract: by40Dapp,
    //   executeGasLimit: executeGasLimit,
    //   maxFeePerGas: maxFeePerGas,
    //   signature: Buffer.from("000000001"),
    // };
    // const newLaunchParams3 = {
    //   erliestArrivalTimestamp: new anchor.BN(1),
    //   latestArrivalTimestamp: new anchor.BN(2),
    //   relayer: newRelayer,
    //   sender: user,
    //   value: thisTestValue2,
    //   destChainid: id,
    //   additionParams: Buffer.alloc(0),
    //   message: errorDappMessage,
    // };
    // await Launch(newLaunchParams3);

    // //error invalid eth address
    // let invalidDapp = encodeEthereumAddressTo40U8Array(
    //   "0xAAA777733332222bb26F31d31AbEA897290109B9"
    // );
    // const errorDappMessage2 = {
    //   mode: 1,
    //   targetContract: invalidDapp,
    //   executeGasLimit: executeGasLimit,
    //   maxFeePerGas: maxFeePerGas,
    //   signature: Buffer.from("000000001"),
    // };
    // const newLaunchParams4 = {
    //   erliestArrivalTimestamp: new anchor.BN(1),
    //   latestArrivalTimestamp: new anchor.BN(2),
    //   relayer: newRelayer,
    //   sender: user,
    //   value: thisTestValue2,
    //   destChainid: id,
    //   additionParams: Buffer.alloc(0),
    //   message: errorDappMessage2,
    // };
    // await Launch(newLaunchParams4);

    //get
    // async function GetEstimateGas(amount_out, dest_chain_id, this_message) {
    //   try {
    //     const estimateGas = await pg.program.methods
    //       .estimateGas(amount_out, dest_chain_id, this_message)
    //       .accounts({

    //         mappingFeeConfig: mappingFeeConfigAuthority,
    //         currentRecordMessage: recordMessageAuthority,
    //       })
    //       .signers([signer])
    //       .rpc();
    //     console.log(`estimateGas tx:${estimateGas}'`);
    //     // Confirm transaction
    //     await pg.connection.confirmTransaction(estimateGas);
    //     const currentRecordMessage =
    //       await pg.program.account.currentRecordMessage.fetch(
    //         recordMessageAuthority
    //       );
    //     const estimateGasNumber =
    //       await currentRecordMessage.estimateGas.toNumber();
    //     console.log("estimateGasNumber:", estimateGasNumber);
    //   } catch (e) {
    //     console.log("estimateGas error:", e);
    //   }
    // }
    // await GetEstimateGas(testAmountOut, id, newMessage);

    // async function GetEstimateTotalFee(
    //   amount_out,
    //   dest_chain_id,
    //   this_message
    // ) {
    //   try {
    //     const estimateTotalFee = await pg.program.methods
    //       .estimateTotalFee(dest_chain_id, amount_out, this_message)
    //       .accounts({

    //         mappingFeeConfig: mappingFeeConfigAuthority,
    //         currentRecordMessage: recordMessageAuthority,
    //       })
    //       .signers([signer])
    //       .rpc();
    //     console.log(`estimateTotalFee tx:${estimateTotalFee}'`);
    //     // Confirm transaction
    //     await pg.connection.confirmTransaction(estimateTotalFee);
    //     const currentRecordMessage =
    //       await pg.program.account.currentRecordMessage.fetch(
    //         recordMessageAuthority
    //       );
    //     const estimateTotalFeeNumber =
    //       await currentRecordMessage.estimateTotalFee.toNumber();
    //     console.log("estimateTotalFeeNumber:", estimateTotalFeeNumber);
    //   } catch (e) {
    //     console.log("GetEstimateTotalFee error:", e);
    //   }
    // }
    // await GetEstimateTotalFee(testAmountOut, id, newMessage);

    const estimateTotalFeeMessage = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: 6,
      maxFeePerGas: 2000,
      signature: Buffer.from("000000001"),
    };

    // 将 executeGasLimit (u32) 转换为 number[]
    const executeGasLimitBuffer = Buffer.alloc(4); 
    executeGasLimitBuffer.writeUInt32LE(estimateTotalFeeMessage.executeGasLimit, 0);
    const executeGasLimitArray = Array.from(
      new Uint8Array(
        executeGasLimitBuffer,
        4
      )
    );
    console.log("executeGasLimitArray:",executeGasLimitArray);

    // 将 maxFeePerGas (u64) 转换为 number[]
    const maxFeePerGasBuffer = Buffer.alloc(8); 
    maxFeePerGasBuffer.writeBigUInt64LE(estimateTotalFeeMessage.maxFeePerGas, 0);
    const maxFeePerGasArray = Array.from(
      new Uint8Array(maxFeePerGasBuffer, 8)
    );
    console.log("maxFeePerGasArray:", maxFeePerGasArray);


    // 将 signature 转换为 number[]
    const signatureArray = Array.from(
      new Uint8Array(estimateTotalFeeMessage.signature)
    );
    console.log("signatureArray:", signatureArray);

    // 将 estimateTotalFeeMessageArray 中的各部分合并为一个数组
    const estimateTotalFeeMessageArray = [
      estimateTotalFeeMessage.mode, // mode 是单个数值
      ...estimateTotalFeeMessage.targetContract, // targetContract 需要转换为 number[]
      ...executeGasLimitArray, // executeGasLimit 是 number[]
      ...maxFeePerGasArray, // maxFeePerGas 是 number[]
      ...signatureArray, // signature 是 number[]
    ];

    // 打印结果
    console.log("Serialized Data:", estimateTotalFeeMessageArray);

    async function GetEstimateVizingGasFee(
      amount_out,
      dest_chain_id,
      addition_params,
      this_message
    ) {
      try {
        const estimateVizingGasFee = await pg.program.methods
          .estimateVizingGasFee(
            dest_chain_id,
            amount_out,
            addition_params,
            this_message
          )
          .accounts({
            mappingFeeConfig: mappingFeeConfigAuthority,
            currentRecordMessage: recordMessageAuthority,
          })
          .signers([signer])
          .rpc();
        console.log(`estimateVizingGasFee tx:${estimateVizingGasFee}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(estimateVizingGasFee);
        const currentRecordMessage =
          await pg.program.account.currentRecordMessage.fetch(
            recordMessageAuthority
          );
        const estimateVizingGasFeeNumber =
          await currentRecordMessage.estimateVizingGasFee.toNumber();
        console.log("estimateVizingGasFeeNumber:", estimateVizingGasFeeNumber);
      } catch (e) {
        console.log("GetEstimateTotalFee error:", e);
      }
    }
    let newAdditionParams = Buffer.from("0");
    await GetEstimateVizingGasFee(
      testAmountOut,
      id,
      newAdditionParams,
      estimateTotalFeeMessageArray
    );
  });
});
