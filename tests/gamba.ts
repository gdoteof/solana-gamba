import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Gamba } from '../target/types/gamba';
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { assert } from 'chai';
import { Wallet } from '@project-serum/anchor/dist/cjs/provider';
import {GambaUtils} from './utils/gamba_utils'



describe('gamba', () => {

  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.Provider.local();
  const providerWallet : Wallet = provider.wallet;


  const program = anchor.workspace.Gamba as Program<Gamba>;

  it('gamba initializes', async () => {
    const gamba = new GambaUtils(provider.connection, providerWallet, program, true);

    const [_gamba_pda, _gamba_bump] = await  gamba.init_gamba();

    const state = await program.account.gambaAccount.fetch(_gamba_pda);

    assert(state.authority.equals(providerWallet.publicKey))
  });

  it('user initializes', async () => {
    const userAccount = anchor.web3.Keypair.generate();
    const gamba = new GambaUtils(provider.connection, providerWallet, program, true);
    const airdropTxnId = await provider.connection.requestAirdrop(userAccount.publicKey, LAMPORTS_PER_SOL/10);
    console.log("airdrop tx:", airdropTxnId);

    var bal = 0;

    while (bal == 0) {
      bal = await provider.connection.getBalance(userAccount.publicKey);
      await new Promise(resolve => setTimeout(resolve,1000));
      console.log("sleeping 1 second")
    }
    console.log("bal is: ", bal)

    const [_user_account_pda, _user_account_bump] = await gamba.init_user(userAccount, "bobby tables");

    const state = await program.account.userAccount.fetch(_user_account_pda);

    assert.equal(state.userName, "bobby tables");
    assert(state.authority.equals(userAccount.publicKey))
  });

  it('epoch initializes', async () => {
    const [_gamba_pda, _gamba_bump] = await PublicKey.findProgramAddress(
      [ Buffer.from(anchor.utils.bytes.utf8.encode("gamba"))],
      program.programId
    );

    const gamba_state = await program.account.gambaAccount.fetch(_gamba_pda);
    assert.equal(gamba_state.currentOpenEpoch, 0);

    const next_epoch = gamba_state.currentOpenEpoch + 1;

    const [_epoch_account_pda, _epoch_account_bump] = await PublicKey.findProgramAddress(
      [ Buffer.from(new Int32Array([next_epoch]).buffer),
        Buffer.from(anchor.utils.bytes.utf8.encode("epoch_account"))],
      program.programId
    );

    const tx = await program.rpc.initializeEpoch(
      next_epoch,
      _epoch_account_bump,
      _gamba_bump,
      { 
        accounts: {
          epochAccount: _epoch_account_pda,
          gambaAccount: _gamba_pda,
          authority: providerWallet.publicKey,
          systemProgram: SystemProgram.programId,
      },
    });

    const epoch_state = await program.account.epochAccount.fetch(_epoch_account_pda);
    assert(epoch_state.authority.equals(providerWallet.publicKey));

    assert.equal(epoch_state.epoch, next_epoch);
    assert.equal(epoch_state.maxBets, 32);
    assert.equal(epoch_state.numBets, 0);

    const gamba_state_after = await program.account.gambaAccount.fetch(_gamba_pda);
    assert.equal(gamba_state_after.currentOpenEpoch, next_epoch);

  });

});
