---
title: Compound Errors
date: 08/31/2022
tags:
  - Compound
  - Governance 
excerpt: Compound’s governance has backfired, and not for the first time. An oracle update has bricked the cEther market for a week. Compound say that “Funds are not immediately at risk, but this is a developing situation…”
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/08/comperror-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/08/comperror-header.png)

**Compound’s on-chain governance has backfired, and [not for the first time](https://rekt.news/overcompensated/).**

An upgrade to the protocol’s oracle contract was implemented yesterday, resulting in unintended consequences. Despite three audits, the new code contained a bug causing transactions to revert for ETH borrowers and suppliers.

Compound [announced](https://twitter.com/compoundfinance/status/1564695152626655234):

**“_Effectively, the cETH market is temporarily frozen._”**

The thread goes on to state that “_Funds are not immediately at risk, but this is a developing situation._”

_Let’s hope, this time, things don’t go from bad to worse…_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

Yesterday, Compound implemented [Proposal 117](https://compound.finance/governance/proposals/117) in order to _“upgrade the protocol's oracle contract (currently v2) to v3, which switches the anchor market from Uniswap v2 to v3.”_

The bug appears to be a result of differences between Compound’s interest-bearing cTokens cErc20 and cEther.

As stated in Compound’s [docs](https://docs.compound.finance/v2/ctokens/): “_CErc20 wraps an underlying ERC-20 asset, while CEther simply wraps Ether itself. As such, the core functions which involve transferring an asset into the protocol have slightly different interfaces depending on the type_”.

**However, it appears that these considerations weren’t taken into account when designing the new oracle.**

OpenZeppelin [explained the error](https://www.comp.xyz/t/proposal-to-upgrade-to-uav-v3/3270/16) as follows:

>cETH does not have an underlying() method assumed to be present in every cToken contract by the new oracle implementation, the getUnderlyingPrice function 12 returns empty bytes that cannot be decoded and the call reverts.

Fortunately, although the cEther market is frozen, users are still able to deposit collateral and avoid potential liquidations. And the news has not significantly affected the [price of COMP](https://www.coingecko.com/en/coins/compound).

The [contract](https://etherscan.io/address/0xad47d5a59b6d1ca4dc3ebd53693fda7d7449f165) in question was audited by three firms, Dedaub, ABDK ([both linked here](https://drive.google.com/drive/folders/1IWj2BtpG9qsb7PBVaWyQSaQyISr9m_E7)) and [OpenZeppelin](https://drive.google.com/file/d/1usfoYjWtlvrf8Ca6akFDvITwHC-wk73Y/view?ts=6303d733), with the most recent of the reports dated 1st April 2022. However, the [latest commit](https://github.com/smartcontractkit/open-oracle/commit/43ced36cdc711a46880a6221c94333eaa363490d) to UniswapAnchoredView was made 26 days later.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

While the bug has been quickly identified and the fix is simply to revert the oracle to the [previous version](https://etherscan.io/address/0x65c816077c29b557bee980ae3cc2dce80204a0c5), the changes cannot be implemented for 7 days, until [Proposal 119](https://compound.finance/governance/proposals/119) passes.

**The protocol’s rigid on-chain governance also caused issues last year, costing Compound dearly.**

Initially, [$80M in excess rewards](https://rekt.news/overcompensated/) were distributed to depositors before [a further ~$68M were released](https://rekt.news/compound-rekt/) while waiting for the fix to be implemented.

Unfortunately, expensive mistakes are common in our industry.

Just yesterday OptiFi Labs [admitted to accidentally shutting down](https://twitter.com/OptifiLabs/status/1564367455220219904) their contract on Solana mainnet, bricking 661K USDC (mostly belonging to a team member). We also recently learned that Crypto.com [accidentally sent a user $10M](https://blockworks.co/crypto-com-sues-user-after-refunding-10m-instead-of-100/), instead of $100, not noticing for seven months.

With examples of carelessness such as these, even at organisations working under the highest scrutiny, fully on-chain governance may prove more of a hindrance.

While decentralisation maxis often claim that a multisig is not enough, perhaps granting temporary powers to an emergency multisig could provide a lifeline in similar situations…

_Is this the price to pay for on-chain governance?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
