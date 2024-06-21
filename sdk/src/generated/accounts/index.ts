export * from './Participant'
export * from './Pool'
export * from './Vault'
export * from './VoteTable'

import { Participant } from './Participant'
import { Pool } from './Pool'
import { Vault } from './Vault'
import { VoteTable } from './VoteTable'

export const accountProviders = { Participant, Pool, Vault, VoteTable }
