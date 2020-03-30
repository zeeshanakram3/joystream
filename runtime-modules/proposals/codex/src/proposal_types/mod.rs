pub(crate) mod parameters {
    use crate::{BalanceOf, ProposalParameters};

    // Proposal parameters for the upgrade runtime proposal
    pub(crate) fn upgrade_runtime<T: crate::Trait>(
    ) -> ProposalParameters<T::BlockNumber, BalanceOf<T>> {
        ProposalParameters {
            voting_period: T::BlockNumber::from(50000u32),
            grace_period: T::BlockNumber::from(10000u32),
            approval_quorum_percentage: 80,
            approval_threshold_percentage: 80,
            slashing_quorum_percentage: 80,
            slashing_threshold_percentage: 80,
            required_stake: Some(<BalanceOf<T>>::from(50000u32)),
        }
    }

    // Proposal parameters for the text proposal
    pub(crate) fn text_proposal<T: crate::Trait>(
    ) -> ProposalParameters<T::BlockNumber, BalanceOf<T>> {
        ProposalParameters {
            voting_period: T::BlockNumber::from(50000u32),
            grace_period: T::BlockNumber::from(10000u32),
            approval_quorum_percentage: 40,
            approval_threshold_percentage: 51,
            slashing_quorum_percentage: 80,
            slashing_threshold_percentage: 82,
            required_stake: Some(<BalanceOf<T>>::from(500u32)),
        }
    }

    // Proposal parameters for the 'Set Election Parameters' proposal
    pub(crate) fn set_election_parameters_proposal<T: crate::Trait>(
    ) -> ProposalParameters<T::BlockNumber, BalanceOf<T>> {
        ProposalParameters {
            voting_period: T::BlockNumber::from(50000u32),
            grace_period: T::BlockNumber::from(10000u32),
            approval_quorum_percentage: 40,
            approval_threshold_percentage: 51,
            slashing_quorum_percentage: 81,
            slashing_threshold_percentage: 80,
            required_stake: Some(<BalanceOf<T>>::from(500u32)),
        }
    }

    // Proposal parameters for the 'Set council mint capacity' proposal
    pub(crate) fn set_council_mint_capacity_proposal<T: crate::Trait>(
    ) -> ProposalParameters<T::BlockNumber, BalanceOf<T>> {
        ProposalParameters {
            voting_period: T::BlockNumber::from(50000u32),
            grace_period: T::BlockNumber::from(10000u32),
            approval_quorum_percentage: 40,
            approval_threshold_percentage: 51,
            slashing_quorum_percentage: 81,
            slashing_threshold_percentage: 84,
            required_stake: Some(<BalanceOf<T>>::from(500u32)),
        }
    }
}
