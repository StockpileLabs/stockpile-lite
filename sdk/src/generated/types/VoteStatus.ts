/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
/**
 * @category enums
 * @category generated
 */
export enum VoteStatus {
  Valid,
  Invalid,
}

/**
 * @category userTypes
 * @category generated
 */
export const voteStatusBeet = beet.fixedScalarEnum(
  VoteStatus
) as beet.FixedSizeBeet<VoteStatus, VoteStatus>