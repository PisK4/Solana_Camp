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
const feeReceiverKeyPair = new web3.Keypair();
console.log("feeReceiverKeyPair:", feeReceiverKeyPair.publicKey.toBase58());

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

let relayer1 = anchor.web3.Keypair.fromSeed(
  Buffer.from(padStringTo32Bytes("trusted_relayer_1"))
);
let relayer2 = anchor.web3.Keypair.fromSeed(
  Buffer.from(padStringTo32Bytes("trusted_relayer_2"))
);
const trustedRelayerKeyPairs = [relayer1, relayer2];

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
      feeCollector: feeReceiverKeyPair.publicKey,
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

    function ethereumAddressToU8Array(address: string): number[] {
      const cleanAddress = address.startsWith("0x")
        ? address.slice(2)
        : address;
      const bytes = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        const byte = parseInt(cleanAddress.substr(i * 2, 2), 16);
        bytes[i] = byte;
      }
      const addressArray: number[] = Array.from(bytes);
      return addressArray;
    }

    let arbitrum_chain_id = new anchor.BN(1101);
    let arbitrum_maxPrice = new anchor.BN(100000);
    let arbitrum_destChainBasePrice = new anchor.BN(10000);
    let arbitrum_tradeLimit = new anchor.BN(500000000000); //500 sol limit
    let arbitrum_tradeFee = {
      molecular: new anchor.BN(1),
      denominator: new anchor.BN(2),
    };
    let arbitrum_DAppBasePrice = new anchor.BN(10000);
    let arbitrum_molecular_decimal = 123;
    let arbitrum_denominator_decimal = 6;

    const Uint256Params={
      high: new anchor.BN(0),
      low: new anchor.BN(10_000_000);
    };

    let vizing_chain_id = new anchor.BN(28516);
    let vizing_maxPrice = new anchor.BN(100000);
    let vizing_destChainBasePrice = new anchor.BN(10000);
    let vizing_tradeLimit = new anchor.BN(500000000000); //500 sol limit
    let vizing_tradeFee = {
      molecular: new anchor.BN(9995),
      denominator: new anchor.BN(10000),
    };
    let vizing_DAppBasePrice = new anchor.BN(10000);

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

    let dapp = ethereumAddressToU8Array(
      "0x1b06677de21ce8B3C8970dAd08970A04DaF99756"
    );
    let dapp2 = ethereumAddressToU8Array(
      "0x922eC41D745372D14204a45A2b68EebaFF39AE77"
    );
    let dapp3 = ethereumAddressToU8Array(
      "0xd1A48613D41E7BB2C68aD90D5fE5e7041ebA5111"
    );

    //initializeVizingPad
    async function InitializeVizingPad() {
      try {
        const vizingPadAccount =
          await pg.program.account.vizingPadConfigs.fetch(vizingPadSettings);
        console.log("vizingPadAccount:", vizingPadAccount.owner.toBase58());
      } catch (e) {
        const tx = await pg.program.methods
          .initializeVizingPad(initParams)
          .accounts({
            vizingPadConfig: vizingPadSettings,
            vizingPadAuthority: vizingAuthority,
            payer: user,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        console.log(`initializeVizingPad: ${tx}`);
      }
    }
    await InitializeVizingPad();

    let base_price = arbitrum_destChainBasePrice;
    let reserve = new anchor.BN(1000000000);
    let molecular = arbitrum_tradeFee.molecular;
    let denominator = arbitrum_tradeFee.denominator;
    let molecular_decimal = 6;
    let denominator_decimal = 6;

    let global_base_price = new anchor.BN(1000);
    let default_gas_limit = new anchor.BN(10000);
    let amount_in_threshold = arbitrum_tradeLimit;
    //init_gas_global
    async function InitGasGlobal(
      thisChainId,
      thisGlobalBasePrice,
      thisDefaultGasLimit,
      thisAmountInThreshold,
      thisMolecular,
      thisDenominator
    ) {
      try {
        const mappingFeeConfig =
          await pg.program.account.mappingFeeConfig.fetch(
            mappingFeeConfigAuthority
          );
        const gasSystemGlobalMappings =
          mappingFeeConfig.gasSystemGlobalMappings;
      } catch (e) {
        const initGasGlobal = await pg.program.methods
          .initGasGlobal(
            thisChainId,
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
        console.log(`initGasGlobal:${initGasGlobal}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(initGasGlobal);
      }
    }
    await InitGasGlobal(
      arbitrum_chain_id,
      global_base_price,
      default_gas_limit,
      amount_in_threshold,
      molecular,
      denominator
    );

    //init_native_token_trade_fee_config
    let native_molecular = new anchor.BN(5);
    let native_denominator = new anchor.BN(10);

    let symbol = "usdt";
    let tokenAddress = ethereumAddressToU8Array(
      "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    );

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
    const OwnerManagementParams = {
      owner: user,
      feeCollector: feeReceiverKeyPair.publicKey,
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
          .modifySettings(OwnerManagementParams)
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
        let vizingPadAccount = await pg.program.account.vizingPadConfigs.fetch(
          vizingPadSettings
        );
        console.log(
          "vizingPadAccount:",
          vizingPadAccount.feeCollector.toBase58()
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
    let new_global_base_price = arbitrum_destChainBasePrice;
    let new_default_gas_limit = new anchor.BN(10);
    let new_amount_in_threshold = arbitrum_tradeLimit;
    async function SetThisGasGlobal(
      thisChainId,
      thisGlobalBasePrice,
      thisDefaultGasLimit,
      thisAmountInThreshold,
      thisMolecular,
      thisDenominator
    ) {
      try {
        const setThisGasGlobal = await pg.program.methods
          .setThisGasGlobal(
            thisChainId,
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
      arbitrum_chain_id,
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
    // await SetThisFeeConfig(
    //   arbitrum_chain_id,
    //   base_price,
    //   reserve,
    //   molecular,
    //   denominator,
    //   molecular_decimal,
    //   denominator_decimal
    // );

    //set_token_fee_config
    async function SetThisTokenFeeConfig() {
      try {
        const setThisTokenFeeConfig = await pg.program.methods
          .setThisTokenFeeConfig(arbitrum_chain_id, molecular, denominator)
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
    // await SetThisTokenFeeConfig();

    async function SetThisDappPriceConfig() {
      try {
        const setThisDappPriceConfig = await pg.program.methods
          .setThisDappPriceConfig(
            arbitrum_chain_id,
            dapp,
            molecular,
            denominator,
            base_price
          )
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
      } catch (e) {
        console.log("SetThisDappPriceConfig error:", e);
      }
    }
    // await SetThisDappPriceConfig();

    //set_exchange_rate
    async function SetThisExchangeRate() {
      try {
        const setThisExchangeRate = await pg.program.methods
          .setThisExchangeRate(
            arbitrum_chain_id,
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
    // await SetThisExchangeRate();

    //batch_set_token_fee_config
    let destChainIds = [arbitrum_chain_id];
    let moleculars = [arbitrum_tradeFee.molecular];
    let denominators = [arbitrum_tradeFee.denominator];
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
    // await BatchSetThisTokenFeeConfig();

    //batch_set_this_trade_fee_config_map
    let tradeFeeConfig_dapps = [dapp];
    let tradeFeeConfig_destChainIds = [arbitrum_chain_id];
    let tradeFeeConfig_moleculars = [arbitrum_tradeFee.molecular];
    let tradeFeeConfig_denominators = [arbitrum_tradeFee.denominator];
    let base_price_group = [new anchor.BN(10000)];
    async function BatchSetThisTradeFeeConfigMap() {
      try {
        const batchSetThisTradeFeeConfigMap = await pg.program.methods
          .batchSetThisTradeFeeConfigMap(
            tradeFeeConfig_dapps,
            tradeFeeConfig_destChainIds,
            tradeFeeConfig_moleculars,
            tradeFeeConfig_denominators,
            base_price_group
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
    // await BatchSetThisTradeFeeConfigMap();

    //batch_set_this_dapp_price_config_in_diff_chain
    let base_prices = [arbitrum_destChainBasePrice];
    let diff_destChainIds = [arbitrum_chain_id];
    let diff_dapps = [dapp];
    let diff_base_prices = [arbitrum_DAppBasePrice];
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
    // await BatchSetThisDappPriceConfigInDiffChain();

    //batch_set_this_dapp_price_config_in_same_chain
    let DappPriceConfig_dapps = [dapp];
    let DappPriceConfig_base_prices = [arbitrum_DAppBasePrice];
    async function BatchSetThisDappPriceConfigInSameChain() {
      try {
        const batchSetThisDappPriceConfigInSameChain = await pg.program.methods
          .batchSetThisDappPriceConfigInSameChain(
            arbitrum_chain_id,
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
    // await BatchSetThisDappPriceConfigInSameChain();

    //get
    async function GetDappBasePrice(dest_chain_id, chain_base_price, dapp) {
      let dapp_base_price;
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const tradeFeeConfigMappings = mappingFeeConfig.tradeFeeConfigMappings;
      const tradeFeeConfigMapping = tradeFeeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const tradeFeeConfigMappingResult = tradeFeeConfigMapping
        ? tradeFeeConfigMapping
        : 0;
      let isTargetInDapps = false;
      if (tradeFeeConfigMappingResult != 0) {
        isTargetInDapps = tradeFeeConfigMapping.dapps.some(
          (contract) => contract.toString() === dapp.toString()
        );
      }

      let dapp_config_value = 0;
      if (isTargetInDapps) {
        dapp_config_value = tradeFeeConfigMapping.value.toNumber();
      }
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
      const feeConfigMappings = mappingFeeConfig.feeConfigMappings;
      const feeConfigMapping = feeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const feeConfigMappingResult = feeConfigMapping ? feeConfigMapping : 0;
      let fee_config_molecular_decimal = 0;
      let fee_config_denominator_decimal = 0;
      let fee_config_molecular = 0;
      let fee_config_denominator = 0;
      if (feeConfigMappingResult != 0) {
        fee_config_molecular_decimal = feeConfigMapping.molecularDecimal;
        fee_config_denominator_decimal = feeConfigMapping.denominatorDecimal;
        fee_config_molecular = feeConfigMapping.molecular.toNumber();
        fee_config_denominator = feeConfigMapping.denominator.toNumber();
      }

      let this_amount_out;
      if (fee_config_molecular_decimal != fee_config_denominator_decimal) {
        if (fee_config_molecular_decimal > fee_config_denominator_decimal) {
          this_amount_out =
            amount_out.low.toNumer() /
            (10 ^
              (fee_config_molecular_decimal - fee_config_denominator_decimal));
        } else {
          this_amount_out =
            amount_out.low.toNumer() /
            (10 ^
              (fee_config_denominator_decimal - fee_config_molecular_decimal));
        }
      } else {
        this_amount_out = amount_out.low.toNumer();
      }

      let amount_in =
        (this_amount_out * fee_config_denominator) /
        fee_config_molecular;
      console.log("ExactOutput:", amount_in);
      return amount_in;
    }

    async function ComputeTradeFee1(dest_chain_id, amount_out) {
      let computeTradeFee1;
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const tradeFeeMappings = mappingFeeConfig.tradeFeeMappings;
      const gasSystemGlobalMappings = mappingFeeConfig.gasSystemGlobalMappings;
      let tradeFee_molecular = 0;
      let tradeFee_denominator = 0;
      let gasSystemGlobal_molecular = 0;
      let gasSystemGlobal_denominator = 0;
      const gasSystemGlobalMapping = gasSystemGlobalMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const tradeFeeMapping = tradeFeeMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const gasSystemGlobalMappingResult = gasSystemGlobalMapping
        ? gasSystemGlobalMapping
        : 0;
      const tradeFeeMappingResult = tradeFeeMapping ? tradeFeeMapping : 0;
      if (gasSystemGlobalMappingResult != 0) {
        gasSystemGlobal_molecular = gasSystemGlobalMapping.molecular.toNumber();
        gasSystemGlobal_denominator =
          gasSystemGlobalMapping.denominator.toNumber();
      }

      if (tradeFeeMappingResult != 0) {
        tradeFee_molecular = tradeFeeMapping.molecular.toNumber();
        tradeFee_denominator = tradeFeeMapping.denominator.toNumber();
      }
      if (tradeFee_denominator == 0) {
        computeTradeFee1 =
          (amount_out.low.toNumer() * gasSystemGlobal_molecular) /
          gasSystemGlobal_denominator;
      } else {
        if (tradeFee_molecular != 0 && tradeFee_denominator != 0) {
          return 0;
        } else {
          computeTradeFee1 =
            (amount_out.low.toNumer() * tradeFee_molecular) / tradeFee_denominator;
        }
      }
      return computeTradeFee1;
    }

    async function ComputeTradeFee2(
      target_contract,
      dest_chain_id,
      amount_out
    ) {
      const isNonZero = target_contract.some((byte) => byte !== 0);
      let computeTradeFee2;
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const tradeFeeConfigMappings = mappingFeeConfig.tradeFeeConfigMappings;
      let trade_fee_config_molecular = 0;
      let trade_fee_config_denominator = 0;
      const tradeFeeConfigMapping = tradeFeeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const tradeFeeConfigMappingResult = tradeFeeConfigMapping
        ? tradeFeeConfigMapping
        : 0;
      let isTargetInDapps = false;
      if (tradeFeeConfigMappingResult != 0) {
        isTargetInDapps = tradeFeeConfigMapping.dapps.some(
          (contract) => contract.toString() === dapp.toString()
        );
      }
      if (isTargetInDapps) {
        trade_fee_config_molecular = tradeFeeConfigMapping.molecular.toNumber();
        trade_fee_config_denominator =
          tradeFeeConfigMapping.denominator.toNumber();
      }
      if (trade_fee_config_denominator != 0 && isNonZero) {
        if (
          trade_fee_config_molecular != 0 &&
          trade_fee_config_denominator != 0
        ) {
          computeTradeFee2 =
            (amount_out.low.toNumer() * trade_fee_config_molecular) /
            trade_fee_config_denominator;
        } else {
          return 0;
        }
      } else {
        computeTradeFee2 = await ComputeTradeFee1(dest_chain_id, amount_out.low.toNumer());
      }
      console.log("ComputeTradeFee2:", computeTradeFee2);
      return computeTradeFee2;
    }

    async function EstimatePrice1(target_contract, dest_chain_id) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const gasSystemGlobalMappings = mappingFeeConfig.gasSystemGlobalMappings;
      const tradeFeeConfigMappings = mappingFeeConfig.tradeFeeConfigMappings;
      const gasSystemGlobalMapping = gasSystemGlobalMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const tradeFeeConfigMapping = tradeFeeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const tradeFeeConfigMappingResult = tradeFeeConfigMapping
        ? tradeFeeConfigMapping
        : 0;
      let isTargetInDapps = false;
      if (tradeFeeConfigMappingResult != 0) {
        isTargetInDapps = tradeFeeConfigMapping.dapps.some(
          (contract) => contract.toString() === target_contract.toString()
        );
      }
      let dapp_config_value = 0;
      if (isTargetInDapps) {
        dapp_config_value = tradeFeeConfigMapping.value.toNumber();
      }

      let gas_system_global_base_price =
        gasSystemGlobalMapping.globalBasePrice.toNumber();
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
      const gasSystemGlobalMappings = mappingFeeConfig.gasSystemGlobalMappings;

      const gasSystemGlobalMapping = gasSystemGlobalMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const feeConfigMapping = feeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );

      const gasSystemGlobalMappingResult = gasSystemGlobalMapping
        ? gasSystemGlobalMapping
        : 0;
      const feeConfigMappingResult = feeConfigMapping ? feeConfigMapping : 0;

      let gas_system_global_base_price = 0;
      let fee_config_base_price = 0;
      if (gasSystemGlobalMappingResult != 0) {
        gas_system_global_base_price =
          gasSystemGlobalMapping.globalBasePrice.toNumber();
      }
      if (feeConfigMappingResult != 0) {
        fee_config_base_price = feeConfigMapping.basePrice.toNumber();
      }
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
      const feeConfigMappings = mappingFeeConfig.feeConfigMappings;
      const feeConfigMapping = feeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const feeConfigMappingResult = feeConfigMapping ? feeConfigMapping : 0;
      let fee_config_molecular_decimal = 0;
      let fee_config_denominator_decimal = 0;
      let fee_config_molecular = 0;
      let fee_config_denominator=0;
      if (feeConfigMappingResult != 0) {
        fee_config_molecular_decimal = feeConfigMapping.molecularDecimal;
        fee_config_denominator_decimal = feeConfigMapping.denominatorDecimal;
        fee_config_molecular = feeConfigMapping.molecular.toNumber();
        fee_config_denominator = feeConfigMapping.denominator.toNumber();
      }
      let this_amount_in;
      if (fee_config_molecular_decimal != fee_config_denominator_decimal) {
        if (fee_config_molecular_decimal > fee_config_denominator_decimal) {
          this_amount_in =
            amount_in.low.toNumer() *
            (10 ^
              (fee_config_molecular_decimal - fee_config_denominator_decimal));
        } else {
          this_amount_in =
            amount_in.low.toNumer() /
            (10 ^
              (fee_config_denominator_decimal - fee_config_molecular_decimal));
        }
      } else {
        this_amount_in = amount_in.low.toNumer();
      }
      let amount_out =
        (this_amount_in * fee_config_molecular) /
        fee_config_denominator;
      console.log("ExactInput:", amount_out);
      return amount_out;
    }

    let testAmountOut = new anchor.BN(100000000);
    const testExecuteGasLimit = new anchor.BN(10);
    const newMessage = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: testExecuteGasLimit,
      maxFeePerGas: arbitrum_maxPrice,
      signature: Buffer.from("transfer from alice to bob 10 usdt"),
    };
    async function EstimateGas(amount_out, dest_chain_id, this_message) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const feeConfigMappings = mappingFeeConfig.feeConfigMappings;
      const gasSystemGlobalMappings = mappingFeeConfig.gasSystemGlobalMappings;
      const feeConfigMapping = feeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const gasSystemGlobalMapping = gasSystemGlobalMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const feeConfigMappingResult = feeConfigMapping ? feeConfigMapping : 0;
      const gasSystemGlobalMappingResult = gasSystemGlobalMapping
        ? gasSystemGlobalMapping
        : 0;

      let base_price;
      let fee;
      let feeConfigBasePrice = 0;
      let global_base_price = 0;
      let default_gas_limit = 0;
      let fee_config_molecular = 0;
      if (feeConfigMappingResult != 0) {
        feeConfigBasePrice = feeConfigMapping.basePrice.toNumber();
        fee_config_molecular = feeConfigMapping.molecular.toNumber();
      }
      if (gasSystemGlobalMappingResult != 0) {
        global_base_price = gasSystemGlobalMapping.globalBasePrice.toNumber();
        default_gas_limit = gasSystemGlobalMapping.defaultGasLimit.toNumber();
      }
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

      if (amount_out.low.toNumer() > 0) {
        let output_amount_in;
        if (fee_config_molecular != 0) {
          output_amount_in = await ExactOutput(dest_chain_id, amount_out.low.toNumer());
        }

        let trade_fee2 = await ComputeTradeFee2(
          this_message.targetContract,
          dest_chain_id,
          output_amount_in
        );
        fee += trade_fee2;
      }
      console.log("EstimateGas fee:", fee);
      return fee;
    }
    // await EstimateGas(testAmountOut, arbitrum_chain_id, newMessage);

    async function EstimateTotalFee(dest_chain_id, amount_out, this_message) {
      const mappingFeeConfig = await pg.program.account.mappingFeeConfig.fetch(
        mappingFeeConfigAuthority
      );
      const feeConfigMappings = mappingFeeConfig.feeConfigMappings;
      const gasSystemGlobalMappings = mappingFeeConfig.gasSystemGlobalMappings;

      const feeConfigMapping = feeConfigMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const gasSystemGlobalMapping = gasSystemGlobalMappings.find(
        (mapping) => mapping.key.toNumber() === dest_chain_id.toNumber()
      );
      const feeConfigMappingResult = feeConfigMapping ? feeConfigMapping : 0;
      const gasSystemGlobalMappingResult = gasSystemGlobalMapping
        ? gasSystemGlobalMapping
        : 0;
      let feeConfigBasePrice = 0;
      let token_amount_limit = 0;
      let global_base_price = 0;
      let default_gas_limit = 0;
      let fee_config_molecular = 0;
      if (feeConfigMappingResult != 0) {
        feeConfigBasePrice = feeConfigMapping.basePrice.toNumber();
        fee_config_molecular = feeConfigMapping.molecular.toNumber();
      }
      if (gasSystemGlobalMappingResult != 0) {
        token_amount_limit =
          gasSystemGlobalMapping.amountInThreshold.toNumber();
        global_base_price = gasSystemGlobalMapping.globalBasePrice.toNumber();
        default_gas_limit = gasSystemGlobalMapping.defaultGasLimit.toNumber();
      }

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

      let output_amount_in = amount_out.low.toNumer();
      let finalFee;
      if (amount_out.low.toNumber() > 0) {
        if (fee_config_molecular != 0) {
          output_amount_in = await ExactOutput(
            dest_chain_id,
            amount_out.low.toNumer()
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
    // await EstimateTotalFee(arbitrum_chain_id, testAmountOut, newMessage);

    async function EstimateVizingGasFee(
      value,
      dest_chain_id,
      _addition_params,
      thisMessage
    ) {
      await EstimateGas(value, dest_chain_id, thisMessage);
    }

    // await SetThisFeeConfig(
    //   new anchor.BN(5),
    //   base_price,
    //   reserve,
    //   molecular,
    //   denominator,
    //   molecular_decimal,
    //   denominator_decimal
    // );

    //batch_set_exchange_rate
    let batchExchangeRate_destChainIds = [arbitrum_chain_id];
    let batchExchangeRate_moleculars = [arbitrum_tradeFee.molecular];
    let batchExchangeRate_denominators = [arbitrum_tradeFee.denominator];
    let molecular_decimals = Buffer.from([6]);
    let denominator_decimals = Buffer.from([6]);
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
    // await BatchSetThisExchangeRate();

    // launch
    const executeGasLimit = new anchor.BN(10);
    const maxFeePerGas = arbitrum_maxPrice;
    let launchRelayer = ethereumAddressToU8Array(
      "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    );

    let testDapp = Buffer.from("0xdAC17F958D2ee523a2206206994597C13D831ec7");

    const message = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice"),
    };

    let launch1Value = Uint256Params; // 0.01 sol

    const launchParams = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: launch1Value, // 0.01 sol
      destChainid: arbitrum_chain_id,
      additionParams: Buffer.alloc(0),
      message: message,
    };
    let feeCollector = feeReceiverKeyPair.publicKey;

    //forecast
    // await EstimateTotalFee(arbitrum_chain_id, launch1Value, message);
    // await EstimateVizingGasFee(
    //   launch1Value,
    //   arbitrum_chain_id,
    //   Buffer.alloc(0),
    //   message
    // );

    async function Launch(
      thisLaunchParams,
      thisVizingPadSettings,
      thisFeeCollector,
      thisMappingFeeConfig
    ) {
      try {
        let launch = await pg.program.methods
          .launch(thisLaunchParams)
          .accounts({
            vizingAppFeePayer: user,
            messageAuthority: user,
            vizing: thisVizingPadSettings,
            feeCollector: thisFeeCollector,
            mappingFeeConfig: thisMappingFeeConfig,
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
    await Launch(
      launchParams,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    //test only set gas_global
    let nullChainId = new anchor.BN(6);
    await SetThisGasGlobal(
      nullChainId,
      new_global_base_price,
      new_default_gas_limit,
      new_amount_in_threshold,
      molecular,
      denominator
    );
    //big number value launch
    //mode1
    let thisTestValue2 = new anchor.BN(1000000);
    const testMessage1 = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice do mode 1"),
    };
    const newLaunchParams1 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: testMessage1,
    };

    const Uint256Params2={
      high: new anchor.BN(0),
      low: new anchor.BN(2);
    };
    const testLaunchParams = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: Uint256Params2,
      destChainid: arbitrum_chain_id,
      additionParams: Buffer.alloc(0),
      message: message,
    };

    //test molecular_decimal=125,denominator_decimal=8
    console.log("test molecular_decimal=125,denominator_decimal=8:");
    await SetThisFeeConfig(
      arbitrum_chain_id,
      base_price,
      reserve,
      molecular,
      denominator,
      125,
      8
    );
    await Launch(
      testLaunchParams,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=8,denominator_decimal=125
    console.log("test molecular_decimal=8,denominator_decimal=125:");
    await SetThisFeeConfig(
      arbitrum_chain_id,
      base_price,
      reserve,
      molecular,
      denominator,
      8,
      125
    );
    await Launch(
      testLaunchParams,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=0,denominator_decimal=0
    console.log("test molecular_decimal=0,denominator_decimal=0:");
    await SetThisFeeConfig(
      arbitrum_chain_id,
      base_price,
      reserve,
      molecular,
      denominator,
      0,
      0
    );
    await Launch(
      testLaunchParams,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=0,denominator_decimal=9
    console.log("test molecular_decimal=0,denominator_decimal=9:");
    await SetThisFeeConfig(
      arbitrum_chain_id,
      base_price,
      reserve,
      molecular,
      denominator,
      0,
      9
    );
    await Launch(
      testLaunchParams,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=9,denominator_decimal=0
    console.log("test molecular_decimal=9,denominator_decimal=0:");
    await SetThisFeeConfig(
      arbitrum_chain_id,
      base_price,
      reserve,
      molecular,
      denominator,
      9,
      0
    );
    await Launch(
      testLaunchParams,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    
    const testLaunchParams2 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: Uint256Params2, 
      destChainid: arbitrum_chain_id,
      additionParams: Buffer.alloc(0),
      message: message,
    };

    //test molecular_decimal=9,denominator_decimal=18
    console.log("test molecular_decimal=9,denominator_decimal=18:");
    await SetThisFeeConfig(
      arbitrum_chain_id,
      base_price,
      reserve,
      molecular,
      denominator,
      9,
      18
    );
    await Launch(
      testLaunchParams2,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    const Uint256Params3={
      high: new anchor.BN(0),
      low: new anchor.BN(1000_000_000),
    };
    const testLaunchParams3 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: Uint256Params3, //1 sol
      destChainid: arbitrum_chain_id,
      additionParams: Buffer.alloc(0),
      message: message,
    };

    //test molecular_decimal=18,denominator_decimal=9
    console.log("test molecular_decimal=18,denominator_decimal=9:");
    await SetThisFeeConfig(
      arbitrum_chain_id,
      base_price,
      reserve,
      molecular,
      denominator,
      18,
      9
    );
    await Launch(
      testLaunchParams3,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    /** 
    await Launch(newLaunchParams1,vizingPadSettings,feeCollector,mappingFeeConfigAuthority);

    //mode 2
    const testMessage2 = {
      mode: 2,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice do mode 2"),
    };
    const newLaunchParams2 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: testMessage2,
    };
    await Launch(newLaunchParams2,vizingPadSettings,feeCollector,mappingFeeConfigAuthority);
    let this_fee_mode2 = await EstimateTotalFee(
      nullChainId,
      thisTestValue2,
      testMessage2
    );
    console.log("this_fee_mode2:",this_fee_mode2);

    //mode 3
    const testMessage3 = {
      mode: 3,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice do mode 3"),
    };
    const newLaunchParams3 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: testMessage3,
    };
    await Launch(newLaunchParams3,vizingPadSettings,feeCollector,mappingFeeConfigAuthority);
    let this_fee_mode3 = await EstimateTotalFee(
      nullChainId,
      thisTestValue2,
      testMessage3
    );
    console.log("this_fee_mode3:",this_fee_mode3);

    //mode 4
    const testMessage4 = {
      mode: 4,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice do mode 4"),
    };
    const newLaunchParams4 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: testMessage4,
    };
    await Launch(newLaunchParams4,vizingPadSettings,feeCollector,mappingFeeConfigAuthority);
    let this_fee_mode4 = await EstimateTotalFee(
      nullChainId,
      thisTestValue2,
      testMessage4
    );
    console.log("this_fee_mode4:",this_fee_mode4);
    */

    /** 
    //any message and dapp
    const errorDappMessage = {
      mode: 1,
      targetContract: Buffer.from("0xaE67336f06B10fbbb26F31d31AbEA897290109B9"),
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from(""),
    };
    const errLaunchParams3 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: errorDappMessage,
    };
    await Launch(
      errLaunchParams3,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );

    //error non fee_collector
    let newNonFeeCollector = new web3.Keypair();
    console.log("newNonFeeCollector:", newNonFeeCollector.publicKey.toBase58());
    const errLaunchParams1 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: message,
    };
    await Launch(
      errLaunchParams1,
      vizingPadSettings,
      newNonFeeCollector.publicKey,
      mappingFeeConfigAuthority
    );
    console.log("error non fee_collector");

    //error non vizingPadSettings
    let newVizingPadSettings = new web3.Keypair();
    console.log(
      "newVizingPadSettings:",
      newVizingPadSettings.publicKey.toBase58()
    );
    await Launch(
      errLaunchParams1,
      newVizingPadSettings.publicKey,
      feeCollector,
      mappingFeeConfigAuthority
    );
    console.log("error non vizingPadSettings");

    //error non mappingFeeConfigAuthority
    let newMappingFeeConfigAuthority = new web3.Keypair();
    console.log(
      "newMappingFeeConfigAuthority:",
      newMappingFeeConfigAuthority.publicKey.toBase58()
    );
    await Launch(
      errLaunchParams1,
      vizingPadSettings,
      feeCollector,
      newMappingFeeConfigAuthority.publicKey
    );
    console.log("error non newMappingFeeConfigAuthority");

    // error over amount_in_threshold
    let errValue = new anchor.BN(500000000001); //current limit=500000000000
    const errLaunchParams2 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: errValue,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: message,
    };
    await Launch(
      errLaunchParams2,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );
    console.log("error over amount_in_threshold");

    //error price < dapp_base_price
    let invalidPrice = new anchor.BN(999); //dapp_base_price=1000
    const errorPriceMessage = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: invalidPrice,
      signature: Buffer.from("000000001"),
    };
    const errLaunchParams4 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: errorPriceMessage,
    };
    await Launch(
      errLaunchParams4,
      vizingPadSettings,
      feeCollector,
      mappingFeeConfigAuthority
    );
    console.log("error price < dapp_base_price");

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
      mode: Buffer.from([1]), // u8
      targetContract: Buffer.from(dapp), // [u8; 32]
      executeGasLimit: new anchor.BN(10000), // u32
      maxFeePerGas: arbitrum_maxPrice, // u64
      signature: Buffer.from("transfer from alice to bob"), // Buffer
    };

    console.log("estimateTotalFeeMessage:", estimateTotalFeeMessage);

    const executeGasLimitBytes = Buffer.from(executeGasLimit.toArray("le", 4)); // u32
    const maxFeePerGasBytes = Buffer.from(maxFeePerGas.toArray("le", 8)); // u64

    const serializedDataArray = [
      ...estimateTotalFeeMessage.mode,
      ...estimateTotalFeeMessage.targetContract,
      ...executeGasLimitBytes,
      ...maxFeePerGasBytes,
      ...estimateTotalFeeMessage.signature,
    ];
    let bufferMessage = Buffer.from(serializedDataArray);
    console.log("bufferMessage:", bufferMessage);
    // const serializedDataBuffer = Buffer.concat([
    //     estimateTotalFeeMessage.mode,               // 1 byte
    //     estimateTotalFeeMessage.targetContract,     // 32 bytes
    //     executeGasLimitBytes,                       // 4 bytes
    //     maxFeePerGasBytes,                          // 8 bytes
    //     estimateTotalFeeMessage.signature           // Signature (Buffer size varies)
    // ]);
    // const serializedData = Array.from(serializedDataBuffer);

    async function GetEstimateVizingGasFee1(
      amount_out,
      dest_chain_id,
      addition_params,
      this_message
    ) {
      try {
        const estimateVizingGasFee1 = await pg.program.methods
          .estimateVizingGasFee1(
            amount_out,
            dest_chain_id,
            addition_params,
            this_message
          )
          .accounts({
            mappingFeeConfig: mappingFeeConfigAuthority,
            currentRecordMessage: recordMessageAuthority,
          })
          .signers([signer])
          .rpc();
        console.log(`estimateVizingGasFee1 tx:${estimateVizingGasFee1}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(estimateVizingGasFee1);
        const currentRecordMessage =
          await pg.program.account.currentRecordMessage.fetch(
            recordMessageAuthority
          );
        const estimateVizingGasFee1Number =
          await currentRecordMessage.estimateVizingGasFee.toNumber();
        console.log(
          "estimateVizingGasFee1Number:",
          estimateVizingGasFee1Number
        );
      } catch (e) {
        console.log("GetEstimateTotalFee1 error:", e);
      }
    }
    let newAdditionParams = Buffer.from("abc");
    await GetEstimateVizingGasFee1(
      testAmountOut,
      arbitrum_chain_id,
      newAdditionParams,
      bufferMessage
    );

    //remove
    async function RemoveTradeFeeDapp(this_chain_id, thisDapp) {
      try {
        const removeTradeFeeDapp = await pg.program.methods
          .removeTradeFeeDapp(this_chain_id, thisDapp)
          .accounts({
            vizing: vizingPadSettings,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`removeTradeFeeDapp tx:${removeTradeFeeDapp}'`);
        // Confirm transaction
        await pg.connection.confirmTransaction(removeTradeFeeDapp);
        const mappingFeeConfig =
          await pg.program.account.mappingFeeConfig.fetch(
            mappingFeeConfigAuthority
          );
        const tradeFeeConfigDapps =
          mappingFeeConfig.tradeFeeConfigMappings[0].dapps;
        console.log("tradeFeeConfigDapps:", tradeFeeConfigDapps);
      } catch (e) {
        console.log("RemoveTradeFeeDapp error:", e);
      }
    }
    await RemoveTradeFeeDapp(arbitrum_chain_id, dapp);
    */
  });
});
