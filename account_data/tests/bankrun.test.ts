import { describe, it } from "node:test";
import * as anchor from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { PublicKey } from '@solana/web3.js';
import {BankrunProvider} from 'anchor-bankrun';
import {startAnchor} from 'anchor-bankrun';
import { AccountData } from "../target/types/account_data";

const IDL = require('../target/idl/account_data.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Account Data!', async () => {
    const context = startAnchor('', [{name: 'anchor program example!', programId: PROGRAM_ID}], []);
    const provider = new BankrunProvider(await context);

    const payer = provider.wallet as anchor.Wallet;
    const program = new anchor.Program<AccountData>(IDL, provider);
    const addressInfoAccount = new Keypair();
    it('Create the address info account', async () => {
        console.log(`Payer Address: ${payer.publicKey}`);
        console.log(`Address Info Act: ${addressInfoAccount.publicKey}`);

        const addressInfo =  {
            name: 'Kien',
            house_number: 3,
            street: '1 Distrist',
        }
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
})
