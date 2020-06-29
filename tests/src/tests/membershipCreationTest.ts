import { WsProvider } from '@polkadot/api';
import { registerJoystreamTypes } from '@joystream/types';
import { Keyring } from '@polkadot/keyring';
import { assert } from 'chai';
import { KeyringPair } from '@polkadot/keyring/types';
import BN = require('bn.js');
import { ApiWrapper } from '../utils/apiWrapper';
import { initConfig } from '../utils/config';

describe('Membership integration tests', () => {
  initConfig();
  const keyring = new Keyring({ type: 'sr25519' });
  const nKeyPairs: KeyringPair[] = new Array();
  const N: number = +process.env.MEMBERSHIP_CREATION_N!;
  const paidTerms: number = +process.env.MEMBERSHIP_PAID_TERMS!;
  const nodeUrl: string = process.env.NODE_URL!;
  const sudoUri: string = process.env.SUDO_ACCOUNT_URI!;
  const defaultTimeout: number = 30000;
  let apiWrapper: ApiWrapper;
  let sudo: KeyringPair;
  let aKeyPair: KeyringPair;
  let membershipFee: BN;
  let membershipTransactionFee: BN;

  before(async function () {
    this.timeout(defaultTimeout);
    registerJoystreamTypes();
    const provider = new WsProvider(nodeUrl);
    apiWrapper = await ApiWrapper.create(provider);
    sudo = keyring.addFromUri(sudoUri);
    for (let i = 0; i < N; i++) {
      nKeyPairs.push(keyring.addFromUri(i.toString()));
    }
    aKeyPair = keyring.addFromUri('A');
    membershipFee = await apiWrapper.getMembershipFee(paidTerms);
    membershipTransactionFee = apiWrapper.estimateBuyMembershipFee(
      sudo,
      paidTerms,
      'member_name_which_is_longer_than_expected'
    );
    await apiWrapper.transferBalanceToAccounts(sudo, nKeyPairs, membershipTransactionFee.add(new BN(membershipFee)));
    await apiWrapper.transferBalance(sudo, aKeyPair.address, membershipTransactionFee);
  });

  it('Buy membeship is accepted with sufficient funds', async () => {
    await Promise.all(
      nKeyPairs.map(async (keyPair, index) => {
        await apiWrapper.buyMembership(keyPair, paidTerms, `new_member_${index}`);
      })
    );
    nKeyPairs.forEach((keyPair, index) =>
      apiWrapper
        .getMembership(keyPair.address)
        .then(membership => assert(!membership.isEmpty, `Account ${index} is not a member`))
    );
  }).timeout(defaultTimeout);

  it('Account A can not buy the membership with insufficient funds', async () => {
    apiWrapper
      .getBalance(aKeyPair.address)
      .then(balance =>
        assert(
          balance.toBn() < membershipFee.add(membershipTransactionFee),
          'Account A already have sufficient balance to purchase membership'
        )
      );
    await apiWrapper.buyMembership(aKeyPair, paidTerms, 'late_member', true);
    apiWrapper.getMembership(aKeyPair.address).then(membership => assert(membership.isEmpty, 'Account A is a member'));
  }).timeout(defaultTimeout);

  it('Account A was able to buy the membership with insufficient funds', async () => {
    await apiWrapper.transferBalance(sudo, aKeyPair.address, membershipFee.add(membershipTransactionFee));
    apiWrapper
      .getBalance(aKeyPair.address)
      .then(balance =>
        assert(balance.toBn() >= membershipFee, 'The account balance is insufficient to purchase membership')
      );
    await apiWrapper.buyMembership(aKeyPair, paidTerms, 'late_member');
    apiWrapper
      .getMembership(aKeyPair.address)
      .then(membership => assert(!membership.isEmpty, 'Account A is a not member'));
  }).timeout(defaultTimeout);

  after(() => {
    apiWrapper.close();
  });
});