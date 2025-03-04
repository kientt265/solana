import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from '@solana/web3.js';
import { AccountData } from "../target/types/account_data";



describe("account_data", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.AccountData as Program<AccountData>;
  const addressInfoAccount = new Keypair();
  it('Create the address info account', async () => {
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    // Instruction Ix data
    const addressInfo = {
      name: 'Joe C',
      house_number: 136,
      street: 'Mile High Dr.',

    };

    await program.methods
      .initialize(addressInfo.name, addressInfo.house_number, addressInfo.street)
      .accounts({
        dataInfo: addressInfoAccount.publicKey,
        user: payer.publicKey,
      })
      .signers([addressInfoAccount])
      .rpc();
  });

  it("Read the new account's data", async () => {
    const addressInfo = await program.account.dataInfo.fetch(addressInfoAccount.publicKey);
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.houseNumber}`);
    console.log(`Street   : ${addressInfo.street}`);

  });
});

