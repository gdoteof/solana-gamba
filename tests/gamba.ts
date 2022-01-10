import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Gamba } from '../target/types/gamba';

describe('gamba', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Gamba as Program<Gamba>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
