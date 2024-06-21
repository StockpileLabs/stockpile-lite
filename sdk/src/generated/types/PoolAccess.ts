/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import { TokenGateInfo, tokenGateInfoBeet } from './TokenGateInfo'
/**
 * This type is used to derive the {@link PoolAccess} type as well as the de/serializer.
 * However don't refer to it in your code but use the {@link PoolAccess} type instead.
 *
 * @category userTypes
 * @category enums
 * @category generated
 * @private
 */
export type PoolAccessRecord = {
  Open: void /* scalar variant */
  Manual: void /* scalar variant */
  TokenGated: { fields: [TokenGateInfo] }
}

/**
 * Union type respresenting the PoolAccess data enum defined in Rust.
 *
 * NOTE: that it includes a `__kind` property which allows to narrow types in
 * switch/if statements.
 * Additionally `isPoolAccess*` type guards are exposed below to narrow to a specific variant.
 *
 * @category userTypes
 * @category enums
 * @category generated
 */
export type PoolAccess = beet.DataEnumKeyAsKind<PoolAccessRecord>

export const isPoolAccessOpen = (
  x: PoolAccess
): x is PoolAccess & { __kind: 'Open' } => x.__kind === 'Open'
export const isPoolAccessManual = (
  x: PoolAccess
): x is PoolAccess & { __kind: 'Manual' } => x.__kind === 'Manual'
export const isPoolAccessTokenGated = (
  x: PoolAccess
): x is PoolAccess & { __kind: 'TokenGated' } => x.__kind === 'TokenGated'

/**
 * @category userTypes
 * @category generated
 */
export const poolAccessBeet = beet.dataEnum<PoolAccessRecord>([
  ['Open', beet.unit],
  ['Manual', beet.unit],
  [
    'TokenGated',
    new beet.BeetArgsStruct<PoolAccessRecord['TokenGated']>(
      [['fields', beet.fixedSizeTuple([tokenGateInfoBeet])]],
      'PoolAccessRecord["TokenGated"]'
    ),
  ],
]) as beet.FixableBeet<PoolAccess, PoolAccess>