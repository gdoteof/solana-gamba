import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program } from "@project-serum/anchor";
import { Wallet } from '@project-serum/anchor/dist/cjs/provider';
import { Gamba } from "../../target/types/gamba";

export class GambaUtils {
  
    conn: Connection;
    wallet: Wallet;
    program: Program<Gamba>;
  
    constructor(conn: Connection, wallet: Wallet, program: Program<Gamba>, devnet: boolean) {
      this.conn = conn;
      this.wallet = wallet;
      this.program = program;
    }

    public async init_gamba(): Promise<[PublicKey, number]> {
        const [_gamba_pda, _gamba_bump] = await this.find_gamba_address();

        const tx = await this.program.rpc.initializeGamba(
        _gamba_bump , 
        this.wallet.publicKey, { 
            accounts: {
            gambaAccount: _gamba_pda,
            authority: this.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        });

        return [_gamba_pda, _gamba_bump];
    }

    public async init_user(user: Keypair, name: String): Promise<[PublicKey, number]> {
        const [_gamba_pda, _gamba_bump] = await this.find_gamba_address();

        const [_user_account_pda, _user_account_bump] = await PublicKey.findProgramAddress(
        [ user.publicKey.encode().reverse(),
            Buffer.from(anchor.utils.bytes.utf8.encode("user_account"))],
        this.program.programId
        );

        const tx = await this.program.rpc.initializeUser(
            _gamba_bump,
            _user_account_bump, 
            name, 
            user.publicKey,
            { 
                accounts: {
                    userAccount: _user_account_pda,
                    gambaAccount: _gamba_pda,
                    authority: user.publicKey,
                    systemProgram: SystemProgram.programId,
                }, 
                signers: [ user ]
            }
        );

        return [_user_account_pda, _user_account_bump];
    }

    public async find_gamba_address(): Promise<[PublicKey, number]> {
        return await PublicKey.findProgramAddress(
        [ Buffer.from(anchor.utils.bytes.utf8.encode("gamba"))],
        this.program.programId
        );
    }
}