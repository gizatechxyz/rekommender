---
title: Bancor LP - REKT
date: 06/22/2022
tags:
  - Bancor
excerpt: The flagship feature of Bancor v3, which protects LPs from the effects of volatility, has been suspended, due to… v o l a t i l i t y. If it sounds too good to be true, it probably is.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/bancorlp-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/bancorlp-header.png)

**Bancor LPs in permanent loss?**

The flagship feature of the protocol’s v3, which protects LPs from the effects of volatility, has been suspended, _due to… v o l a t i l i t y_.

Bancor [announced](https://blog.bancor.network/market-conditions-update-june-19-2022-e5b857b39336) on Monday that its IL protection had been paused in the face of “_hostile market conditions_”.

In November last year, we reported on the losses made to IL by the majority of [Uniswap V3 LPs](https://rekt.news/uniswap-v3-lp-rekt/), something Bancor promised to be able to fix. But this market has been a stress test for even the most established of protocols, and the cracks are beginning to show.

During the pause, users are free to withdraw their funds, but will not receive the IL protection that attracted them in the first place.

_How did such an unpopular move get approved?_

It's not just prices, traders, CeFi and NFT NYC attendees, DeFi governance is DAOn bad, too.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**Bancor claimed to have [found the solution](https://docs.bancor.network/about-bancor-network/faqs/impermanent-loss-protection) to impermanent loss via single-staking pools.**

Rather than LPing a 50/50 split of two tokens, assets are paired against a common reserve of BNT. Swaps are then routed through BNT to other assets, and any impermanent loss is absorbed by BNT emissions. Any LPs whose positions have incurred IL are issued the equivalent in BNT upon withdrawal.

The IL protection is funded by fees from protocol-owned BNT liquidity on the platform as well as using a proportion of protocol fees to buyback and burn vBNT; a gradual drip-feed which, it appears, is not robust enough to withstand the recent chaos.

The mechanism is shilled as “_Deposit one token, stay exposed to one token_”, but the LPs relying on BNT to make them whole, might now be experiencing more permanence on their losses than they were previously promised.

The scheme may be good for marketing, especially for LPs who are sick of [IL outpacing their gains](https://rekt.news/uniswap-v3-lp-rekt/).

**But everything comes at a cost. In [the words](https://blog.bancor.network/market-conditions-update-june-19-2022-e5b857b39336) of Bancor themselves:**

>BNT rewards effectively have a double-cost:
>
>1. They depreciate the BNT value, resulting in impermanent loss on the network.
>2. This IL is compensated with additional BNT emissions, causing further value depreciation.

Following the recent market drops, the IL protection became unsustainable, as large quantities of rewards (also issued in BNT) were dumped onto the market, triggering the above feedback loop.

Bancor blames the incident on “_two large centralized entities_”, who had apparently been long-time farmers on the protocol, dumping rewards to cover liabilities, coupled with a substantial BNT short position which has been recently opened.

_No prizes for guessing which entities they are referring to…_

Now, users have had their positions held hostage, facing the tough decision to get out now, and lose what they’re owed, or cross their fingers and wait.

**The announcement reassures users that:**

“On-chain data shows some of the worst BNT rewards dumping is behind us. The temporary measure to pause IL protection should give the protocol some room to breathe and help BNT recover.”

However, while the [price of BNT](https://www.coingecko.com/en/coins/bancor-network) bottomed out on June 19 when IL protection was paused, any recovery has so far been miniscule, and non-existent when denominated in ETH.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/bancorlp-chart.png)

Even if Bancor manages to avoid a [bank](https://twitter.com/XBDeFi/status/1539156202063552512)  [run](https://twitter.com/fullyallocated/status/1538958931435094016), as soon as the IL scheme is reactivated, many users will surely withdraw and race to dump the BNT, recreating the same sell/emission loop.

_The protocol may now be facing a promise it can’t fulfil._

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**Whether the model will prove sustainable or not remains to be seen, but it’s surprising that a DeFi OG such as Bancor chose to make such a decision without securing the DAO’s approval first.**

Recent governance blunders such as Solend’s [attempt to take control](https://protos.com/solend-dao-struggles-another-day-another-dao/) of a user’s position which threatened the health of the protocol, or Tribe DAO’s [U-turn](https://twitter.com/cobie/status/1537220039522545664) on whether to refund victims of April’s [$80M exploit](https://rekt.news/fei-rari-rekt/) may be expected from newer protocols.

**But in this case, the permission to suspend IL protection was granted within the fine-print of an earlier, security-focused proposal.**

Bancor’s multisig signers are able to enact certain changes with “_retroactive community approval_” according to a proposal [passed](https://vote.bancor.network/#/proposal/0x69e6083f6fb711185f51b7e0cdcafd29e96a40fabe62c927681701c09bcabd0d) in April. Point 7. of [BIP21](https://gov.bancor.network/t/bip21-dao-multisig-intervention-policy/3504) states:

>The DAO isn’t allowed to intervene with the withdrawal of funds, save for the **adjustments to the protection mechanism**.

However, the proposal’s focus is to protect the protocol from “_hacks and exploits, and coordinated exits (“rug-pulls”) affecting a token that has passed the vetting process_” allowing the DAO to intervene “_in the operation of affected pools, immediately after a token has become compromised_”.

Of course users passed BIP21, nobody would want delays in crisis response if a whitelisted token were to compromise the protocol.

**But the IL protection pause does not concern the exploit or rugpull of a whitelisted token.**

Bancor is using powers granted for security reasons to attempt to prop up its struggling IL protection mechanism, hurting their own users when they need it the most.

_If it sounds too good to be true, it probably is._

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)

>If you enjoy our work, please consider donating to our [Gitcoin Grant](https://gitcoin.co/grants/1632/rekt-the-dark-web-of-defi-journalism).

