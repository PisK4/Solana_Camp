import * as anchor from "@coral-xyz/anchor";
import { Program, Coder } from "@coral-xyz/anchor";
import { expect } from "chai";

import { VizingCore } from "../target/types/vizing_core";
import { VizingApp } from "../target/types/vizing_app";
import { VizingAppMock } from "../target/types/vizing_app_mock";

function padStringTo32Bytes(str: string): Buffer {
  const buffer = Buffer.alloc(32);
  buffer.write(str.replace("0x", ""), "hex");
  return buffer;
}

function padEthereumAddressToBuffer(address: string): Buffer {
  const cleanAddress = address.startsWith("0x") ? address.slice(2) : address;
  const buffer = Buffer.alloc(32);
  buffer.write(
    cleanAddress,
    32 - cleanAddress.length / 2,
    cleanAddress.length / 2,
    "hex"
  );
  return buffer;
}

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

describe("Vizing Test", () => {
  const provider = anchor.AnchorProvider.env();

  console.log("provider:", provider.publicKey.toBase58());

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const vizingProgram = anchor.workspace.VizingCore as Program<VizingCore>;
  const vizingAppProgram = anchor.workspace.VizingApp as Program<VizingApp>;
  const vizingAppMockProgram = anchor.workspace
    .VizingAppMock as Program<VizingAppMock>;
  const VizingPadConfigsSeed = Buffer.from("Vizing_Pad_Settings_Seed");
  const vizingAuthoritySeed = Buffer.from("Vizing_Authority_Seed");
  const vizingAppConfigSeed = Buffer.from("Vizing_App_Config_Seed");
  const vizingAppSolReceiverSeed = Buffer.from("Vizing_App_Sol_Receiver_Seed");
  const vizingFeeRouterSeed = Buffer.from("Vizing_Fee_Router_Seed");
  const vizingMessageAuthoritySeed = Buffer.from(
    "Vizing_Message_Authority_Seed"
  );
  const vizingGasSystemSeed = Buffer.from("init_mapping_fee_config");

  let vizingPadConfigs: anchor.web3.PublicKey;
  let vizingAuthority: anchor.web3.PublicKey;
  let vizingAppConfig: anchor.web3.PublicKey;
  let vizingFeeRouter: anchor.web3.PublicKey;
  let vizingMessageAuthority: anchor.web3.PublicKey;
  let vizingGasSystem: anchor.web3.PublicKey;

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

  const EVM_src_address = "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD";
  const EVM_address_buffer = padEthereumAddressToBuffer(EVM_src_address);
  console.log(EVM_address_buffer);

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

    {
      const seed = [vizingGasSystemSeed];
      const [gasSys, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      vizingGasSystem = gasSys;

      console.log(
        `vizingGasSystem: ${vizingGasSystem.toBase58()}, bump: ${bump}`
      );

      const initGasSystemParams = {
        chainId: new anchor.BN(28516),
        basePrice: new anchor.BN(anchor.web3.LAMPORTS_PER_SOL),
        molecular: new anchor.BN(0),
        denominator: new anchor.BN(10),
        molecularDecimal: 1,
        denominatorDecimal: 1,
        globalBasePrice: new anchor.BN(anchor.web3.LAMPORTS_PER_SOL),
        defaultGasLimit: new anchor.BN(1),
        amountInThreshold: new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 100),
        globalMolecular: new anchor.BN(0),
        globalDenominator: new anchor.BN(10),
      };

      await vizingProgram.methods
        .initializeGasSystem(initGasSystemParams)
        .accounts({
          mappingFeeConfig: vizingGasSystem,
          payer: provider.wallet.publicKey,
        })
        .rpc();

      {
        // ###test loop
        let id = new anchor.BN(100);
        for (let i = 0; i < 10; i++) {
          let new_global_base_price = new anchor.BN(
            anchor.web3.LAMPORTS_PER_SOL * (i + 1)
          );
          let new_default_gas_limit = new anchor.BN(1);
          let new_amount_in_threshold = new anchor.BN(
            anchor.web3.LAMPORTS_PER_SOL * 100
          );
          await vizingProgram.methods
            .setThisGasGlobal(
              id.add(new anchor.BN(i)),
              new_global_base_price,
              new_default_gas_limit,
              new_amount_in_threshold,
              initGasSystemParams.molecular,
              initGasSystemParams.denominator
            )
            .accounts({
              mappingFeeConfig: vizingGasSystem,
              vizing: vizingPadConfigs,
              user: gasPoolAdminKeyPair.publicKey,
            })
            .signers([gasPoolAdminKeyPair])
            .rpc();
        }
      }

      let dapp = padEthereumAddressToBuffer(
        "0xE3020Ac60f45842A747F6008390d0D28dDbBD981"
      );

      let dapp2 = padEthereumAddressToBuffer(
        "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD"
      );

      let tradeFeeConfig_dapps = [dapp, dapp2];
      let tradeFeeConfig_destChainIds = [
        new anchor.BN(28516),
        new anchor.BN(28516),
      ];
      let tradeFeeConfig_moleculars = [new anchor.BN(0), new anchor.BN(0)];
      let tradeFeeConfig_denominators = [new anchor.BN(10), new anchor.BN(10)];
      let base_price_group = [
        new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 2),
        new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 2),
      ];

      await vizingProgram.methods
        .batchSetThisTradeFeeConfigMap(
          tradeFeeConfig_dapps,
          tradeFeeConfig_destChainIds,
          tradeFeeConfig_moleculars,
          tradeFeeConfig_denominators,
          base_price_group
        )
        .accounts({
          vizing: vizingPadConfigs,
          mappingFeeConfig: vizingGasSystem,
          user: gasPoolAdminKeyPair.publicKey,
        })
        .signers([gasPoolAdminKeyPair])
        .rpc();
    }

    {
      const [recordMessageAuthority, recordMessageBump] =
        anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("init_current_record_message")],
          vizingProgram.programId
        );

      await vizingProgram.methods
        .initRecordMessage()
        .accounts({
          currentRecordMessage: recordMessageAuthority,
          user: provider.wallet.publicKey,
        })
        .signers([])
        .rpc();
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

  it("Launch", async () => {
    const message = {
      mode: 1,
      targetProgram: EVM_address_buffer,
      executeGasLimit: new anchor.BN(1),
      maxFeePerGas: new anchor.BN(2000000000),
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
      value: new anchor.BN(anchor.web3.LAMPORTS_PER_SOL),
      destChainid: new anchor.BN(28516),
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
            mappingFeeConfig: vizingGasSystem,
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

      const feePayerBalanceBefore = await provider.connection.getBalance(
        provider.wallet.publicKey
      );

      const tx = await vizingProgram.methods
        .launch(launchParams)
        .accounts({
          vizingAppFeePayer: provider.wallet.publicKey,
          vizingAppMessageAuthority: provider.wallet.publicKey,
          vizingPadConfig: vizingPadConfigs,
          vizingPadFeeCollector: feeCollectorKeyPair.publicKey,
          mappingFeeConfig: vizingGasSystem,
        })
        .rpc();
      console.log(`launch: ${tx}`);

      const feeReceiverBalanceAfter = await provider.connection.getBalance(
        feeCollectorKeyPair.publicKey
      );

      const feePayerBalanceAfter = await provider.connection.getBalance(
        provider.wallet.publicKey
      );

      const feePayerDiff = feePayerBalanceBefore - feePayerBalanceAfter;

      console.log(
        `feeReceiverBalanceBefore: ${
          feeReceiverBalanceBefore / anchor.web3.LAMPORTS_PER_SOL
        }, feeReceiverBalanceAfter: ${
          feeReceiverBalanceAfter / anchor.web3.LAMPORTS_PER_SOL
        }, feePayerDiff: ${feePayerDiff / anchor.web3.LAMPORTS_PER_SOL}`
      );

      expect(feeReceiverBalanceAfter).to.greaterThan(feeReceiverBalanceBefore);
    }

    {
      console.log("###test loop start");
      let id = new anchor.BN(100);
      for (let i = 0; i < 10; i++) {
        const newLaunchParams = {
          ...launchParams,
          destChainid: id.add(new anchor.BN(i)),
          message: {
            ...launchParams.message,
            maxFeePerGas: new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * (i + 1)),
          },
        };

        const feeReceiverBalanceBefore = await provider.connection.getBalance(
          feeCollectorKeyPair.publicKey
        );

        const tx = await vizingProgram.methods
          .launch(newLaunchParams)
          .accounts({
            vizingAppFeePayer: provider.wallet.publicKey,
            vizingAppMessageAuthority: provider.wallet.publicKey,
            vizingPadConfig: vizingPadConfigs,
            vizingPadFeeCollector: feeCollectorKeyPair.publicKey,
            mappingFeeConfig: vizingGasSystem,
          })
          .rpc();

        const feeReceiverBalanceAfter = await provider.connection.getBalance(
          feeCollectorKeyPair.publicKey
        );

        const diff = feeReceiverBalanceAfter - feeReceiverBalanceBefore;

        console.log(
          `chain id: ${id.add(new anchor.BN(i))}, price: ${
            newLaunchParams.message.maxFeePerGas
          },feeReceiverBalanceBefore: ${
            feeReceiverBalanceBefore / anchor.web3.LAMPORTS_PER_SOL
          }, feeReceiverBalanceAfter: ${
            feeReceiverBalanceAfter / anchor.web3.LAMPORTS_PER_SOL
          }, diff: ${diff / anchor.web3.LAMPORTS_PER_SOL}`
        );

        expect(diff).to.equal(
          anchor.web3.LAMPORTS_PER_SOL * (i + 1) +
            newLaunchParams.value.toNumber()
        );
      }
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
          mappingFeeConfig: vizingGasSystem,
        })
        .rpc();
      console.log(`launchVizing: ${tx}`);

      const feeReceiverBalanceAfter = await provider.connection.getBalance(
        feeCollectorKeyPair.publicKey
      );

      const diff = feeReceiverBalanceAfter - feeReceiverBalanceBefore;

      console.log(
        `feeReceiverBalanceBefore: ${
          feeReceiverBalanceBefore / anchor.web3.LAMPORTS_PER_SOL
        }, feeReceiverBalanceAfter: ${
          feeReceiverBalanceAfter / anchor.web3.LAMPORTS_PER_SOL
        }, diff: ${diff / anchor.web3.LAMPORTS_PER_SOL}`
      );

      expect(diff).to.equal(anchor.web3.LAMPORTS_PER_SOL + 718);
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
    console.log(
      `resultDataAccount: ${resultDataAccount.toBase58()}, bump: ${resultDataBump}`
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
        solPdaReceiver: solPdaReceiver,
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

      expect(fetchedVizingAppConfig.solPdaReceiver.toBase58()).to.equal(
        solReceiver.toBase58()
      );

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
      mode: 1,
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
      srcChainid: new anchor.BN(28516),
      srcTxHash: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      srcContract: EVM_address_buffer,
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
});
