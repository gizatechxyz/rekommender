---
title: Cat and Mouse
date: 09/08/2023
tags:
  - Privacy
  - Regulation
  - Tornado Cash
excerpt: Privacy is dying, but all hope is not lost. The tug of war between crypto-enabled privacy and regulatory oversight has been on-going for years. Are Privacy Pools the answer?
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/cat-and-mouse-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/cat-and-mouse-header.png)

**Privacy is dying.**

_But all hope is not lost._

As cold hard cash gets rapidly replaced by traceable transactions, a catalogued web containing all financial interactions of every citizen is constantly being built.

_A surveillance state’s wet dream._

Cryptocurrency’s countercultural origins may have been borne of a variety of inspirations, but the preservation of privacy in an ever more connected world has been a leading use-case for everyone, from ordinary users, hardcore cypherpunks, political activists, and even [state-sponsored hacking groups](https://rekt.news/big-phish/).

**The tug of war between crypto-enabled privacy and regulatory oversight has been on-going for years.**

_But the issue was kicked into overdrive with last year’s [sanctioning of Tornado Cash](https://rekt.news/eye-of-the-storm/) by the US Treasury._

Since the initial sanctions, things have gone from bad to worse for blockchain privacy.

Developers have been arrested (first [Alexey Pertsev](https://rekt.news/war-on-code/), then [Roman Storm](https://www.justice.gov/usao-sdny/pr/tornado-cash-founders-charged-money-laundering-and-sanctions-violations) last month), and [address screening](https://rekt.news/winds-of-change/) was implemented across many front-ends (_including for griefed addresses [dusted](https://twitter.com/justinsuntron/status/1558397647165091840) with Tornado ETH_).

**Even Tornado’s protocol governance came [under attack](https://rekt.news/tornado-gov-rekt/) via a hidden [‘metamorphic contract’](https://0age.medium.com/the-promise-and-the-peril-of-metamorphic-contracts-9eb8b8413c5e) snuck into a proposal in May.**

Other blockchain-based privacy tools exist, but none has had the impact of Tornado Cash. Even post-sanctions, there has been no clear successor, likely due to the sheer simplicity and familiarity of ETH’s OG mixer.

Now, a group of devs, researchers and privacy advocates have proposed a tweaked version of the model; a compromise protecting the privacy of legitimate users, while ensuring bad actors are kept from contaminating the tool’s reputation.

**But the question still remains…**

_[Who](https://twitter.com/ameensol/status/1632089813020024833) decides the ‘good’ actors from the ‘bad’?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**The new Privacy Pools0, [announced](https://twitter.com/ameensol/status/1699424914229321966) Wednesday, allow users to “_provably dissociate from illicit funds_” via zk-proofs.**

The [docs](https://github.com/ameensol/privacy-pools) state that:

>This design aims to be a crypto-native solution that allows the community to defend against hackers abusing the anonymity sets of honest users without requiring blanket regulation or sacrificing on crypto ideals.

Inspired by an idea Vitalik [described](https://www.youtube.com/clip/Ugkx7LeQPvONM0OFOfAUazyjf0JSj_9y7Tqw) in the wake of the Tornado Cash sanctions, the project has been coded by RAI’s [Ameen Soleimani](https://twitter.com/ameensol/), and [presented](https://twitter.com/ameensol/status/1699425534134927625) in collaboration with [two](https://twitter.com/fschaer)  [researchers](https://twitter.com/mat_nadler) at Basel University.

Privacy Pools were [teased](https://twitter.com/ameensol/status/1632083054272430080) by Soleimani back in March, but the [paper](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4563364) goes into detail on the mechanisms, as well as exploring potential applications which could ensure compliance while preserving privacy.

**When withdrawing funds, users will be able to create an accompanying zk-proof that the funds are sourced from a sub-set of pool deposits (i.e. excluding any flagged addresses).**

These proofs can then be referenced by existing transaction screening tools, which currently tend to flag mixers as blanket ‘tainted’ funding sources, with knock-on effects for legitimate users.

**The protocol would allow for custom association/exclusion sets, effectively allowing US users to prove compliance with US regs, EU with EU regs, any user to comply with both, etc.**

Other examples given in the paper include a network of banks maintaining an association set which proves KYC/AML compliance, or being able to prove a link between sources of funds (i.e. an association set of 1:1), without linking between individual accounts.

**Not only does the protocol allow users to prove that their funds have been sourced from clean deposits but, with time, legitimate use will decrease the appeal for bad actors:**

>Over time as communities curate deposit lists, the anonymity set for hackers actually shrinks to include only those bad deposits, naturally hindering even the possibility of money laundering to occur.

_Win-win?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**The only trouble is… who decides who’s compliant and who’s not?**

>If there is a perfect consensus on which funds are “good” and which are “bad”, the system will lead to a simple separating equilibrium

_That’s a big ‘if’._

**While the use of custom association sets ensures that the tool itself is credibly neutral, users choosing is not a workable/scalable solution.**

_Ultimately, users will have to rely on lists being maintained by third parties, and given a co-author is Chainalysis’ Chief Scientist, no prizes for guessing who that might be._

The authors acknowledge the difficulty that integrating mixed funds into a regulated poses, and present their tool as a flexible solution that could be used in a variety of ways.

_What that looks like in practice, however, is still up for debate._

On-chain proofs seem the most versatile, though would increase transaction fees, and a low barrier to entry is key for maintaining a sufficiently large anonymity set.

>Having the proofs readily available on-chain, introduces additional transaction costs, but reduces the coordination effort, levels the playing field and mitigates the risk that screening tool providers could have a quasi-monopoly due to their knowledge of non-public proofs.

**Some may argue that the likes of TRM and Chainalysis retaining the power to choose, whilst being instructed by state authorities, is against crypto ethos.**

_In the wake of the OFAC sanctions, the rush to ‘block first, ask questions later’ seems a worrying precedent for compliance organisations to perpetuate for the future._

But, while hardcore cryptolibertarians might complain of gatekeeping, the model seems a good and workable compromise for the vast majority of users who just want to maintain privacy on the network without being branded a hacker or terrorist.

Essentially, the project boils down to the following quote, one that seems a good summary of the [threat crypto is currently facing](https://rekt.news/grudgematch-sec/):

>In many cases, privacy and regulatory compliance are perceived as incompatible. This paper suggests that this does not necessarily have to be the case

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**As middlemen become [obsolete](https://twitter.com/NeerajKA/status/1346836020927619077), states will blindly clutch at any perceived negatives of new tech, in an attempt to retain power over a system that has already outgrown them.**

_Innovations like Privacy Pools will give them less ammo to come after a technology they [do not understand](https://twitter.com/lex_node/status/1699966400083775988)._

>this is barbarbism and technological ignorance, pure and simple

**But, as we saw just yesterday, even attempting to comply gets you [nowhere](https://www.cftc.gov/PressRoom/PressReleases/8774-23).**

We will continue to build the tools, let them decide if they want to participate.

_[Why](https://twitter.com/spreekaway/status/1699919086694597079) even play by their rules anymore?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
