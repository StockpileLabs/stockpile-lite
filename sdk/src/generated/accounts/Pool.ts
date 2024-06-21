/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import * as beetSolana from '@metaplex-foundation/beet-solana'
import { PoolState, poolStateBeet } from '../types/PoolState'
import { PoolAccess, poolAccessBeet } from '../types/PoolAccess'
import { SybilStrategy, sybilStrategyBeet } from '../types/SybilStrategy'

/**
 * Arguments used to create {@link Pool}
 * @category Accounts
 * @category generated
 */
export type PoolArgs = {
  name: string
  start: beet.bignum
  end: beet.bignum
  mint: web3.PublicKey
  admins: web3.PublicKey[]
  poolState: PoolState
  poolAccess: PoolAccess
  sybilStrategy: SybilStrategy
  participantIndex: number
  bump: number
}
/**
 * Holds the data for the {@link Pool} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class Pool implements PoolArgs {
  private constructor(
    readonly name: string,
    readonly start: beet.bignum,
    readonly end: beet.bignum,
    readonly mint: web3.PublicKey,
    readonly admins: web3.PublicKey[],
    readonly poolState: PoolState,
    readonly poolAccess: PoolAccess,
    readonly sybilStrategy: SybilStrategy,
    readonly participantIndex: number,
    readonly bump: number
  ) {}

  /**
   * Creates a {@link Pool} instance from the provided args.
   */
  static fromArgs(args: PoolArgs) {
    return new Pool(
      args.name,
      args.start,
      args.end,
      args.mint,
      args.admins,
      args.poolState,
      args.poolAccess,
      args.sybilStrategy,
      args.participantIndex,
      args.bump
    )
  }

  /**
   * Deserializes the {@link Pool} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [Pool, number] {
    return Pool.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link Pool} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<Pool> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find Pool account at ${address}`)
    }
    return Pool.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'BNsqbpyoGuh66NJh5tRw6DNqy6y6X9s6LgF6yunDGRKt'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, poolBeet)
  }

  /**
   * Deserializes the {@link Pool} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [Pool, number] {
    return poolBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link Pool} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return poolBeet.serialize(this)
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link Pool} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: PoolArgs) {
    const instance = Pool.fromArgs(args)
    return poolBeet.toFixedFromValue(instance).byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link Pool} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: PoolArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      Pool.byteSize(args),
      commitment
    )
  }

  /**
   * Returns a readable version of {@link Pool} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      name: this.name,
      start: (() => {
        const x = <{ toNumber: () => number }>this.start
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      end: (() => {
        const x = <{ toNumber: () => number }>this.end
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      mint: this.mint.toBase58(),
      admins: this.admins,
      poolState: 'PoolState.' + PoolState[this.poolState],
      poolAccess: this.poolAccess.__kind,
      sybilStrategy: this.sybilStrategy.__kind,
      participantIndex: this.participantIndex,
      bump: this.bump,
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const poolBeet = new beet.FixableBeetStruct<Pool, PoolArgs>(
  [
    ['name', beet.utf8String],
    ['start', beet.u64],
    ['end', beet.u64],
    ['mint', beetSolana.publicKey],
    ['admins', beet.array(beetSolana.publicKey)],
    ['poolState', poolStateBeet],
    ['poolAccess', poolAccessBeet],
    ['sybilStrategy', sybilStrategyBeet],
    ['participantIndex', beet.u8],
    ['bump', beet.u8],
  ],
  Pool.fromArgs,
  'Pool'
)
