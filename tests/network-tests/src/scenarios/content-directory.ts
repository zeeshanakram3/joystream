import createAndUpdateChannel from '../flows/clis/createAndUpdateChannel'
import commentsAndReactions from '../flows/content/commentsAndReactions'
import videoCategories from '../flows/content/videoCategories'
import curatorModerationActions from '../flows/content/curatorModerationActions'
import nftAuctionAndOffers from '../flows/content/nftAuctionAndOffers'
import leadOpening from '../flows/working-groups/leadOpening'
import { scenario } from '../Scenario'

// eslint-disable-next-line @typescript-eslint/no-floating-promises
scenario('Content directory', async ({ job }) => {
  const leadSetupJob = job('Set content working group leads', leadOpening(true, ['contentWorkingGroup']))

  // following jobs must be run sequentially due to some QN queries that could interfere
  const channelJob = job('Create and Update Channel with assets', createAndUpdateChannel).requires(leadSetupJob)
  const videoCategoriesJob = job('video categories', videoCategories).requires(channelJob)
  /* TODO: fix this test
  const videoCountersJob = job('check active video counters', activeVideoCounters).requires(videoCategoriesJob)
  const nftAuctionAndOffersJob = job('nft auction and offers', nftAuctionAndOffers).after(videoCountersJob)
  */
  const nftAuctionAndOffersJob = job('nft auction and offers', nftAuctionAndOffers).after(videoCategoriesJob)
  const curatorModerationActionsJob = job('curator moderation actions', curatorModerationActions).after(
    nftAuctionAndOffersJob
  )
  job('video comments and reactions', commentsAndReactions).after(curatorModerationActionsJob)
})
