import { BaseQueryNodeFixture } from '../../Fixture'
import { JoystreamCLI, ICreatedVideoData } from '../../cli/joystream'
import { QueryNodeApi } from '../../QueryNodeApi'
import { Api } from '../../Api'
import * as path from 'path'
import { getVideoDefaults, getChannelDefaults } from './contentTemplates'
import { IMember } from './createMembersAndCurators'
import { ChannelUpdateInputParameters } from '@joystream/cli/src/Types'

export const cliExamplesFolderPath = path.dirname(require.resolve('@joystream/cli/package.json')) + '/examples/content'

export class CreateChannelsAndVideosFixture extends BaseQueryNodeFixture {
  private cli: JoystreamCLI
  private channelCount: number
  private videoCount: number
  private videoCategoryId: string
  private author: IMember
  private channelCollaborators: NonNullable<ChannelUpdateInputParameters['collaborators']>
  private createdItems: {
    channelIds: number[]
    videosData: ICreatedVideoData[]
  }

  constructor(
    api: Api,
    query: QueryNodeApi,
    cli: JoystreamCLI,
    channelCount: number,
    videoCount: number,
    videoCategoryId: string,
    author: IMember,
    channelCollaborators: ChannelUpdateInputParameters['collaborators'] = []
  ) {
    super(api, query)
    this.cli = cli
    this.channelCount = channelCount
    this.videoCount = videoCount
    this.videoCategoryId = videoCategoryId
    this.author = author
    this.channelCollaborators = channelCollaborators

    this.createdItems = {
      channelIds: [],
      videosData: [],
    }
  }

  public getCreatedItems() {
    return this.createdItems
  }

  /*
    Execute this Fixture.
  */
  public async execute(): Promise<void> {
    this.debug('Setting author')
    await this.cli.importAccount(this.author.keyringPair)

    this.debug('Creating channels')

    this.createdItems.channelIds = await this.createChannels(this.channelCount, this.channelCollaborators)

    this.debug('Creating videos')
    this.createdItems.videosData = await this.createVideos(
      this.videoCount,
      this.createdItems.channelIds[0],
      this.videoCategoryId
    )
  }

  /**
    Creates a new channel.
  */
  private async createChannels(
    count: number,
    channelCollaborators: NonNullable<ChannelUpdateInputParameters['collaborators']>
  ): Promise<number[]> {
    const createdIds = await this.createCommonEntities(count, (index) =>
      this.cli.createChannel(
        {
          ...getChannelDefaults(cliExamplesFolderPath),
          collaborators: channelCollaborators,
        },
        ['--context', 'Member', '--useMemberId', this.author.memberId.toString()]
      )
    )

    return createdIds
  }

  /**
    Creates a new video.

    Note: Assets have to be accepted later on for videos to be counted as active.
  */
  private async createVideos(count: number, channelId: number, videoCategoryId: string): Promise<ICreatedVideoData[]> {
    const createVideo = async () => {
      return await this.cli.createVideo(channelId, {
        ...getVideoDefaults(cliExamplesFolderPath),
        category: videoCategoryId,
      })
    }
    const newVideosData = (await this.createCommonEntities(count, createVideo)) as ICreatedVideoData[]

    return newVideosData
  }

  /**
    Creates a bunch of content entities.
  */
  private async createCommonEntities<T>(count: number, createPromise: (index: number) => Promise<T>): Promise<T[]> {
    const createdIds = await Array.from(Array(count).keys()).reduce(async (accPromise, index: number) => {
      const acc = await accPromise
      const createdId = await createPromise(index)

      return [...acc, createdId]
    }, Promise.resolve([]) as Promise<T[]>)

    return createdIds
  }
}
