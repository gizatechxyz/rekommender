---
title: Winds of Change
date: 08/26/2022
tags:
  - Tornado Cash
  - Sanctions
  - Regulation
excerpt: It’s been a stormy summer for DeFi. The sanctioning of Tornado Cash has forced us to face certain questions about Ethereum’s future. Will the censorship-resistant, permissionless dream survive, or is OFAC’s influence an existential threat?
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/08/woc-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/08/woc-header.png)

**It’s been a stormy summer for DeFi.**

After two painful market crashes, and with a gloomy macro outlook, all our hopes are pinned on the success of the recently confirmed Ethereum merge.

But the US Treasury’s [sanctioning of Tornado Cash](https://rekt.news/eye-of-the-storm/) has forced us to face certain questions about Ethereum’s future.

Is the protocol sufficiently decentralised? Can the actions of a single government affect the entire network? How much, if any, censorship is acceptable?

_And why were we not prepared for this?_

**Just as the most important update yet is about to take place on the Ethereum protocol, the path ahead suddenly seems treacherous…**

_Will the censorship-resistant, permissionless dream survive, or is OFAC’s influence an existential threat?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**Tornado Cash developer Alexey Pertsev has been jailed (without charge) for [up to three months](https://www.coindesk.com/policy/2022/08/24/alleged-tornado-developer-pertsev-must-stay-in-jail-dutch-judge-rules/) in the Netherlands.**

Though his expected legal case; that the tool was developed “[for the sole purpose of committing criminal acts](https://twitter.com/fund_defi/status/1559906169082499073)” may be difficult to prove, authorities are making an example of Pertsev, and using him to send a message to the industry.

Faced with possible jail time, certain doxxed teams behind DAOs and their legal entities have opted to comply with OFAC’s rules, at least on the front-end. However, [front-end screening](https://twitter.com/TornadoCash/status/1514904975037669386) was not enough to save Tornado Cash.

[TRM Labs](https://www.trmlabs.com/), a compliance organisation which [uses](https://twitter.com/ChainLinkGod/status/1560728626395369472) “_advanced intelligence to investigate wallets and blockchain transaction activity_”, has come under the spotlight since the sanctions, as they have presented themselves as a way for protocols to remain compliant and for developers to stay out of jail.

_But at what cost to the user?_

Blacklists from the likes of TRM [classify addresses](https://www.trmlabs.com/post/how-defi-platforms-are-using-data-from-trm-labs-to-respond-to-tornado-cash-sanctions) in terms of “ownership”, “counterparty” and “indirect risk”. However, many legit addresses have been blocked via just a couple of degrees of separation from a Tornado user.

**[Hack victims](https://depression2022.substack.com/p/8172022-my-eth-wallet-is-wrongly), [whitehats](https://twitter.com/zachxbt/status/1560718069881765888) from the [Nomad incident](https://rekt.news/nomad-rekt/), and addresses targeted by [dusting attacks](https://twitter.com/justinsuntron/status/1558397647165091840) have reported issues because of their on-chain proximity to the Tornado contracts.**

A cursory glance at a block explorer would be sufficient to confirm these users’ claims, but the “block first, ask questions later” model of compliance doesn’t do much to help those affected.

Censored front-ends don’t stop the underlying protocols from functioning, they simply make things harder for regular users, while more sophisticated blackhats will have no problem with skirting the sanctions.

Regardless of their accuracy, the wide-net approach seems to be working. Blocks including transactions to/from sanctioned addresses have [fallen significantly](https://twitter.com/bantg/status/1560994714328997888) since the ruling:

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/08/woc-blocks.png)

**But an even greater threat lies in the immediate future…**

Given that OFAC’s rules apply to US-linked persons and organisations, it appears that the [majority of validators](https://twitter.com/TheEylon/status/1558911348255461378) will be forced to comply. Not only that, but [block builders](https://www.blocknative.com/blog/ethereum-block-building) will be using [mev-boost](https://writings.flashbots.net/writings/why-run-mevboost/) post-merge, and Flashbots [has stated](https://twitter.com/bantg/status/1559948198508118016) they are, and will continue to be, OFAC-compliant.

That said, PoW is not necessarily any better. While better globally distributed, decentralisation is [lower overall](https://twitter.com/bantg/status/1561385167868010498). And mining giant [Ethermine has begun](https://twitter.com/takenstheorem/status/1560690035955011585) blanket censorship at the consensus layer.

**But if, under Proof of Stake, top validators implement such large-scale censorship, Ethereum has surely failed, in an idealistic sense.**

**“[Progress](https://twitter.com/brian_armstrong/status/1561455357448183808)”, at least in Coinbase’s eyes, also sounds eerily similar to the system that crypto set out to disrupt.**

_And with such over-zealous compliance sweeping over DeFi, are we sleepwalking into a CeDeFi future?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

>All hope is not lost.

**Tether is crypto’s largest stablecoin, and they’ve [made it clear](https://tether.to/en/tether-holds-firm-on-decision-not-to-freeze-tornado-cash-addresses-awaits-law-enforcement-instruction/) that they won’t be _preemptively_ freezing funds, which which hopefully instill confidence in smaller teams.**

While the ‘won’t comply until we’re told to’ attitude is a pretty low bar, Tether comes across as a beacon of resistance compared to the many others in the industry, who have immediately dropped their principles and bent over for OFAC.

Uniswap is currently [weighing up](https://github.com/Uniswap/interface/pull/4418) the situation in a transparent and nuanced discussion, rather than implementing blanket blacklisting.

And perhaps most surprisingly, after being [called out](https://twitter.com/LefterisJP/status/1558944794658873344), Coinbase CEO Brian Armstrong [stated](https://twitter.com/brian_armstrong/status/1560016827253551104) the company’s intent to resist censorship at the validator level. However, it would be naive to think such a large business would take a stand if that also represented an existential legal or financial threat.

Aside from Flashbots, who have [opensourced their code](https://writings.flashbots.net/writings/Flashbots-Relay-open-sourcing/) to allow for relayer diversity in preparation for the merge, other [relayers](https://twitter.com/bantg/status/1559959987278995457) are available, with Manifold making their mission [bluntly clear](https://twitter.com/foldfinance/status/1561052463305658368).

Even Vitalik has stated a preference for [punishing validators](https://twitter.com/VitalikButerin/status/1559271315080679432) who comply with sanctions, and the case for [social slashing](https://ercwl.medium.com/the-case-for-social-slashing-59277ff4d9c7) via UASF is promising.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

Overall these past weeks have brought many questions into the spotlight.

Questions which had never urgently needed an answer, instead brushed to the side during bull market mania or bear market apathy.

Now, after years of work, Ethereum is facing the beginning of a new era while undergoing a true test of its resilience.

**Established powers have timed their crackdown to weaken a movement they cannot fully control via a classic tactic: divide and conquer.**

_Have corporate interests put Ethereum in an impossible position?_

_Or are there still enough real cypherpunks to resist the attack?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/08/woc-conc.png)
