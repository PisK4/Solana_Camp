import * as anchor from "@coral-xyz/anchor";
import { Program, Coder } from "@coral-xyz/anchor";
import { expect } from "chai";

import { VizingCore } from "../target/types/vizing_core";

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
  const vizingPadSettingsSeed = Buffer.from("Vizing_Pad_Settings_Seed");

  let vizingPadSettings: anchor.web3.PublicKey;

  const engineAdminKeyPairs = [
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("engine_admin_1"))
    ),
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("engine_admin_2"))
    ),
  ];
  const stationAdminKeyPairs = [
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("station_admin_1"))
    ),
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("station_admin_2"))
    ),
  ];

  const gasPoolAdminKeyPairs = [
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("gas_pool_admin_1"))
    ),
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("gas_pool_admin_2"))
    ),
  ];

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

  it("Initializes Vizing Pad", async () => {
    console.log("start initialize");
    const seed = [vizingPadSettingsSeed];
    const [vizingPad, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seed,
      vizingProgram.programId
    );

    vizingPadSettings = vizingPad;

    console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);

    const initParams = {
      owner: provider.wallet.publicKey,
      engineAdmin: engineAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      stationAdmin: stationAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: gasPoolAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      trustedRelayer: trustedRelayerKeyPairs.map(
        (keypair) => keypair.publicKey
      )[0],
      registeredValidator: registeredValidatorKeyPairs.map(
        (keypair) => keypair.publicKey
      )[0],
    };

    {
      const tx = await vizingProgram.methods
        .initializeVizingPad(initParams)
        .accounts({
          vizing: vizingPad,
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
      expect(vizingPadAccount.stationAdmin.toBase58()).to.equal(
        initParams.stationAdmin.toBase58()
      );
      expect(vizingPadAccount.gasPoolAdmin.toBase58()).to.equal(
        initParams.gasPoolAdmin.toBase58()
      );
      expect(vizingPadAccount.trustedRelayer.toBase58()).to.equal(
        initParams.trustedRelayer.toBase58()
      );
      expect(vizingPadAccount.registeredValidator.toBase58()).to.equal(
        initParams.registeredValidator.toBase58()
      );
      expect(vizingPadAccount.bump).to.equal(bump);
    }

    {
      try {
        await vizingProgram.methods
          .initializeVizingPad(initParams)
          .accounts({
            vizing: vizingPad,
            payer: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.transactionMessage).to.be.eql(
          "Transaction simulation failed: Error processing Instruction 0: custom program error: 0x0"
        );
      }
    }
  });

  it("modify Vizing Pad", async () => {
    const modifyParams = {
      owner: provider.wallet.publicKey,
      engineAdmin: anchor.web3.Keypair.generate().publicKey,
      stationAdmin: stationAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: gasPoolAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      trustedRelayer: trustedRelayerKeyPairs.map(
        (keypair) => keypair.publicKey
      )[0],
      registeredValidator: registeredValidatorKeyPairs.map(
        (keypair) => keypair.publicKey
      )[0],
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
      expect(vizingPadAccount.engineAdmin.toBase58()).to.equal(
        modifyParams.engineAdmin.toBase58()
      );
      expect(vizingPadAccount.stationAdmin.toBase58()).to.equal(
        modifyParams.stationAdmin.toBase58()
      );
      expect(vizingPadAccount.gasPoolAdmin.toBase58()).to.equal(
        modifyParams.gasPoolAdmin.toBase58()
      );
      expect(vizingPadAccount.trustedRelayer.toBase58()).to.equal(
        modifyParams.trustedRelayer.toBase58()
      );
      expect(vizingPadAccount.registeredValidator.toBase58()).to.equal(
        modifyParams.registeredValidator.toBase58()
      );
    }
  });

  it("Launch", async () => {
    const message = {
      mode: 5,
      targetContract: anchor.web3.Keypair.generate().publicKey,
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
      const tx = await vizingProgram.methods
        .launch(launchParams)
        .accounts({
          feePayer: provider.wallet.publicKey,
          messageAuthority: provider.wallet.publicKey,
        })
        .rpc();
      console.log(`launch: ${tx}`);
    }
  });
});
