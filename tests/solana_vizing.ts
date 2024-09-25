import * as anchor from "@coral-xyz/anchor";
import { Program, Coder } from "@coral-xyz/anchor";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

import { VizingCore } from "../target/types/vizing_core";
import { VizingApp } from "../target/types/vizing_app";
import { VizingAppMock } from "../target/types/vizing_app_mock";

function padStringTo32Bytes(str: string): Buffer {
  const buffer = Buffer.alloc(32);
  buffer.write(str);
  return buffer;
}

describe("Vizing Test", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const vizingProgram = anchor.workspace.VizingCore as Program<VizingCore>;
  const VizingPadConfigsSeed = Buffer.from("Vizing_Pad_Settings_Seed");
  const vizingAuthoritySeed = Buffer.from("Vizing_Authority_Seed");
  const vizingAppConfigSeed = Buffer.from("Vizing_App_Config_Seed");
  const vizingAppSolReceiverSeed = Buffer.from("Vizing_App_Sol_Receiver_Seed");
  const vizingFeeRouterSeed = Buffer.from("Vizing_Fee_Router_Seed");
  const vizingMessageAuthoritySeed = Buffer.from(
    "Vizing_Message_Authority_Seed"
  );

  let PROGRAM_ID = vizingProgram.programId;

  let mappingFeeConfigAuthority;
  let mappingFeeConfigBump;

  let recordMessageAuthority;
  let recordMessageBump;

  let vizingPadConfigs: anchor.web3.PublicKey;
  let vizingAuthority: anchor.web3.PublicKey;
  let vizingAppConfig: anchor.web3.PublicKey;
  let vizingFeeRouter: anchor.web3.PublicKey;
  let vizingMessageAuthority: anchor.web3.PublicKey;

  const feeCollectorKeyPair = anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("fee_collector"))
  );

  const feePayerKeyPair = anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("fee_payer"))
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

  let id = new anchor.BN(4);
  let id2 = new anchor.BN(5);
  let chainId = Buffer.from([4]);
  let chainId2 = Buffer.from([5]);
  console.log("chainId buffer:", chainId, chainId2);
  let global_base_price = new anchor.BN(500);
  let reserve = new anchor.BN(1000000);
  let molecular = new anchor.BN(5);
  let denominator = new anchor.BN(10);
  let default_gas_limit = new anchor.BN(100);
  let amount_in_threshold = new anchor.BN(100_000_000_000); //100 sol
  let molecular_decimal = 6;
  let denominator_decimal = 6;

  let user = provider.wallet.publicKey;
  console.log("user:", user.toBase58());

  it("account setup", async () => {
    console.log("feeCollector: ", feeCollectorKeyPair.publicKey.toBase58());

    // get airdrop
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        trustedRelayerKeyPairs[0].publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      )
    );

    // get airdrop
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        feePayerKeyPair.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      )
    );
  });

  it("Initializes Vizing Pad", async () => {
    console.log("start initialize");
    let vizingPadBump: number;
    let relayerBump: number;
    {
      const seed = [VizingPadConfigsSeed];
      const [vizingPad, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      vizingPadConfigs = vizingPad;
      vizingPadBump = bump;

      console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);
    }

    const initParams = {
      owner: provider.wallet.publicKey,
      feeCollector: feeCollectorKeyPair.publicKey,
      engineAdmin: engineAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
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
        vizingProgram.programId
      );

      const authorityU8Array = new Uint8Array(
        authority.toBuffer().slice(0, 32)
      );

      vizingAuthority = authority;
      console.log(`authority: ${authority.toBase58()}, bump: ${bump}`);
    }

    {
      const tx = await vizingProgram.methods
        .initializeVizingPad(initParams)
        .accounts({
          vizingPadConfig: vizingPadConfigs,
          vizingPadAuthority: vizingAuthority,
          payer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log(`initialize: ${tx}`);

      const vizingPadAccount =
        await vizingProgram.account.vizingPadConfigs.fetch(vizingPadConfigs);

      expect(vizingPadAccount.owner.toBase58()).to.equal(
        provider.wallet.publicKey.toBase58()
      );
      expect(vizingPadAccount.engineAdmin.toBase58()).to.equal(
        initParams.engineAdmin.toBase58()
      );
      expect(vizingPadAccount.feeCollector.toBase58()).to.equal(
        initParams.feeCollector.toBase58()
      );
      expect(vizingPadAccount.stationAdmin.toBase58()).to.equal(
        initParams.stationAdmin.toBase58()
      );
      expect(vizingPadAccount.gasPoolAdmin.toBase58()).to.equal(
        initParams.gasPoolAdmin.toBase58()
      );
      expect(vizingPadAccount.registeredValidator.toBase58()).to.equal(
        initParams.registeredValidator.toBase58()
      );
      expect(vizingPadAccount.bump).to.equal(vizingPadBump);

      expect(vizingPadAccount.isPaused).to.equal(false);
    }

    {
      try {
        await vizingProgram.methods
          .initializeVizingPad(initParams)
          .accounts({
            vizingPadConfig: vizingPadConfigs,
            vizingPadAuthority: vizingAuthority,
            payer: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.transactionMessage).to.be.eql(
          "Transaction simulation failed: Error processing Instruction 0: custom program error: 0x0"
        );
        console.log("could not initialize vizing pad twice");
      }
    }
  });

  const initGasSystemParams = {
    chainId: id,
    basePrice: global_base_price,
    molecular: molecular,
    denominator: denominator,
    molecularDecimal: molecular_decimal,
    denominatorDecimal: denominator_decimal,
    globalBasePrice: global_base_price,
    defaultGasLimit: default_gas_limit,
    amountInThreshold: amount_in_threshold,
    globalMolecular: molecular,
    globalDenominator: denominator,
  };

  it("Initializes gas global", async () => {
    //init_mapping_fee_config
    [mappingFeeConfigAuthority, mappingFeeConfigBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_mapping_fee_config")],
        PROGRAM_ID
      );
    console.log(
      "mappingFeeConfigAuthority:",
      mappingFeeConfigAuthority.toString()
    );
    console.log("mappingFeeConfigBump:", mappingFeeConfigBump);
    try {
      const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
        mappingFeeConfigAuthority
      );
    } catch (e) {
      const initializeGasSystem = await vizingProgram.methods
        .initializeGasSystem(initGasSystemParams)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          vizingPadConfig: vizingPadConfigs,
          user: user,
          systemProgram: PROGRAM_ID,
        })
        .signers([])
        .rpc();
      console.log(`initializeGasSystem:${initializeGasSystem}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(initializeGasSystem);
    }
  });

  it("Initializes record message", async () => {
    //init_current_record_message
    [recordMessageAuthority, recordMessageBump] =
      await PublicKey.findProgramAddress(
        [Buffer.from("init_current_record_message")],
        PROGRAM_ID
      );
    console.log("recordMessageAuthority:", recordMessageAuthority.toString());
    console.log("recordMessageBump:", recordMessageBump);
    try {
      const recordValid =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const valid = await recordValid.initState;
      console.log("valid:", valid);
    } catch (e) {
      const initRecordMessage = await vizingProgram.methods
        .initRecordMessage()
        .accounts({
          currentRecordMessage: recordMessageAuthority,
          user: user,
          systemProgram: PROGRAM_ID,
        })
        .signers([])
        .rpc();
      console.log(`initRecordMessage:${initRecordMessage}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(initRecordMessage);
    }
  });

  it("modify Vizing Pad", async () => {
    const modifyParams = {
      owner: provider.wallet.publicKey,
      feeCollector: feeCollectorKeyPair.publicKey,
      engineAdmin: engineAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
      stationAdmin: stationAdminKeyPair.publicKey,
      trustedRelayers: trustedRelayerKeyPairs.map(
        (keypair) => keypair.publicKey
      ),
      registeredValidator: anchor.web3.Keypair.generate().publicKey,
      isPaused: false,
    };

    {
      try {
        const fakeOwner = anchor.web3.Keypair.generate();
        await vizingProgram.methods
          .modifySettings(modifyParams)
          .accounts({
            owner: fakeOwner.publicKey,
            vizing: vizingPadConfigs,
          })
          .signers([fakeOwner])
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal("Unauthorized: Not Owner");
      }
    }

    {
      const tx = await vizingProgram.methods
        .modifySettings(modifyParams)
        .accounts({
          owner: provider.wallet.publicKey,
          vizing: vizingPadConfigs,
        })
        .rpc();
      console.log(`modify: ${tx}`);

      const vizingPadAccount =
        await vizingProgram.account.vizingPadConfigs.fetch(vizingPadConfigs);

      expect(vizingPadAccount.owner.toBase58()).to.equal(
        provider.wallet.publicKey.toBase58()
      );
      expect(vizingPadAccount.feeCollector.toBase58()).to.equal(
        modifyParams.feeCollector.toBase58()
      );
      expect(vizingPadAccount.engineAdmin.toBase58()).to.equal(
        modifyParams.engineAdmin.toBase58()
      );
      expect(vizingPadAccount.stationAdmin.toBase58()).to.equal(
        modifyParams.stationAdmin.toBase58()
      );
      expect(vizingPadAccount.registeredValidator.toBase58()).to.equal(
        modifyParams.registeredValidator.toBase58()
      );
    }
  });

  it("grant fee collector", async () => {
    const newFeeCollector = anchor.web3.Keypair.generate();

    {
      try {
        const fakeGasPoolAdmin = anchor.web3.Keypair.generate();
        await vizingProgram.methods
          .grantFeeCollector(newFeeCollector.publicKey)
          .accounts({
            gasPoolAdmin: fakeGasPoolAdmin.publicKey,
            vizing: vizingPadConfigs,
          })
          .signers([fakeGasPoolAdmin])
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal(
          "Unauthorized: Not Gas Pool Admin"
        );
      }
    }

    {
      const tx = await vizingProgram.methods
        .grantFeeCollector(newFeeCollector.publicKey)
        .accounts({
          gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
          vizing: vizingPadConfigs,
        })
        .signers([gasPoolAdminKeyPair])
        .rpc();
      console.log(`grant fee collector: ${tx}`);

      let vizingPadAccount = await vizingProgram.account.vizingPadConfigs.fetch(
        vizingPadConfigs
      );

      expect(vizingPadAccount.feeCollector.toBase58()).to.equal(
        newFeeCollector.publicKey.toBase58()
      );

      await vizingProgram.methods
        .grantFeeCollector(feeCollectorKeyPair.publicKey)
        .accounts({
          gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
          vizing: vizingPadConfigs,
        })
        .signers([gasPoolAdminKeyPair])
        .rpc();

      vizingPadAccount = await vizingProgram.account.vizingPadConfigs.fetch(
        vizingPadConfigs
      );

      expect(vizingPadAccount.feeCollector.toBase58()).to.equal(
        feeCollectorKeyPair.publicKey.toBase58()
      );
    }
  });

  async function SetThisGasGlobal(
    thisChainId,
    thisGlobalBasePrice,
    thisDefaultGasLimit,
    thisAmountInThreshold,
    thisMolecular,
    thisDenominator
  ) {
    try {
      const setThisGasGlobal = await vizingProgram.methods
        .setThisGasGlobal(
          thisChainId,
          thisGlobalBasePrice,
          thisDefaultGasLimit,
          thisAmountInThreshold,
          thisMolecular,
          thisDenominator
        )
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          vizing: vizingPadConfigs,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(`setThisGasGlobal:${setThisGasGlobal}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(setThisGasGlobal);
    } catch (e) {
      console.log("SetThisGasGlobal error:", e);
    }
  }
  let new_global_base_price = new anchor.BN(500);
  let new_default_gas_limit = new anchor.BN(1000);
  let new_amount_in_threshold = new anchor.BN(100000000);
  it("Set gas global", async () => {
    await SetThisGasGlobal(
      id,
      new_global_base_price,
      new_default_gas_limit,
      new_amount_in_threshold,
      molecular,
      denominator
    );
  });

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
      const setThisFeeConfig = await vizingProgram.methods
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
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(`setThisFeeConfig tx:${setThisFeeConfig}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(setThisFeeConfig);
    } catch (e) {
      console.log("SetThisFeeConfig error:", e);
    }
  }
  let base_price = new anchor.BN(500);
  it("Set fee config", async () => {
    //chainId1
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      molecular_decimal,
      denominator_decimal
    );
    //chainId2
    await SetThisFeeConfig(
      id2,
      base_price,
      reserve,
      molecular,
      denominator,
      molecular_decimal,
      denominator_decimal
    );
  });

  async function SetThisTokenFeeConfig(
    thisChainId,
    thisMolecular,
    thisDenominator
  ) {
    try {
      const setThisTokenFeeConfig = await vizingProgram.methods
        .setThisTokenFeeConfig(thisChainId, thisMolecular, thisDenominator)
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(`setThisTokenFeeConfig:${setThisTokenFeeConfig}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(setThisTokenFeeConfig);
    } catch (e) {
      console.log("SetThisTokenFeeConfig error:", e);
    }
  }
  it("Set token fee config", async () => {
    await SetThisTokenFeeConfig(id, molecular, denominator);
  });

  function ethereumAddressToU8Array(address: string): number[] {
    //remove 0x
    const cleanAddress = address.startsWith("0x") ? address.slice(2) : address;
    const bytes = new Uint8Array(32);
    for (let i = 0; i < 32; i++) {
      const byte = parseInt(cleanAddress.substr(i * 2, 2), 16);
      bytes[i] = byte;
    }
    const addressArray: number[] = Array.from(bytes);
    return addressArray;
  }

  let dapp = ethereumAddressToU8Array(
    "0xaE67336f06B10fbbb26F31d31AbEA897290109B9"
  );
  let dapp2 = ethereumAddressToU8Array(
    "0xE3020Ac60f45842A747F6008390d0D28dDbBD98D"
  );
  let dapp3 = ethereumAddressToU8Array(
    "0xd1A48613D41E7BB2C68aD90D5fE5e7041ebA5111"
  );

  async function SetThisDappPriceConfig(
    thisChainid,
    thisDapp,
    thisMolecular,
    thisDenominator,
    thisBasePrice
  ) {
    try {
      const setThisDappPriceConfig = await vizingProgram.methods
        .setThisDappPriceConfig(
          thisChainid,
          thisDapp,
          thisMolecular,
          thisDenominator,
          thisBasePrice
        )
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(`setThisDappPriceConfig tx:${setThisDappPriceConfig}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(setThisDappPriceConfig);
    } catch (e) {
      console.log("SetThisDappPriceConfig error:", e);
    }
  }
  it("Set dapp price config", async () => {
    await SetThisDappPriceConfig(id, dapp, molecular, denominator, base_price);
  });

  async function SetThisExchangeRate(
    thisChainId,
    thisMolecular,
    thisDenominator,
    thisMolecularDecimal,
    thisDenominatorDecimal
  ) {
    try {
      const setThisExchangeRate = await vizingProgram.methods
        .setThisExchangeRate(
          thisChainId,
          thisMolecular,
          thisDenominator,
          thisMolecularDecimal,
          thisDenominatorDecimal
        )
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(`setThisExchangeRate tx:${setThisExchangeRate}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(setThisExchangeRate);
    } catch (e) {
      console.log("SetThisExchangeRate error:", e);
    }
  }
  it("Set exchange rate", async () => {
    await SetThisExchangeRate(
      id,
      molecular,
      denominator,
      molecular_decimal,
      denominator_decimal
    );
  });

  async function BatchSetThisTokenFeeConfig(
    thisChainIds,
    thisMoleculars,
    thisDenominators
  ) {
    try {
      const batchSetThisTokenFeeConfig = await vizingProgram.methods
        .batchSetThisTokenFeeConfig(
          thisChainIds,
          thisMoleculars,
          thisDenominators
        )
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(`batchSetThisTokenFeeConfig:${batchSetThisTokenFeeConfig}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(batchSetThisTokenFeeConfig);
    } catch (e) {
      console.log("BatchSetThisTokenFeeConfig error:", e);
    }
  }
  let destChainIds = [id];
  let moleculars = [new anchor.BN(5)];
  let denominators = [new anchor.BN(10)];
  it("Batch set token fee config", async () => {
    await BatchSetThisTokenFeeConfig(destChainIds, moleculars, denominators);
  });

  async function BatchSetThisTradeFeeConfigMap(
    thisDapps,
    thisChainIds,
    thisMoleculars,
    thisDenominators,
    thisBasePriceGroup
  ) {
    try {
      const batchSetThisTradeFeeConfigMap = await vizingProgram.methods
        .batchSetThisTradeFeeConfigMap(
          thisDapps,
          thisChainIds,
          thisMoleculars,
          thisDenominators,
          thisBasePriceGroup
        )
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(
        `batchSetThisTradeFeeConfigMap tx:${batchSetThisTradeFeeConfigMap}'`
      );
      // Confirm transaction
      await provider.connection.confirmTransaction(
        batchSetThisTradeFeeConfigMap
      );
    } catch (e) {
      console.log("BatchSetThisTradeFeeConfigMap error:", e);
    }
  }
  let tradeFeeConfig_dapps = [dapp, dapp2];
  let tradeFeeConfig_destChainIds = [new anchor.BN(4), new anchor.BN(5)];
  let tradeFeeConfig_moleculars = [new anchor.BN(5), new anchor.BN(5)];
  let tradeFeeConfig_denominators = [new anchor.BN(10), new anchor.BN(10)];
  let base_price_group = [new anchor.BN(100), new anchor.BN(200)];
  it("Batch set trade fee config mapping", async () => {
    await BatchSetThisTradeFeeConfigMap(
      tradeFeeConfig_dapps,
      tradeFeeConfig_destChainIds,
      tradeFeeConfig_moleculars,
      tradeFeeConfig_denominators,
      base_price_group
    );
  });

  async function BatchSetThisDappPriceConfigInDiffChain(
    thisChainIds,
    thisDapps,
    thisBasePriceGroup
  ) {
    try {
      const batchSetThisDappPriceConfigInDiffChain = await vizingProgram.methods
        .batchSetThisDappPriceConfigInDiffChain(
          thisChainIds,
          thisDapps,
          thisDapps
        )
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(
        `batchSetThisDappPriceConfigInDiffChain tx:${batchSetThisDappPriceConfigInDiffChain}'`
      );
      // Confirm transaction
      await provider.connection.confirmTransaction(
        batchSetThisDappPriceConfigInDiffChain
      );
    } catch (e) {
      console.log("BatchSetThisDappPriceConfigInDiffChain error:", e);
    }
  }
  let diff_destChainIds = [new anchor.BN(4), new anchor.BN(5)];
  let diff_dapps = [dapp, dapp2];
  let diff_base_prices = [new anchor.BN(1000), new anchor.BN(2000)];
  it("Batch set dapp price config in different chain", async () => {
    await BatchSetThisDappPriceConfigInDiffChain(
      diff_destChainIds,
      diff_dapps,
      diff_base_prices
    );
  });

  async function BatchSetThisDappPriceConfigInSameChain(
    thisChainId,
    thisDapps,
    thisBasePriceGroup
  ) {
    try {
      const batchSetThisDappPriceConfigInSameChain = await vizingProgram.methods
        .batchSetThisDappPriceConfigInSameChain(
          thisChainId,
          thisDapps,
          thisBasePriceGroup
        )
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(
        `batchSetThisDAppPriceConfigInSameChain tx:${batchSetThisDappPriceConfigInSameChain}'`
      );
      // Confirm transaction
      await provider.connection.confirmTransaction(
        batchSetThisDappPriceConfigInSameChain
      );
    } catch (e) {
      console.log("BatchSetThisDappPriceConfigInSameChain error:", e);
    }
  }
  let DappPriceConfig_dapps = [dapp];
  let DappPriceConfig_base_prices = [new anchor.BN(1000)];
  it("Batch set dapp price config in same chain", async () => {
    await BatchSetThisDappPriceConfigInSameChain(
      id,
      DappPriceConfig_dapps,
      DappPriceConfig_base_prices
    );
  });

  async function GetDappBasePrice(dest_chain_id, chain_base_price, dapp) {
    let dapp_base_price;
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const tradeFeeConfigMappings = vizingGasSystem.tradeFeeConfigMappings;
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
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const feeConfigMappings = vizingGasSystem.feeConfigMappings;
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
          amount_out /
          (10 ^
            (fee_config_molecular_decimal - fee_config_denominator_decimal));
      } else {
        this_amount_out =
          amount_out /
          (10 ^
            (fee_config_denominator_decimal - fee_config_molecular_decimal));
      }
    } else {
      this_amount_out = amount_out;
    }

    let amount_in =
      (this_amount_out * fee_config_denominator) / fee_config_denominator;
    console.log("ExactOutput:", amount_in);
    return amount_in;
  }

  async function ComputeTradeFee1(dest_chain_id, amount_out) {
    let computeTradeFee1;
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const tradeFeeMappings = vizingGasSystem.tradeFeeMappings;
    const gasSystemGlobalMappings = vizingGasSystem.gasSystemGlobalMappings;
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
        (amount_out * gasSystemGlobal_molecular) / gasSystemGlobal_denominator;
    } else {
      if (tradeFee_molecular != 0 && tradeFee_denominator != 0) {
        return 0;
      } else {
        computeTradeFee1 =
          (amount_out * tradeFee_molecular) / tradeFee_denominator;
      }
    }
    return computeTradeFee1;
  }

  async function ComputeTradeFee2(target_contract, dest_chain_id, amount_out) {
    const isNonZero = target_contract.some((byte) => byte !== 0);
    let computeTradeFee2;
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const tradeFeeConfigMappings = vizingGasSystem.tradeFeeConfigMappings;
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
          (amount_out * trade_fee_config_molecular) /
          trade_fee_config_denominator;
      } else {
        return 0;
      }
    } else {
      computeTradeFee2 = await ComputeTradeFee1(dest_chain_id, amount_out);
    }
    console.log("ComputeTradeFee2:", computeTradeFee2);
    return computeTradeFee2;
  }

  async function EstimatePrice1(target_contract, dest_chain_id) {
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const gasSystemGlobalMappings = vizingGasSystem.gasSystemGlobalMappings;
    const tradeFeeConfigMappings = vizingGasSystem.tradeFeeConfigMappings;
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
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const feeConfigMappings = await vizingGasSystem.feeConfigMappings;
    const gasSystemGlobalMappings = vizingGasSystem.gasSystemGlobalMappings;

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
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const feeConfigMappings = vizingGasSystem.feeConfigMappings;
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
    let this_amount_in;
    if (fee_config_molecular_decimal != fee_config_denominator_decimal) {
      if (fee_config_molecular_decimal > fee_config_denominator_decimal) {
        this_amount_in =
          amount_in *
          (10 ^
            (fee_config_molecular_decimal - fee_config_denominator_decimal));
      } else {
        this_amount_in =
          amount_in /
          (10 ^
            (fee_config_denominator_decimal - fee_config_molecular_decimal));
      }
    } else {
      this_amount_in = amount_in;
    }
    let amount_out =
      (this_amount_in * fee_config_molecular) / fee_config_denominator;
    console.log("ExactInput:", amount_out);
    return amount_out;
  }

  async function EstimateGas(amount_out, dest_chain_id, this_message) {
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const feeConfigMappings = vizingGasSystem.feeConfigMappings;
    const gasSystemGlobalMappings = vizingGasSystem.gasSystemGlobalMappings;
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

    if (amount_out > 0) {
      let output_amount_in;
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
    console.log("EstimateGas fee:", fee);
    return fee;
  }
  // await EstimateGas(testAmountOut, id, newMessage);

  async function EstimateTotalFee(dest_chain_id, amount_out, this_message) {
    const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
      mappingFeeConfigAuthority
    );
    const feeConfigMappings = vizingGasSystem.feeConfigMappings;
    const gasSystemGlobalMappings = vizingGasSystem.gasSystemGlobalMappings;

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
      token_amount_limit = gasSystemGlobalMapping.amountInThreshold.toNumber();
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

    let output_amount_in = amount_out;
    let finalFee;
    if (amount_out.toNumber() > 0) {
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

  async function EstimateVizingGasFee(
    value,
    dest_chain_id,
    _addition_params,
    thisMessage
  ) {
    await EstimateGas(value, dest_chain_id, thisMessage);
  }

  async function BatchSetThisExchangeRate(
    thisChainIds,
    thisMoleculars,
    thisDenominators,
    thisMolecularDecimals,
    thisDenominatorDecimals
  ) {
    try {
      const batchSetThisExchangeRate = await vizingProgram.methods
        .batchSetThisExchangeRate(
          thisChainIds,
          thisMoleculars,
          thisDenominators,
          thisMolecularDecimals,
          thisMolecularDecimals
        )
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();
      console.log(`batchSetThisExchangeRate:${batchSetThisExchangeRate}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(batchSetThisExchangeRate);
    } catch (e) {
      console.log("BatchSetThisExchangeRate error:", e);
    }
  }
  let batchExchangeRate_destChainIds = [new anchor.BN(4), new anchor.BN(5)];
  let batchExchangeRate_moleculars = [new anchor.BN(10), new anchor.BN(20)];
  let batchExchangeRate_denominators = [new anchor.BN(50), new anchor.BN(100)];
  let molecular_decimals = Buffer.from([6, 6]);
  let denominator_decimals = Buffer.from([6, 6]);
  it("Batch set exchange rate", async () => {
    await BatchSetThisExchangeRate(
      batchExchangeRate_destChainIds,
      batchExchangeRate_moleculars,
      batchExchangeRate_denominators,
      molecular_decimals,
      denominator_decimals
    );
  });

  //GetEstimateVizingGasFee
  let testAmountOut1 = new anchor.BN(1000);
  it("Get estimate vizing gas1 fee for dev", async () => {
    const executeGasLimit = new anchor.BN(6);
    const maxFeePerGas = new anchor.BN(2000);
    const estimateTotalFeeMessage = {
      mode: Buffer.from([1]), // u8
      targetContract: Buffer.from(dapp), // [u8; 32]
      executeGasLimit: executeGasLimit, // u32
      maxFeePerGas: maxFeePerGas, // u64
      signature: Buffer.from("transfer from alice to bob"), // Buffer
    };

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
    let newAdditionParams = Buffer.from("abc");
    console.log("bufferMessage:", bufferMessage);
    try {
      const estimateVizingGasFee1 = await vizingProgram.methods
        .estimateVizingGasFee1(
          testAmountOut1,
          id,
          newAdditionParams,
          bufferMessage
        )
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`estimateVizingGasFee1 tx:${estimateVizingGasFee1}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(estimateVizingGasFee1);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const estimateVizingGasFeeNumber =
        await currentRecordMessage.estimateVizingGasFee.toNumber();
      console.log("estimateVizingGasFeeNumber1:", estimateVizingGasFeeNumber);
    } catch (e) {
      console.log("GetEstimateTotalFee error:", e);
    }
  });

  it("Get estimate vizing gas2 fee for dev", async () => {
    const executeGasLimit = new anchor.BN(6);
    const maxFeePerGas = new anchor.BN(2000);
    const estimateTotalFeeMessage = {
      mode: 1, // u8
      targetContract: dapp, // [u8; 32]
      executeGasLimit: executeGasLimit, // u32
      maxFeePerGas: maxFeePerGas, // u64
      signature: Buffer.from("transfer from alice to bob"), // Buffer
    };
    let newAdditionParams = Buffer.from("abc");
    try {
      const estimateVizingGasFee2 = await vizingProgram.methods
        .estimateVizingGasFee2(
          testAmountOut1,
          id,
          newAdditionParams,
          estimateTotalFeeMessage
        )
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`estimateVizingGasFee2 tx:${estimateVizingGasFee2}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(estimateVizingGasFee2);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const estimateVizingGasFeeNumber =
        await currentRecordMessage.estimateVizingGasFee.toNumber();
      console.log("estimateVizingGasFeeNumber2:", estimateVizingGasFeeNumber);
    } catch (e) {
      console.log("GetEstimateTotalFee2 error:", e);
    }
  });

  it("Launch", async () => {
    const message = {
      mode: 5,
      targetProgram: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      executeGasLimit: new anchor.BN(6),
      maxFeePerGas: new anchor.BN(7),
      signature: Buffer.from("1234"),
    };

    const additionParams = {
      mode: 0,
      signature: Buffer.alloc(0),
    };

    const launchParams = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      sender: provider.wallet.publicKey,
      value: new anchor.BN(3),
      destChainid: new anchor.BN(4),
      additionParams: additionParams,
      message: message,
    };

    {
      // vizing app settings
      const [feeRouter, bump2] = anchor.web3.PublicKey.findProgramAddressSync(
        [vizingFeeRouterSeed],
        vizingAppMockProgram.programId
      );

      vizingFeeRouter = feeRouter;

      console.log(`feeRouter: ${feeRouter.toBase58()}, bump: ${bump2}`);

      const [messageAuthority, bump3] =
        anchor.web3.PublicKey.findProgramAddressSync(
          [vizingMessageAuthoritySeed],
          vizingAppMockProgram.programId
        );

      vizingMessageAuthority = messageAuthority;

      console.log(
        `messageAuthority: ${messageAuthority.toBase58()}, bump: ${bump3}`
      );

      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
          vizingFeeRouter,
          1_000_000_000
        ),
        "confirmed"
      );

      await vizingAppMockProgram.methods
        .initializeVizingEmitter()
        .accounts({
          messagePdaAuthority: vizingMessageAuthority,
          payer: provider.wallet.publicKey,
        })
        .rpc();
    }

    {
      try {
        await vizingProgram.methods
          .launch(launchParams)
          .accounts({
            vizingAppFeePayer: provider.wallet.publicKey,
            vizingAppMessageAuthority: provider.wallet.publicKey,
            vizingPadConfig: vizingPadConfigs,
            vizingPadFeeCollector: anchor.web3.Keypair.generate().publicKey,
          })
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal(
          "Unauthorized: Fee Collector Invalid"
        );
      }
    }

    {
      const feeReceiverBalanceBefore = await provider.connection.getBalance(
        feeCollectorKeyPair.publicKey
      );

      const tx = await vizingProgram.methods
        .launch(launchParams)
        .accounts({
          vizingAppFeePayer: provider.wallet.publicKey,
          vizingAppMessageAuthority: provider.wallet.publicKey,
          vizingPadConfig: vizingPadConfigs,
          vizingPadFeeCollector: feeCollectorKeyPair.publicKey,
        })
        .rpc();
      console.log(`launch: ${tx}`);

      const feeReceiverBalanceAfter = await provider.connection.getBalance(
        feeCollectorKeyPair.publicKey
      );

      console.log(
        `feeReceiverBalanceBefore: ${
          feeReceiverBalanceBefore / anchor.web3.LAMPORTS_PER_SOL
        }, feeReceiverBalanceAfter: ${
          feeReceiverBalanceAfter / anchor.web3.LAMPORTS_PER_SOL
        }`
      );

      expect(feeReceiverBalanceAfter).to.greaterThan(feeReceiverBalanceBefore);
    }

    {
      console.log("launchVizing");
      // vizing app launch
      const feeReceiverBalanceBefore = await provider.connection.getBalance(
        feeCollectorKeyPair.publicKey
      );
      const tx = await vizingAppMockProgram.methods
        .launchVizing()
        .accounts({
          user: provider.wallet.publicKey,
          vizingAppMessageAuthority: vizingMessageAuthority,
          vizingPadConfig: vizingPadConfigs,
          vizingPadFeeCollector: feeCollectorKeyPair.publicKey,
          vizingPadProgram: vizingProgram.programId,
        })
        .rpc();
      console.log(`launchVizing: ${tx}`);

      const feeReceiverBalanceAfter = await provider.connection.getBalance(
        feeCollectorKeyPair.publicKey
      );

      console.log(
        `feeReceiverBalanceBefore: ${
          feeReceiverBalanceBefore / anchor.web3.LAMPORTS_PER_SOL
        }, feeReceiverBalanceAfter: ${
          feeReceiverBalanceAfter / anchor.web3.LAMPORTS_PER_SOL
        }`
      );

      expect(feeReceiverBalanceAfter).to.greaterThan(feeReceiverBalanceBefore);
    }
  });

  it("Landing", async () => {
    let solPdaReceiver: anchor.web3.PublicKey;
    const resultDataSeed = "result_data_seed";

    const [resultDataAccount, resultDataBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(resultDataSeed)],
        vizingAppMockProgram.programId
      );

    const targetProgram = vizingAppMockProgram.programId;

    {
      // #### register vizing app start
      const [vizingAppContract, vizingAppBump] =
        anchor.web3.PublicKey.findProgramAddressSync(
          [vizingAppConfigSeed, targetProgram.toBuffer()],
          vizingProgram.programId
        );

      console.log(
        `vizingAppConfig: ${vizingAppContract.toBase58()}, bump: ${vizingAppBump}`
      );

      const [solReceiver, bump1] = anchor.web3.PublicKey.findProgramAddressSync(
        [vizingAppSolReceiverSeed],
        targetProgram
      );

      solPdaReceiver = solReceiver;

      console.log(`solPdaReceiver: ${solPdaReceiver.toBase58()}`);

      vizingAppConfig = vizingAppContract;

      const registerParams = {
        solPdaReciver: solPdaReceiver,
        vizingAppAccounts: [resultDataAccount],
        vizingAppProgramId: targetProgram,
      };

      const tx = await vizingProgram.methods
        .registerVizingApp(registerParams)
        .accounts({
          admin: provider.wallet.publicKey,
          vizingAppConfigs: vizingAppConfig,
        })
        .rpc();
      console.log(`register vizing app: ${tx}`);

      const fetchedVizingAppConfig =
        await vizingProgram.account.vizingAppConfig.fetch(vizingAppConfig);

      expect(fetchedVizingAppConfig.vizingAppAccounts[0].toBase58()).to.equal(
        resultDataAccount.toBase58()
      );

      expect(fetchedVizingAppConfig.vizingAppProgramId.toBase58()).to.equal(
        targetProgram.toBase58()
      );

      expect(fetchedVizingAppConfig.admin.toBase58()).to.equal(
        provider.wallet.publicKey.toBase58()
      );

      expect(fetchedVizingAppConfig.bump).to.equal(vizingAppBump);
    }

    // const [solReceiver2, bump2] = anchor.web3.PublicKey.findProgramAddressSync(
    //   [vizingAppSolReceiverSeed],
    //   vizingAppProgram.programId
    // );

    // await vizingAppProgram.methods
    //   .initialize()
    //   .accounts({
    //     solPdaReceiver: solReceiver2,
    //     payer: provider.wallet.publicKey,
    //   })
    //   .rpc();

    await vizingAppMockProgram.methods
      .initialize()
      .accounts({
        resultAccount: resultDataAccount,
        payer: provider.wallet.publicKey,
      })
      .rpc();

    await vizingAppMockProgram.methods
      .initializeVizingReceiver()
      .accounts({
        solPdaReceiver: solPdaReceiver,
        payer: provider.wallet.publicKey,
      })
      .rpc();

    const message = {
      mode: 5,
      targetProgram: targetProgram,
      executeGasLimit: new anchor.BN(6),
      maxFeePerGas: new anchor.BN(7),
      signature: Buffer.concat([
        Buffer.from([0, 0, 0, 0, 0, 0, 0, 2]),
        Buffer.from([0, 0, 0, 0, 0, 0, 0, 3]),
      ]),
    };
    console.log(`signature: ${message.signature.toString("hex")}`);

    const landingParams = {
      messageId: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      srcChainid: new anchor.BN(3),
      srcTxHash: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      srcContract: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      srcChainNonce: new anchor.BN(4),
      sender: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      value: new anchor.BN(5),
      additionParams: Buffer.alloc(0),
      message: message,
    };

    {
      try {
        const mockRelayer = anchor.web3.Keypair.generate();
        await vizingProgram.methods
          .landing(landingParams)
          .accounts({
            relayer: mockRelayer.publicKey,
            vizing: vizingPadConfigs,
            vizingAuthority: vizingAuthority,
            targetProgram: targetProgram,
            vizingAppConfigs: vizingAppConfig,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([mockRelayer])
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal("Unauthorized: Not Relayer");
      }
    }

    {
      try {
        await vizingProgram.methods
          .landing(landingParams)
          .accounts({
            relayer: trustedRelayerKeyPairs[0].publicKey,
            vizing: vizingPadConfigs,
            vizingAuthority: vizingAuthority,
            targetProgram: targetProgram,
            vizingAppConfigs: vizingAppConfig,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([trustedRelayerKeyPairs[0]])
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal(
          "Vizing App Not In Remaining Accounts"
        );
      }
    }

    {
      try {
        await vizingProgram.methods
          .landing(landingParams)
          .accounts({
            relayer: trustedRelayerKeyPairs[0].publicKey,
            vizing: vizingPadConfigs,
            vizingAuthority: vizingAuthority,
            targetProgram: targetProgram,
            vizingAppConfigs: null,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([trustedRelayerKeyPairs[0]])
          .rpc();
        throw new Error("should not come here");
      } catch (error: any) {
        expect(error.toString()).to.not.contain(
          "Vizing App Not In Remaining Accounts"
        );
      }
    }

    {
      const padReceiverBalanceBefore = new anchor.BN(
        await provider.connection.getBalance(solPdaReceiver)
      );

      const relayerBalanceBefore = new anchor.BN(
        await provider.connection.getBalance(feePayerKeyPair.publicKey)
      );

      const relayer = trustedRelayerKeyPairs[0];
      const feepayer = feePayerKeyPair;
      console.log(`relayer: ${relayer.publicKey.toBase58()}`);
      console.log(`feepayer: ${feepayer.publicKey.toBase58()}`);
      const tx = await vizingProgram.methods
        .landing(landingParams)
        .accounts({
          relayer: relayer.publicKey,
          vizing: vizingPadConfigs,
          vizingAuthority: vizingAuthority,
          targetProgram: targetProgram,
          vizingAppConfigs: vizingAppConfig,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .remainingAccounts([
          {
            pubkey: vizingAuthority,
            isSigner: false,
            isWritable: false,
          },
          {
            pubkey: solPdaReceiver,
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: vizingPadConfigs,
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: resultDataAccount,
            isSigner: false,
            isWritable: false,
          },
        ])
        .signers([relayer])
        .rpc();
      console.log(`landing tx: ${tx}`);

      const padReceiverBalanceAfter = new anchor.BN(
        await provider.connection.getBalance(solPdaReceiver)
      );

      const relayerBalanceAfter = new anchor.BN(
        await provider.connection.getBalance(feePayerKeyPair.publicKey)
      );

      console.log(
        `relayer b:${relayerBalanceBefore},relayer a:${relayerBalanceAfter}`
      );

      console.log(
        `feepayer b:${padReceiverBalanceBefore},feepayer a:${padReceiverBalanceAfter}`
      );

      expect(Number(padReceiverBalanceAfter)).equal(
        Number(padReceiverBalanceBefore.add(landingParams.value))
      );
    }
  });

  //test dev get interface
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

  it("Compute trade fee1", async () => {
    try {
      const computeTradeFee1 = await vizingProgram.methods
        .computeTradeFee1(id, testAmountOut)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`computeTradeFee1 tx:${computeTradeFee1}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(computeTradeFee1);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const computeTradeFee1Number =
        await currentRecordMessage.computeTradeFee1.toNumber();
      console.log("computeTradeFee1Number:", computeTradeFee1Number);
      let check_ComputeTradeFee1 = await ComputeTradeFee1(id, testAmountOut);
      expect(computeTradeFee1Number).to.equal(check_ComputeTradeFee1);
    } catch (e) {
      console.log("computeTradeFee1 error:", e);
    }
  });

  it("Compute trade fee2", async () => {
    try {
      const computeTradeFee2 = await vizingProgram.methods
        .computeTradeFee2(id, testAmountOut, dapp)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`computeTradeFee2 tx:${computeTradeFee2}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(computeTradeFee2);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const computeTradeFee2Number =
        await currentRecordMessage.computeTradeFee2.toNumber();
      console.log("computeTradeFee2Number:", computeTradeFee2Number);
      let check_ComputeTradeFee2 = await ComputeTradeFee2(
        id,
        testAmountOut,
        dapp
      );
      expect(computeTradeFee2Number).to.equal(check_ComputeTradeFee2);
    } catch (e) {
      console.log("computeTradeFee2 error:", e);
    }
  });

  it("Estimate price1", async () => {
    try {
      const estimatePrice1 = await vizingProgram.methods
        .estimatePrice1(dapp, id)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`estimatePrice1 tx:${estimatePrice1}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(estimatePrice1);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const estimatePrice1Number =
        await currentRecordMessage.estimatePrice1.toNumber();
      console.log("estimatePrice1Number:", estimatePrice1Number);
      let check_EstimatePrice1 = await EstimatePrice1(dapp, id);
      expect(estimatePrice1Number).to.equal(check_EstimatePrice1);
    } catch (e) {
      console.log("estimatePrice1 error:", e);
    }
  });

  it("Estimate price2", async () => {
    try {
      const estimatePrice2 = await vizingProgram.methods
        .estimatePrice2(id)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`estimatePrice2 tx:${estimatePrice2}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(estimatePrice2);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const estimatePrice2Number =
        await currentRecordMessage.estimatePrice2.toNumber();
      console.log("estimatePrice2Number:", estimatePrice2Number);
      let check_EstimatePrice2 = await EstimatePrice2(id);
      expect(estimatePrice2Number).to.equal(check_EstimatePrice2);
    } catch (e) {
      console.log("estimatePrice2 error:", e);
    }
  });

  it("Exact output", async () => {
    try {
      const exactOutput = await vizingProgram.methods
        .exactOutput(id, testAmountOut)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`exactOutput tx:${exactOutput}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(exactOutput);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const exactOutputNumber =
        await currentRecordMessage.exactOutput.toNumber();
      console.log("exactOutputNumber:", exactOutputNumber);
      let check_ExactOutput = await ExactOutput(id, testAmountOut);
      expect(exactOutputNumber).to.equal(check_ExactOutput);
    } catch (e) {
      console.log("exactOutput error:", e);
    }
  });

  it("Exact input", async () => {
    let testInAmount = new anchor.BN(1000);
    try {
      const exactInput = await vizingProgram.methods
        .exactOutput(id, testInAmount)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`exactInput tx:${exactInput}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(exactInput);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const exactInputNumber = await currentRecordMessage.exactInput.toNumber();
      console.log("exactInputNumber:", exactInputNumber);
      let check_ExactInput = await ExactInput(id, testInAmount);
      expect(exactInputNumber).to.equal(check_ExactInput);
    } catch (e) {
      console.log("exactInput error:", e);
    }
  });

  it("Get Estimate Gas", async () => {
    try {
      const estimateGas = await vizingProgram.methods
        .estimateGas(testAmountOut, id, newMessage)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`estimateGas tx:${estimateGas}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(estimateGas);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const estimateGasNumber =
        await currentRecordMessage.estimateGas.toNumber();
      console.log("estimateGasNumber:", estimateGasNumber);
      let check_EstimateGas = await EstimateGas(testAmountOut, id, newMessage);
      expect(estimateGasNumber).to.equal(check_EstimateGas);
    } catch (e) {
      console.log("estimateGas error:", e);
    }
  });

  it("Get Estimate Total Fee", async () => {
    try {
      const estimateTotalFee = await vizingProgram.methods
        .estimateTotalFee(id, testAmountOut, newMessage)
        .accounts({
          vizingGasSystem: mappingFeeConfigAuthority,
          currentRecordMessage: recordMessageAuthority,
        })
        .signers([])
        .rpc();
      console.log(`estimateTotalFee tx:${estimateTotalFee}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(estimateTotalFee);
      const currentRecordMessage =
        await vizingProgram.account.currentRecordMessage.fetch(
          recordMessageAuthority
        );
      const estimateTotalFeeNumber =
        await currentRecordMessage.estimateTotalFee.toNumber();
      console.log("estimateTotalFeeNumber:", estimateTotalFeeNumber);
      let check_EstimateTotalFee = await EstimateTotalFee(
        id,
        testAmountOut,
        newMessage
      );
      expect(estimateTotalFeeNumber).to.equal(check_EstimateTotalFee);
    } catch (e) {
      console.log("GetEstimateTotalFee error:", e);
    }
  });

  //remove dapp
  it("Remove dapp", async () => {
    try {
      const removeTradeFeeDapp = await vizingProgram.methods
        .removeTradeFeeDapp(id, dapp)
        .accounts({
          vizing: vizingPadConfigs,
          vizingGasSystem: mappingFeeConfigAuthority,
          user: user,
          systemProgram: PROGRAM_ID,
        })
        .signers([])
        .rpc();
      console.log(`removeTradeFeeDapp tx:${removeTradeFeeDapp}'`);
      // Confirm transaction
      await provider.connection.confirmTransaction(removeTradeFeeDapp);
      const vizingGasSystem = await vizingProgram.account.vizingGasSystem.fetch(
        mappingFeeConfigAuthority
      );
      const tradeFeeConfigDapps =
        vizingGasSystem.tradeFeeConfigMappings[0].dapps;
      console.log("tradeFeeConfigDapps:", tradeFeeConfigDapps);
    } catch (e) {
      console.log("RemoveTradeFeeDapp error:", e);
    }
  });

  async function Launch(
    thisLaunchParams,
    thisvizingPadConfigs,
    thisFeeCollector,
    thisMappingFeeConfig
  ) {
    try {
      let launch = await vizingProgram.methods
        .launch(thisLaunchParams)
        .accounts({
          vizingAppFeePayer: user,
          messageAuthority: user,
          vizing: thisvizingPadConfigs,
          feeCollector: thisFeeCollector,
          vizingGasSystem: thisMappingFeeConfig,
          systemProgram: vizingProgram.programId,
        })
        .signers([])
        .rpc();

      console.log(`Launch tx:${launch}'`);
      await provider.connection.confirmTransaction(launch);
    } catch (e) {
      console.log("launch error:", e);
    }
  }

  //launch success multi mode
  it("Launch test success multi mode", async () => {
    //big number value launch
    let nullChainId = new anchor.BN(6);
    const executeGasLimit = new anchor.BN(6);
    const maxFeePerGas = new anchor.BN(2000);
    const feeCollector = feeCollectorKeyPair.publicKey;
    //mode1
    let thisTestValue2 = {
      high: new anchor.BN(0),
      low: new anchor.BN(1000_000_000),
    };
    const testMessage1 = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice do mode 1"),
    };
    let this_fee_mode1 = await EstimateTotalFee(
      nullChainId,
      thisTestValue2,
      testMessage1
    );
    console.log("this_fee_mode1:", this_fee_mode1);
    const launchRelayer = new anchor.web3.Keypair();
    const newLaunchParams1 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer.publicKey,
      sender: user,
      value: thisTestValue2,
      destChainid: nullChainId,
      additionParams: Buffer.alloc(0),
      message: testMessage1,
    };
    await Launch(
      newLaunchParams1,
      vizingPadConfigs,
      feeCollector,
      mappingFeeConfigAuthority
    );

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
    await Launch(
      newLaunchParams2,
      vizingPadConfigs,
      feeCollector,
      mappingFeeConfigAuthority
    );
    let this_fee_mode2 = await EstimateTotalFee(
      nullChainId,
      thisTestValue2,
      testMessage2
    );
    console.log("this_fee_mode2:", this_fee_mode2);

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
    await Launch(
      newLaunchParams3,
      vizingPadConfigs,
      feeCollector,
      mappingFeeConfigAuthority
    );
    let this_fee_mode3 = await EstimateTotalFee(
      nullChainId,
      thisTestValue2,
      testMessage3
    );
    console.log("this_fee_mode3:", this_fee_mode3);

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
    await Launch(
      newLaunchParams4,
      vizingPadConfigs,
      feeCollector,
      mappingFeeConfigAuthority
    );
    let this_fee_mode4 = await EstimateTotalFee(
      nullChainId,
      thisTestValue2,
      testMessage4
    );
    console.log("this_fee_mode4:", this_fee_mode4);
  });

  //launch different molecular_decimal and denominator_decimal
  it("Launch test decimal", async () => {
    let launchRelayer = ethereumAddressToU8Array(
      "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    );

    let testCrossValue = {
      high: new anchor.BN(0),
      low: new anchor.BN(10_000_000),
    };
    //test molecular_decimal=8,denominator_decimal=125
    console.log("test molecular_decimal=6,denominator_decimal=103:");
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      8,
      125
    );
    const executeGasLimit = new anchor.BN(6);
    const maxFeePerGas = new anchor.BN(2000);
    const testMessage = {
      mode: 1,
      targetContract: dapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice do mode 1"),
    };
    const launchParams = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: testCrossValue,
      destChainid: id,
      additionParams: Buffer.alloc(0),
      message: testMessage,
    };
    await Launch(
      testMessage,
      vizingPadConfigs,
      feeCollectorKeyPair.publicKey,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=0,denominator_decimal=0
    console.log("test molecular_decimal=0,denominator_decimal=0:");
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      0,
      0
    );
    await Launch(
      launchParams,
      vizingPadConfigs,
      feeCollectorKeyPair.publicKey,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=0,denominator_decimal=9
    console.log("test molecular_decimal=0,denominator_decimal=9:");
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      0,
      9
    );
    await Launch(
      launchParams,
      vizingPadConfigs,
      feeCollectorKeyPair.publicKey,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=9,denominator_decimal=0
    console.log("test molecular_decimal=9,denominator_decimal=0:");
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      9,
      0
    );
    await Launch(
      launchParams,
      vizingPadConfigs,
      feeCollectorKeyPair.publicKey,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=9,denominator_decimal=18
    console.log("test molecular_decimal=9,denominator_decimal=18:");
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      9,
      18
    );
    await Launch(
      launchParams,
      vizingPadConfigs,
      feeCollectorKeyPair.publicKey,
      mappingFeeConfigAuthority
    );

    //test molecular_decimal=18,denominator_decimal=9
    console.log("test molecular_decimal=18,denominator_decimal=9:");
    await SetThisFeeConfig(
      id,
      base_price,
      reserve,
      molecular,
      denominator,
      18,
      9
    );
    await Launch(
      launchParams,
      vizingPadConfigs,
      feeCollectorKeyPair.publicKey,
      mappingFeeConfigAuthority
    );
  });

  //launch test error
  it("Launch test error", async () => {
    let invalidDapp = Buffer.from("0xdAC17F958D2ee523a2206206994597C13D831ec7");
    const executeGasLimit = new anchor.BN(1);
    const maxFeePerGas = new anchor.BN(10000);
    let launchRelayer = ethereumAddressToU8Array(
      "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    );
    let testCrossValue = {
      high: new anchor.BN(0),
      low: new anchor.BN(10_000_000),
    };

    let testChainId = new anchor.BN(88);
    let test_global_base_price = new anchor.BN(1000);
    let test_default_gas_limit = new anchor.BN(5);
    let test_amount_in_threshold = new anchor.BN(100000000000);
    let test_molecular = new anchor.BN(1000);
    let test_denominator = new anchor.BN(995);
    //set test chainId
    await SetThisGasGlobal(
      testChainId,
      test_global_base_price,
      test_default_gas_limit,
      test_amount_in_threshold,
      test_molecular,
      test_denominator
    );
    const message = {
      mode: 1,
      targetContract: invalidDapp,
      executeGasLimit: executeGasLimit,
      maxFeePerGas: maxFeePerGas,
      signature: Buffer.from("transfer from bob to alice 1000 usdt,wagmi"),
    };
    let newNonFeeCollector = new anchor.web3.Keypair();
    console.log("newNonFeeCollector:", newNonFeeCollector.publicKey.toBase58());
    const errLaunchParams1 = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: launchRelayer,
      sender: user,
      value: testCrossValue,
      destChainid: testChainId,
      additionParams: Buffer.alloc(0),
      message: message,
    };
    // error non fee_collector
    {
      await Launch(
        errLaunchParams1,
        vizingPadConfigs,
        newNonFeeCollector.publicKey,
        mappingFeeConfigAuthority
      );
      console.log("error non fee_collector");
    }

    //error non vizingPadConfigs
    {
      let newvizingPadConfigs = new anchor.web3.Keypair();
      console.log(
        "newvizingPadConfigs:",
        newvizingPadConfigs.publicKey.toBase58()
      );
      await Launch(
        errLaunchParams1,
        newvizingPadConfigs.publicKey,
        feeCollectorKeyPair.publicKey,
        mappingFeeConfigAuthority
      );
      console.log("error non vizingPadConfigs");
    }

    //error non mappingFeeConfigAuthority
    {
      let newMappingFeeConfigAuthority = new anchor.web3.Keypair();
      console.log(
        "newMappingFeeConfigAuthority:",
        newMappingFeeConfigAuthority.publicKey.toBase58()
      );
      await Launch(
        errLaunchParams1,
        vizingPadConfigs,
        feeCollectorKeyPair.publicKey,
        newMappingFeeConfigAuthority.publicKey
      );
      console.log("error non newMappingFeeConfigAuthority");
    }

    //error over amount_in_threshold
    {
      let errValue = new anchor.BN(100000000001); //
      const errLaunchParams2 = {
        erliestArrivalTimestamp: new anchor.BN(1),
        latestArrivalTimestamp: new anchor.BN(2),
        relayer: launchRelayer,
        sender: user,
        value: errValue,
        destChainid: testChainId,
        additionParams: Buffer.alloc(0),
        message: message,
      };
      await Launch(
        errLaunchParams2,
        vizingPadConfigs,
        feeCollectorKeyPair.publicKey,
        mappingFeeConfigAuthority
      );
      console.log("error over amount_in_threshold");
    }

    //error price < dapp_base_price
    {
      let invalidPrice = new anchor.BN(999); //dapp_base_price=1000
      const errorPriceMessage = {
        mode: 1,
        targetContract: dapp,
        executeGasLimit: executeGasLimit,
        maxFeePerGas: invalidPrice,
        signature: Buffer.from("transfer from alice to bob 100000$"),
      };
      const errLaunchParams4 = {
        erliestArrivalTimestamp: new anchor.BN(1),
        latestArrivalTimestamp: new anchor.BN(2),
        relayer: launchRelayer,
        sender: user,
        value: testCrossValue,
        destChainid: testChainId,
        additionParams: Buffer.alloc(0),
        message: errorPriceMessage,
      };
      await Launch(
        errLaunchParams4,
        vizingPadConfigs,
        feeCollectorKeyPair.publicKey,
        mappingFeeConfigAuthority
      );
      console.log("error price < dapp_base_price");
    }
  });
});
