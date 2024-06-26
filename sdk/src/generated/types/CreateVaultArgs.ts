/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import { Vault, vaultBeet } from '../accounts/Vault'
export type CreateVaultArgs = {
  vault: Vault
}

/**
 * @category userTypes
 * @category generated
 */
export const createVaultArgsBeet = new beet.BeetArgsStruct<CreateVaultArgs>(
  [['vault', vaultBeet]],
  'CreateVaultArgs'
)
