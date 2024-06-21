/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category AcceptParticipant
 * @category generated
 */
export const AcceptParticipantStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number
}>([['instructionDiscriminator', beet.u8]], 'AcceptParticipantInstructionArgs')

export const acceptParticipantInstructionDiscriminator = 5

/**
 * Creates a _AcceptParticipant_ instruction.
 *
 * @category Instructions
 * @category AcceptParticipant
 * @category generated
 */
export function createAcceptParticipantInstruction(
  programId = new web3.PublicKey('BNsqbpyoGuh66NJh5tRw6DNqy6y6X9s6LgF6yunDGRKt')
) {
  const [data] = AcceptParticipantStruct.serialize({
    instructionDiscriminator: acceptParticipantInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = []

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}