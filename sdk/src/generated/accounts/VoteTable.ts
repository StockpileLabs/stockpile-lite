/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beetSolana from '@metaplex-foundation/beet-solana'
import * as beet from '@metaplex-foundation/beet'
import { VoteTicket, voteTicketBeet } from '../types/VoteTicket'

/**
 * Arguments used to create {@link VoteTable}
 * @category Accounts
 * @category generated
 */
export type VoteTableArgs = {
  pool: web3.PublicKey
  participant: web3.PublicKey
  table: Map<number, VoteTicket>
  index: number
  bump: number
}
/**
 * Holds the data for the {@link VoteTable} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class VoteTable implements VoteTableArgs {
  private constructor(
    readonly pool: web3.PublicKey,
    readonly participant: web3.PublicKey,
    readonly table: Map<number, VoteTicket>,
    readonly index: number,
    readonly bump: number
  ) {}

  /**
   * Creates a {@link VoteTable} instance from the provided args.
   */
  static fromArgs(args: VoteTableArgs) {
    return new VoteTable(
      args.pool,
      args.participant,
      args.table,
      args.index,
      args.bump
    )
  }

  /**
   * Deserializes the {@link VoteTable} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [VoteTable, number] {
    return VoteTable.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link VoteTable} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<VoteTable> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find VoteTable account at ${address}`)
    }
    return VoteTable.fromAccountInfo(accountInfo, 0)[0]
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
    return beetSolana.GpaBuilder.fromStruct(programId, voteTableBeet)
  }

  /**
   * Deserializes the {@link VoteTable} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [VoteTable, number] {
    return voteTableBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link VoteTable} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return voteTableBeet.serialize(this)
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link VoteTable} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: VoteTableArgs) {
    const instance = VoteTable.fromArgs(args)
    return voteTableBeet.toFixedFromValue(instance).byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link VoteTable} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: VoteTableArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      VoteTable.byteSize(args),
      commitment
    )
  }

  /**
   * Returns a readable version of {@link VoteTable} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      pool: this.pool.toBase58(),
      participant: this.participant.toBase58(),
      table: this.table,
      index: this.index,
      bump: this.bump,
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const voteTableBeet = new beet.FixableBeetStruct<
  VoteTable,
  VoteTableArgs
>(
  [
    ['pool', beetSolana.publicKey],
    ['participant', beetSolana.publicKey],
    ['table', beet.map(beet.u8, voteTicketBeet)],
    ['index', beet.u8],
    ['bump', beet.u8],
  ],
  VoteTable.fromArgs,
  'VoteTable'
)
