import * as path from 'path'
import { Utils } from '../../utils'
import { Api } from '../../Api'
import { ICreatedPlaylistData, JoystreamCLI } from '../../cli/joystream'
import { BaseQueryNodeFixture } from '../../Fixture'
import { QueryNodeApi } from '../../QueryNodeApi'
import { getPlaylistDefaults } from './contentTemplates'
import { IMember } from './createMembers'
import _ from 'lodash'

const cliExamplesFolderPath = path.dirname(require.resolve('@joystream/cli/package.json')) + '/examples/content'

export class PlaylistActionsFixture extends BaseQueryNodeFixture {
  private cli: JoystreamCLI
  private action: 'CREATE' | 'UPDATE' | 'DELETE'
  private channelId: number
  private videoIds: Long[]
  private author: IMember
  private playlistId: number | undefined
  private createdPlaylist: ICreatedPlaylistData | undefined

  constructor(
    api: Api,
    query: QueryNodeApi,
    cli: JoystreamCLI,
    action: 'CREATE' | 'UPDATE' | 'DELETE',
    channelId: number,
    videoIds: Long[],
    author: IMember,
    playlistId: number | undefined
  ) {
    super(api, query)
    this.cli = cli
    this.channelId = channelId
    this.videoIds = videoIds
    this.author = author
    this.playlistId = playlistId
    this.action = action
  }

  public getCreatedPlaylist(): ICreatedPlaylistData {
    return this.createdPlaylist as ICreatedPlaylistData
  }

  /*
    Execute this Fixture.
  */
  public async execute(): Promise<void> {
    if (this.action === 'CREATE') {
      this.debug('Creating playlist')
      this.createdPlaylist = await this.createPlaylist(this.channelId, this.videoIds)
    } else if (this.action === 'UPDATE') {
      if (!this.playlistId) throw new Error('Playlist ID not provided')
      this.debug('Updating playlist')
      await this.updatePlaylist(this.playlistId, this.videoIds)
    }

    // TODO: assert playlist created events
  }

  /**
    Creates a new playlist.
  */
  private async createPlaylist(channelId: number, videoIds: Long[]): Promise<ICreatedPlaylistData> {
    const newPlaylistsData = await this.cli.createPlaylist(channelId, {
      ...getPlaylistDefaults(cliExamplesFolderPath, videoIds),
    })

    // assert playlist
    this.assertPlaylist(newPlaylistsData.playlistId, videoIds)

    return newPlaylistsData
  }

  /**
    Updates a playlist.
  */
  private async updatePlaylist(playlistId: number, videoIds: Long[]): Promise<void> {
    await this.cli.updatePlaylist(playlistId, {
      ...getPlaylistDefaults(cliExamplesFolderPath, videoIds),
    })

    // assert playlist
    this.assertPlaylist(playlistId, videoIds)
  }

  private async assertPlaylist(playlistId: number, videoIds: Long[]): Promise<void> {
    await this.query.tryQueryWithTimeout(
      () => this.query.playlistById(playlistId.toString()),
      (playlist) => {
        Utils.assert(playlist, 'Playlist not found')
        console.log(
          playlist.videos.map((v) => Number(v.id)),
          videoIds
        )
        Utils.assert(
          _.isEqual(
            playlist.videos.map((v) => Number(v.id)),
            videoIds
          ),
          'Invalid videos in playlist'
        )
      }
    )
  }
}