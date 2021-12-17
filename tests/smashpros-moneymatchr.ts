import * as anchor from '@project-serum/anchor';
import * as assert from 'assert';
import { Program } from '@project-serum/anchor';
import { SmashprosMoneymatchr } from '../target/types/smashpros_moneymatchr';

describe('smashpros-moneymatchr', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SmashprosMoneymatchr as Program<SmashprosMoneymatchr>
  const initiator: anchor.web3.Keypair = anchor.web3.Keypair.generate()

  before(async () => {
    const amount = 10 * anchor.web3.LAMPORTS_PER_SOL
    const tx = await provider.connection.requestAirdrop(initiator.publicKey, amount)
    await provider.connection.confirmTransaction(tx)
    
    it('expect airdrop to work', async () => {
      await provider.connection.getBalance(initiator.publicKey)
    })
  })

  it('creates a program and set amount needed', async () => {
    console.log(await provider.connection.getBalance(initiator.publicKey))
    const uuid = "1"
    const amount = new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL)
    const programId = new anchor.web3.PublicKey(program.programId)
    const wins_needed = new anchor.BN(1)
    
    const [moneymatchr, moneymatchrBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("moneymatchr"),
        initiator.publicKey.toBuffer()
      ],
      programId
    )
    const bump = new anchor.BN(moneymatchrBump)

    const tx = await program.rpc.initialize(bump, amount, wins_needed, uuid, {
      accounts: {
        moneymatchr: moneymatchr,
        player: initiator.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [initiator]
    })

    console.log(await provider.connection.getBalance(initiator.publicKey))
    console.log(await provider.connection.getProgramAccounts(program.programId))

    const moneymatchrr = await program.account.moneymatchr.fetch(moneymatchr)
    console.log(moneymatchrr)
  })
});
