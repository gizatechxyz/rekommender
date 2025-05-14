---
title: Multichain - R3KT
date: 07/14/2023
tags:
  - Multichain
  - Fantom
  - R3KT
excerpt: So the rumours were true. At least according to Multichain. CEO Zhaojun has been in custody in China since May 21st, and he held all the keys. Multichain’s recent troubles have turned Fantom into a ghost town…
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/multi3-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/multi3-header.png)

**So the rumours _were_ true.**

_At least [according](https://twitter.com/MultichainOrg/status/1679768407628185600) to Multichain._

**Multichain’s CEO, [Zhaojun](https://twitter.com/zhaojun_sh), has been in custody in China since May 21st.**

_...and he held all the keys._

Now, all devices with access to Multichain’s MPC wallets are allegedly in the hands of the Chinese authorities, except a single computer belonging to Zhaojun’s sister (who is now also said to be in custody).

**Looks like Multichain’s [docs](https://docs.multichain.org/getting-started/introduction) weren’t telling the truth:**

>The SMPC nodes are run by different organisations, institutions and individuals

Not to mention being a beyond-embarrassing OPSEC disaster, such an irresponsible level of centralisation has completely paralysed a major piece of DeFi infrastructure.

The Fantom ecosystem depended heavily on multiAssets (for non-native USDC, USDT, DAI, wETH and wBTC), all of which have since depegged as the situation worsened.

**Multichain’s recent troubles have turned [Fantom](https://defillama.com/chain/Fantom) into a ghost town…**

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/multi3-ftmtvl.png)

_How did we get here?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**The rumours began on May 23rd, when a [delayed upgrade](https://multichain.zendesk.com/hc/en-us/articles/7036431417103-Multichain-23-May-Operation-Update) resulted in the loss of functionality on certain bridging routes.**

Suspicions of an insider dumping tokens and an ambiguous translation of a chinese-language [tweet](https://twitter.com/yuyue_chris/status/1661326962554535936) led to panic that the entire TVL of Multichain may have been compromised.

But with no clarification from Multichain, who explained it away as “_force majeure_”, the panic quickly subsided as the bridges remained operational, with fixes being implemented on the remaining routes.

However, as described in today’s [statement](https://twitter.com/MultichainOrg/status/1679768407628185600):

>Zhaojun's family only allowed Multichain team engineers physical access to the home computer to fix technical issues with Router2 and Router5.

But access was never handed over to other team members, who were told that:

>Zhaojun would be released soon and [were] asked to continue maintaining the system and await further updates.

**But then came the [9-figure wake-up call](https://rekt.news/multichain-rekt2/).**

**And then [another](https://twitter.com/MetaSleuth/status/1678656092488994820), a few days later.**

_Something was clearly very wrong._

The two withdrawals of $126M on the 6th (~$65M since frozen) and $103M on the 10th of July were, if the team is to be believed, first a hack (“_from an IP address in Kunming_”) and then a rescue operation.

**However, with the depegging of [Fantom-based assets](https://debank.com/profile/0x1eed63efba5f81d95bfe37d82c8e736b974f477b?chain=ftm) (down 80-90%), the $103M is worth [just $69M](https://debank.com/profile/0x1eed63efba5f81d95bfe37d82c8e736b974f477b) at the time of writing.**

Another [wallet](https://debank.com/profile/0x6b6314f4f07c974600d872182dcde092c480e57b) also containing funds claimed to have been rescued by Zhaojun’s sister holds approx $75M.

**Today’s [bombshell](https://twitter.com/MultichainOrg/status/1679768407628185600) breaks the radio silence maintained by the team since July 7th, when the previous incident was left unexplained.**

_Or perhaps the whole thing is a fantasy, and Multichain is just another rug, albeit with more of a penchant for storytelling than the [industry standard](https://rekt.news/?tag=Rugpull)._

**Either way, Multichain is again urging users not to interact with the contracts, and hoping that the front end will be taken down, as they don’t have access to the domain account, either.**

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**The fallout from this catastrophe has mostly affected one chain, Fantom, and its users who had been reassured by the Fantom Foundation itself.**

**In an attempt to avoid an FTM-exodus amongst the rumours, the Foundation reassured users with the [following](https://twitter.com/fantomfdn/status/1664314480245850120) on June 1st:**

>Fantom uses the regular bridge and router 1. Multichain’s CEO does not have admin control over either.
>
>His absence has no impact on Fantom's assets and bridging. They are just as safe as they were before.

However, they must have known something was wrong, as partnerships with [LayerZero](https://twitter.com/LayerZero_Labs/status/1676989298003021824) (who are now doing [victory laps](https://twitter.com/LayerZero_Labs/status/1679885409596375040)) and [Axelar](https://twitter.com/axelarcore/status/1677006891355099138) went live on July 6th, the very same day that $126M went missing from Multichain.

_If the Fantom Foundation had enough doubts to whip up bridging partnerships with two providers, why would they risk going to bat for Multichain like that?_

_**And if they knew something was wrong, why did they blatantly throw their own users under the bus?**_

It seems unthinkable that an entire ecosystem would trust their bridging infra to a single provider, especially after being [hacked](https://rekt.news/anyswap-rekt/) two years before and no viable alternative set up.

**The real victims here are Fantom users, following a fallen idol onto a chain with no escape route, whilst being assured all would be fine…**

Geist, Fantom’s Aave fork, has already [announced](https://twitter.com/GeistFinance/status/1679827762444378114) it will be shutting down, as it uses Chainlink feeds on native feeds, which don’t reflect the depegged multiTokens.

_How many more projects will haunt the ghost chain by the time the dust settles?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**While Multichain’s off-chain security set-up is unforgivable, it’s hard to believe that the project underwent [eight audits](https://github.com/anyswap/Anyswap-Audit), none of which covered the security practices behind the contract code.**

It may not be within the scope of a typical audit but, as [Mikko Ohtamaa](https://twitter.com/moo9000/status/1679875166887727107) points out:

>If security research teams think they are gods that are "very busy" and should be fed massive amounts of dollars for the shit they produce, then they deserve to be called out as snakeoil sellers, not security sellers.

**As a community we must demand a more holistic approach from auditors, a Solidity checklist is not enough, and they know it.**

_But when test-in-prod-from-idolised-devs era still has its followers, it pays to take shortcuts._

As we have said [before](https://rekt.news/sifu-scandal/):

>**Put your faith in idols and you will get rekt.**

[Yearn](https://rekt.news/yearn-rekt/) (and [again](https://rekt.news/yearn2-rekt/)), [CREAM](https://rekt.news/cream-rekt/) (and [again](https://rekt.news/cream-rekt-2/)), [Anyswap](https://rekt.news/anyswap-rekt/) (renamed to [Multichain](https://rekt.news/multichain-rekt2/)), and many more have all made it to the [leaderboard](https://rekt.news/leaderboard/).

**And now Fantom looks to be rekt by proxy…**

_Coincidence or the Cronje Curse?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
