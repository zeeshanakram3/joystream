/* global api, hashing, keyring, types, util, joy */

// run this script with:
// yarn workspace api-scripts script example
//
// or copy and paste the code into the Polkadot-js/apps javascript toolbox at:
// https://polkadot.js.org/apps/#/js
//
// Example works on nicaea release+

const script = async ({ api, hashing, keyring, types, util, joy }) => {
  // Retrieve the chain & node information information via rpc calls
  const [chain, nodeName, nodeVersion, runtimeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version(),
    api.runtimeVersion,
  ])

  console.log(`Chain: ${chain}`)
  console.log(`Software: ${nodeName} v${nodeVersion}`)
  console.log(`Runtime specVersion: ${runtimeVersion.specVersion}`)
}

if (typeof module === 'undefined') {
  // Polkadot-js/apps js-toolbox
  // Available globals [api, hashing, keyring, types, util, joy]
  script({ api, hashing, keyring, types, util, joy })
} else {
  // Node
  module.exports = script
}
