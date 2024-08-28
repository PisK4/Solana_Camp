import * as anchor from "@coral-xyz/anchor";
import { Program, Coder } from "@coral-xyz/anchor";
import { expect } from "chai";

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
  const vizingAppProgram = anchor.workspace.VizingApp as Program<VizingApp>;
  const vizingAppMockProgram = anchor.workspace
    .VizingAppMock as Program<VizingAppMock>;
  const vizingPadSettingsSeed = Buffer.from("Vizing_Pad_Settings_Seed");
  // const relayerSettingsSeed = Buffer.from("Relayer_Settings_Seed");
  const vizingAuthoritySeed = Buffer.from("Vizing_Authority_Seed");
  const vizingAppConfigSeed = Buffer.from("Vizing_App_Config_Seed");
  const vizingAppSolReceiverSeed = Buffer.from("Vizing_App_Sol_Receiver_Seed");

  let vizingPadSettings: anchor.web3.PublicKey;
  let relayerSettings: anchor.web3.PublicKey;
  let vizingAuthority: anchor.web3.PublicKey;
  let vizingAppConfig: anchor.web3.PublicKey;

  const feeReceiverKeyPair = anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("fee_receiver"))
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

  it("account setup", async () => {
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
      const seed = [vizingPadSettingsSeed];
      const [vizingPad, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      vizingPadSettings = vizingPad;
      vizingPadBump = bump;

      console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);
    }

    const initParams = {
      owner: provider.wallet.publicKey,
      feeReceiver: feeReceiverKeyPair.publicKey,
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
    }

    {
      const tx = await vizingProgram.methods
        .initializeVizingPad(initParams)
        .accounts({
          vizing: vizingPadSettings,
          vizingAuthority: vizingAuthority,
          payer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log(`initialize: ${tx}`);

      const vizingPadAccount =
        await vizingProgram.account.vizingPadSettings.fetch(vizingPadSettings);

      expect(vizingPadAccount.owner.toBase58()).to.equal(
        provider.wallet.publicKey.toBase58()
      );
      expect(vizingPadAccount.engineAdmin.toBase58()).to.equal(
        initParams.engineAdmin.toBase58()
      );
      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        initParams.feeReceiver.toBase58()
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
            vizing: vizingPadSettings,
            vizingAuthority: vizingAuthority,
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

  it("modify Vizing Pad", async () => {
    const modifyParams = {
      owner: provider.wallet.publicKey,
      feeReceiver: feeReceiverKeyPair.publicKey,
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
            vizing: vizingPadSettings,
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
          vizing: vizingPadSettings,
        })
        .rpc();
      console.log(`modify: ${tx}`);

      const vizingPadAccount =
        await vizingProgram.account.vizingPadSettings.fetch(vizingPadSettings);

      expect(vizingPadAccount.owner.toBase58()).to.equal(
        provider.wallet.publicKey.toBase58()
      );
      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        modifyParams.feeReceiver.toBase58()
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
            vizing: vizingPadSettings,
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
          vizing: vizingPadSettings,
        })
        .signers([gasPoolAdminKeyPair])
        .rpc();
      console.log(`grant fee collector: ${tx}`);

      let vizingPadAccount =
        await vizingProgram.account.vizingPadSettings.fetch(vizingPadSettings);

      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        newFeeCollector.publicKey.toBase58()
      );

      await vizingProgram.methods
        .grantFeeCollector(feeReceiverKeyPair.publicKey)
        .accounts({
          gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
          vizing: vizingPadSettings,
        })
        .signers([gasPoolAdminKeyPair])
        .rpc();

      vizingPadAccount = await vizingProgram.account.vizingPadSettings.fetch(
        vizingPadSettings
      );

      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        feeReceiverKeyPair.publicKey.toBase58()
      );
    }
  });

  it("Launch", async () => {
    const message = {
      mode: 5,
      targetProgram: anchor.web3.Keypair.generate().publicKey,
      executeGasLimit: new anchor.BN(6),
      maxFeePerGas: new anchor.BN(7),
      signature: Buffer.from("1234"),
    };

    const launchParams = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: provider.wallet.publicKey,
      sender: provider.wallet.publicKey,
      value: new anchor.BN(3),
      destChainid: new anchor.BN(4),
      additionParams: Buffer.alloc(0),
      message: message,
    };

    {
      try {
        await vizingProgram.methods
          .launch(launchParams)
          .accounts({
            feePayer: provider.wallet.publicKey,
            messageAuthority: provider.wallet.publicKey,
            vizing: vizingPadSettings,
            feeCollector: anchor.web3.Keypair.generate().publicKey,
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
        feeReceiverKeyPair.publicKey
      );

      const tx = await vizingProgram.methods
        .launch(launchParams)
        .accounts({
          feePayer: provider.wallet.publicKey,
          messageAuthority: provider.wallet.publicKey,
          vizing: vizingPadSettings,
          feeCollector: feeReceiverKeyPair.publicKey,
        })
        .rpc();
      console.log(`launch: ${tx}`);

      const feeReceiverBalanceAfter = await provider.connection.getBalance(
        feeReceiverKeyPair.publicKey
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

    const [solReceiver2, bump2] = anchor.web3.PublicKey.findProgramAddressSync(
      [vizingAppSolReceiverSeed],
      vizingAppProgram.programId
    );

    await vizingAppProgram.methods
      .initialize()
      .accounts({
        solPdaReceiver: solReceiver2,
        payer: provider.wallet.publicKey,
      })
      .rpc();

    await vizingAppMockProgram.methods
      .initialize()
      .accounts({
        resultAccount: resultDataAccount,
        solPdaReceiver: solPdaReceiver,
        payer: provider.wallet.publicKey,
      })
      .rpc();

    const message = {
      mode: 5,
      targetProgram: targetProgram,
      executeGasLimit: new anchor.BN(6),
      maxFeePerGas: new anchor.BN(7),
      signature: Buffer.alloc(0),
    };

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
      srcContract: anchor.web3.Keypair.generate().publicKey,
      srcChainNonce: new anchor.BN(4),
      sender: provider.wallet.publicKey,
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
            vizing: vizingPadSettings,
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
            vizing: vizingPadSettings,
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
          vizing: vizingPadSettings,
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
            pubkey: vizingPadSettings,
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: solPdaReceiver,
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
