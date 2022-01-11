import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Gamba } from '../target/types/gamba';
import { PublicKey, SystemProgram, Transaction } from '@solana/web3.js';



describe('gamba', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.Provider.local();
  const providerWallet = provider.wallet;


  const program = anchor.workspace.Gamba as Program<Gamba>;

  it('It initializes', async () => {
    // Add your test here.
    const userAccount = anchor.web3.Keypair.generate();

    const [_user_account_pda, _user_account_bump] = await PublicKey.findProgramAddress(
      [ providerWallet.publicKey.encode().reverse(),
        Buffer.from(anchor.utils.bytes.utf8.encode("user_account"))],
      program.programId
    );

    console.log("program id", program.programId.toString());
    console.log("user account pda:", _user_account_pda.toString());
    console.log("user account bump:", _user_account_bump.toString());
    console.log("authority publicid", providerWallet.publicKey.toString());
    console.log("public key encode", providerWallet.publicKey.encode());
    console.log(anchor.utils.bytes.utf8.encode("user_account"));





      const tx = await program.rpc.initialize(_user_account_bump, "bobby tables", {
      accounts: {
        userAccount: _user_account_pda,
        authority: providerWallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    console.log("Your transaction signature", tx);
  });
});
