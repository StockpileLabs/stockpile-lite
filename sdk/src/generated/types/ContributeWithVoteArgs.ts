/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
export type ContributeWithVoteArgs = {
  amount: beet.bignum
  nextBump: beet.COption<number>
}

/**
 * @category userTypes
 * @category generated
 */
export const contributeWithVoteArgsBeet =
  new beet.FixableBeetArgsStruct<ContributeWithVoteArgs>(
    [
      ['amount', beet.u64],
      ['nextBump', beet.coption(beet.u8)],
    ],
    'ContributeWithVoteArgs'
  )
