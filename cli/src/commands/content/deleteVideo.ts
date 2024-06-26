import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import { flags } from '@oclif/command'
import { PalletContentIterableEnumsChannelActionPermission as ChannelActionPermission } from '@polkadot/types/lookup'
import BN from 'bn.js'
import chalk from 'chalk'
import { formatBalance } from '@polkadot/util'
import { createType } from '@joystream/types'
import ExitCodes from '../../ExitCodes'

export default class DeleteVideoCommand extends ContentDirectoryCommandBase {
  static description = 'Delete the video and optionally all associated data objects.'

  protected requiresQueryNode = true

  static flags = {
    videoId: flags.integer({
      char: 'v',
      required: true,
      description: 'ID of the Video',
    }),
    force: flags.boolean({
      char: 'f',
      default: false,
      description: 'Force-remove all associated video data objects',
    }),
    context: ContentDirectoryCommandBase.channelManagementContextFlag,
    ...ContentDirectoryCommandBase.flags,
  }

  async getDataObjectsInfo(videoId: number): Promise<[string, BN][]> {
    const dataObjects = await this.getQNApi().dataObjectsByVideoId(videoId.toString())

    if (dataObjects.length) {
      this.log('Following data objects are still associated with the video:')
      dataObjects.forEach((o) => {
        this.log(`${o.id} - ${o.type.__typename}`)
      })
    }

    return dataObjects.map((o) => [o.id, new BN(o.stateBloatBond)])
  }

  async run(): Promise<void> {
    const { videoId, force, context } = this.parse(DeleteVideoCommand).flags
    // Ensure video exists
    const video = await this.getApi().videoById(videoId)
    const channel = await this.getApi().channelById(video.inChannel.toNumber())
    // Context
    const [actor, address] = await this.getChannelManagementActor(channel, context)

    const dataObjectsInfo = await this.getDataObjectsInfo(videoId)

    // Ensure actor is authorized to perform video deletion
    const requiredPermissions: ChannelActionPermission['type'][] = dataObjectsInfo.length
      ? ['DeleteVideo', 'ManageVideoAssets']
      : ['DeleteVideo']
    if (!(await this.hasRequiredChannelAgentPermissions(actor, channel, requiredPermissions))) {
      this.error(
        `Only channel owner or collaborator with ${requiredPermissions} permissions can perform this deletion!`,
        {
          exit: ExitCodes.AccessDenied,
        }
      )
    }

    if (dataObjectsInfo.length) {
      if (!force) {
        this.error(`Cannot remove associated data objects unless ${chalk.magentaBright('--force')} flag is used`, {
          exit: ExitCodes.InvalidInput,
        })
      }
      const dataObjectsStateBloatBond = dataObjectsInfo.reduce((sum, [, bloatBond]) => sum.add(bloatBond), new BN(0))
      this.log(
        `Video state bloat bond of ${chalk.cyanBright(
          formatBalance(video.videoStateBloatBond.amount)
        )} will be transferred to ${chalk.magentaBright(
          video.videoStateBloatBond.repaymentRestrictedTo.unwrapOr(address).toString()
        )}\n` +
          `Data objects state bloat bond of ${chalk.cyanBright(
            formatBalance(dataObjectsStateBloatBond)
          )} will be repaid with accordance to the bloat bond policy.`
      )
    }

    await this.requireConfirmation(
      `Are you sure you want to remove video ${chalk.magentaBright(videoId)}${
        force ? ' and all associated data objects' : ''
      }?`
    )

    await this.sendAndFollowNamedTx(await this.getDecodedPair(address), 'content', 'deleteVideo', [
      actor,
      videoId,
      createType('u64', dataObjectsInfo.length),
      createType('Option<u32>', await this.getStorageBucketsNumWitness(video.inChannel)),
    ])
  }
}
