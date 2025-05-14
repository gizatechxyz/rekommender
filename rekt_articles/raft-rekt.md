---
title: Raft - REKT
date: 11/12/2023
rekt:
  amount: 3300000
  audit: Trail of Bits, Hats Finance
  date: 11/10/2023
tags:
  - Raft
  - REKT
excerpt: Raft was raided by on-chain pirates for $3.3M on Friday, but the loot ended up falling overboard. When braving DeFi's stormy seas, we all must choose our vessel. Would you trust a Raft?
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/raft-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/raft-header.png)

**On Friday, [Raft](https://twitter.com/raft_fi/) fell victim to an on-chain pirate raid, causing the project’s [R stablecoin](https://www.coingecko.com/en/coins/r) to depeg.**

Although the infinite mint exploit should have netted a tidy $3.3M profit, the clumsy crypto corsair dropped the loot overboard.

**1570 ETH sent to the burn address and around $8k out of pocket [overall](https://twitter.com/0xngmi/status/1723073285263380924).**

_An embarrassing, but ultra-sound, self-rekt._

Ancilia [raised the alarm](https://twitter.com/AnciliaInc/status/1723055359034568828), with the Raft team quickly [acknowledging](https://twitter.com/raft_fi/status/1723057566664548623) the incident. An hour after the initial response, came the following [update](https://twitter.com/raft_fi/status/1723073536909099302):

>Update: Further minting of R has been paused.
>
>Existing users are still able to repay their positions and receive their collateral.

The next day, the team [informed](https://twitter.com/raft_fi/status/1723317254480425028) users that a recovery plan is in the works (_[rugging](https://twitter.com/FrankResearcher/status/1723519462995898787) a [rug-artist](https://rekt.news/bald-rekt/), perhaps?_), while reminding users who had minted R that they are still able to recover collateral.

**The team also advised against speculating on the now-partially-unbacked stablecoin, adding:**

>The current version of Raft will be sunsetted.

_Will they manage to keep themselves afloat for a v2?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

Credit: _[Ancilia](https://twitter.com/AnciliaInc/status/1723055359034568828), [Igor Igamberdiev](https://twitter.com/FrankResearcher/status/1723099971824582713), [BlockSec](https://twitter.com/BlockSecTeam/status/1723224079170478142)_

**The hack involved inflating the value of collateral by liquidating previously opened positions from an address holding excess ETH (sourced via flash loan).**

The over-valued collateral then allowed the attacker to mint 6.7M R stablecoin, which were dumped for (_what should have been_) over $3M profit.

_For a full technical breakdown of the exploit, see threads by [Ancilia](https://twitter.com/AnciliaInc/status/1723055359034568828), [Igor Igamberdiev](https://twitter.com/FrankResearcher/status/1723099971824582713) and [BlockSec](https://twitter.com/BlockSecTeam/status/1723224079170478142). See also the team's [post-mortem](https://mirror.xyz/0xa486d3a7679D56D545dd5d357469Dd5ed4259340/_Nk6_1_VvInyC0pdvHiZuAXiqm6tYSsGYGHSfOhcO1I)._

**The freshly minted R tokens were dumped into the existing liquidity pool, causing the price to tumble:**

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/raft-chart.png)

**The token’s collateral reserves were not affected, and any users who have a CDP on Raft should be able to return R and withdraw their collateral.**

Attacker address: **[0xc1f2b71a502b551a65eee9c96318afdd5fd439fa](https://etherscan.io/address/0xc1f2b71a502b551a65eee9c96318afdd5fd439fa)**

Attack tx: [0xfeedbf51…](https://etherscan.io/tx/0xfeedbf51b4e2338e38171f6e19501327294ab1907ab44cfd2d7e7336c975ace7)

Preparatory tx: [0xa1378a4d…](https://etherscan.io/tx/0xa1378a4d61e81339daaf2c7c8bb669be42002919f10379c616d0aee34047794e)

Exploited contract: [0x9ab6b21cdf116f611110b048987e58894786c244](https://etherscan.io/address/0x9ab6b21cdf116f611110b048987e58894786c244)

The profits, however, were not collected by the attacker, and were sent to the ETH [null address 0x0000](https://etherscan.io/address/0x0000000000000000000000000000000000000000) instead. Igor Igamberdiev [explains](https://twitter.com/FrankResearcher/status/1723099982633332959):

>The problem is that the code for converting R to ETH and transferring it to the exploiter was called from another contract using delegatecall
>
>But delegatecall looks at the storage of the parent contract, in which the slot with the exploit address was not initialized

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/raft-oops.png)

**_[Oops](https://twitter.com/peckshield/status/1723136855749873761/) indeed._**

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

_Despite how badly this exploit went for the attacker, they certainly [aren’t stupid](https://twitter.com/giraffe0x/status/1723157477745967614)._

**Raft had been [extensively](https://docs.raft.fi/additional-information/security) audited by four organisations, including [Trail of Bits](https://github.com/trailofbits/publications/blob/master/reviews/2023-04-tempus-raft-securityreview.pdf) and a [Hats Finance](https://hatsfinance.medium.com/raft-finance-audit-competition-final-note-16e87dce23a2) contest.**

As we’ve said before, even the best audits are [no silver bullet](https://twitter.com/trailofbits/status/1724344137065886183); incidents like these show the value of continued investment in a [full suite](https://twitter.com/thebensams/status/1723485454627143784) of security practices.

_But when vulnerabilities are still being [found in DeFi giants](https://twitter.com/aave/status/1720868368331219100), it reminds us that **nowhere is truly safe.**_

**When braving DeFi's stormy seas, we all must choose our vessel.**

_Would you trust a Raft?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
