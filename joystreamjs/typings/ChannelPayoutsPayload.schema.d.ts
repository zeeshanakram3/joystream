/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type Side = 0 | 1
export type ChannelPayoutsPayload = ChannelPayoutProof[]

export interface ChannelPayoutProof {
  channelId: number
  cumulativeRewardEarned: string
  merkleBranch: ProofElement[]
  reason: string
}
export interface ProofElement {
  hash: string
  side: Side
}
