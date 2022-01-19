import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Gamba } from '../target/types/gamba';
import { PublicKey, SystemProgram, Transaction } from '@solana/web3.js';
import { assert } from 'chai';



describe('gamba', () => {

  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.Provider.local();
  const providerWallet = provider.wallet;


  const program = anchor.workspace.Gamba as Program<Gamba>;

  it('gamba initializes', async () => {
    const userAccount = anchor.web3.Keypair.generate();

    const [_gamba_pda, _gamba_bump] = await PublicKey.findProgramAddress(
      [ Buffer.from(anchor.utils.bytes.utf8.encode("gamba"))],
      program.programId
    );

    const tx = await program.rpc.initializeGamba(
      _gamba_bump , 
      providerWallet.publicKey, { 
        accounts: {
          gambaAccount: _gamba_pda,
          authority: providerWallet.publicKey,
          systemProgram: SystemProgram.programId,
      },
    });

    const state = await program.account.gambaAccount.fetch(_gamba_pda);

    assert(state.authority.equals(providerWallet.publicKey))
  });

  it('user initializes', async () => {
    const userAccount = anchor.web3.Keypair.generate();

    const [_user_account_pda, _user_account_bump] = await PublicKey.findProgramAddress(
      [ providerWallet.publicKey.encode().reverse(),
        Buffer.from(anchor.utils.bytes.utf8.encode("user_account"))],
      program.programId
    );

    const tx = await program.rpc.initializeUser(
      _user_account_bump, 
      "bobby tables", 
      providerWallet.publicKey,
      { 
        accounts: {
          userAccount: _user_account_pda,
          authority: providerWallet.publicKey,
          systemProgram: SystemProgram.programId,
      },
    });

    const state = await program.account.userAccount.fetch(_user_account_pda);

    assert.equal(state.userName, "bobby tables");
    assert(state.authority.equals(providerWallet.publicKey))
  });

  it('epoch initializes', async () => {
    const [_gamba_pda, _gamba_bump] = await PublicKey.findProgramAddress(
      [ Buffer.from(anchor.utils.bytes.utf8.encode("gamba"))],
      program.programId
    );

    const gamba_state = await program.account.gambaAccount.fetch(_gamba_pda);
    assert.equal(gamba_state.currentOpenEpoch, 0);

    const next_epoch = gamba_state.currentOpenEpoch + 1;
    const userAccount = anchor.web3.Keypair.generate();

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
