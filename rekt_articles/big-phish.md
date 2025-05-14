---
title: Big Phish
date: 04/19/2022
tags:
  - North Korea
  - Ronin
excerpt: The DeFi class of 2022 has introduced some unexpected players. The FBI have announced that a collective of North Korean state-sponsored hackers (The Lazarus Group) are responsible for multiple DeFi hacks, including the largest - the Ronin Network.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/04/bigphish-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/04/bigphish-header.png)
**The DeFi class of 2022 has introduced some unexpected players.**

Last month’s attack on the [Ronin Network](https://rekt.news/ronin-rekt/) was the largest crypto hack _ever_, but despite the amount, the story didn’t generate anywhere near as much interest as the former #1, [Poly Network.](https://rekt.news/polynetwork-rekt/)

However, the Ronin case came back into the spotlight after the [FBI announced](https://twitter.com/Ronin_Network/status/1514650523873468423) that they believe _The Lazarus Group_, a collective of North Korean state-sponsored hackers, are responsible for the theft.

Yesterday, the Cybersecurity and Infrastructure Security Agency (CISA) published [a report](https://www.cisa.gov/uscert/ncas/alerts/aa22-108a) with examples of the methods used by the group, also known as Advanced Persistent Threat (APT) 36.

In the report, they state:

>”North Korean state-sponsored cyber actors use a full array of tactics and techniques to exploit computer networks of interest, acquire sensitive cryptocurrency-intellectual property, and gain financial assets.”

We wonder, how many of [our stories](https://rekt.news/leaderboard/) can be linked to North Korea?

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

The Lazarus group, and specifically the financially-focussed BlueNoroff sub-group, has a long history, [including](https://www.justice.gov/opa/pr/north-korean-regime-backed-programmer-charged-conspiracy-conduct-multiple-cyber-attacks-and) ties to the Sony hack (2014), the Bangladesh Bank robbery (2016), in which >$100M was transferred out of the Federal Reserve Bank of New York via the SWIFT network, and the WannaCry ransomware attack (2017).

**In the cryptosphere, the group’s attacks have tended to focus on CEXs, including Kucoin, Liquid.com, NiceHash and Bithumb, using social engineering to obtain access to private keys.**

**However, more recently, a move towards DeFi has been noted.**

Phishing attacks are common in DeFi, especially among the NFT community, with tales of rookie (and veteran) users alike constantly being targeted via Discord and Telegram.

[Back in 2020](https://rekt.news/nxm-hugh-speaks-out/) we spoke to Hugh Karp of Nexus Mutual about his experience losing $8M, and last month, DeFiance Capital’s [Arthur Cheong lost ~$1.7M](https://www.theblockcrypto.com/post/138724/hacker-steals-1-7-million-in-nfts-from-defiance-capital-founder-cheong).

**In Ronin’s case, though, the entire network was compromised, in no small part due to a worryingly small validator set.**

The >$600M were taken from the network’s bridge after attackers gained access to 5 of just 9 validator signatures securing the network.

**With four of the validators all coming from the same organisation (Sky Mavis), the set-up must have already looked promising for a potential phishing campaign.**

But given that the fifth signature needed for a majority was accessible via an old, but still active, arrangement between Sky Mavis and Axie DAO to ease network traffic, it was only a matter of time until Lazarus found their way in.

**The level of sophistication demonstrated by the Lazarus Group’s social engineering campaigns is remarkable. As [this blog entry](https://securelist.com/the-bluenoroff-cryptocurrency-hunt-is-still-on/105488/) from cybersecurity firm Kaspersky states:**

>“If there’s one thing BlueNoroff has been very good at, it’s the abuse of trust.”

The article goes on to detail an attack vector via a malicious document shared via Google Drive. Other novel vectors include [browser-in-the-browser (BitB) attacks](https://threatpost.com/browser-in-the-browser-attack-makes-phishing-nearly-invisible/179014/), [Google Docs comments](https://www.avanan.com/blog/google-docs-comment-exploit-allows-for-distribution-of-phishing-and-malware), and even developing a [malicious wallet application.](https://securelist.com/lazarus-trojanized-defi-app/106195/)

A longer list of examples, with screenshots, can be found in [this thread by Taylor Monahan.](https://twitter.com/tayvano_/status/1516225457640787969)

**While the specifics may vary, the key to the successful delivery of a payload is minimising scrutiny, achieved by knowing intricate details of the target organisation.**

>“The goal of the infiltration team is to build a map of interactions between individuals and understand possible topics of interest. This lets them mount high-quality social engineering attacks that look like totally normal interactions. A document sent from one colleague to another on a topic, which is currently being discussed, is unlikely to trigger any suspicion. BlueNoroff compromises companies through precise identification of the necessary people and the topics they are discussing at a given time.”

Yesterday’s [CISA report](https://www.cisa.gov/uscert/ncas/alerts/aa22-108a) provides:

> _information on tactics, techniques, and procedures (TTPs) and indicators of compromise (IOCs) to stakeholders in the blockchain technology and cryptocurrency industry to help them identify and mitigate cyber threats against cryptocurrency._”

It describes a series of malicious applications that are delivered via the spearphishing to employees disguised as “_trading or price prediction tools_”.

>Observed payloads include updated macOS and Windows variants of Manuscrypt, a custom remote access trojan (RAT), that collects system information and has the ability to execute arbitrary commands and download additional payloads (see [North Korean Remote Access Tool: COPPERHEDGE](https://www.cisa.gov/uscert/ncas/analysis-reports/ar20-133a)). Post-compromise activity is tailored specifically to the victim’s environment and at times has been completed within a week of the initial intrusion.

The report goes on to list some Indicators of Compromise and recommendations for mitigation, which include:

> _A cybersecurity aware workforce is one of the best defenses against social engineering techniques like phishing._

Both [Kaspersky](https://www.kaspersky.com/about/press-releases/2022_snatch-that-crypto-bluenoroff-threat-actor-drains-cryptocurrency-startups-accounts) and [Cheong](https://twitter.com/arthur_0x/status/1514890456596840449) have also warned teams in the industry to be especially vigilant, publishing some basic recommendations that organisations can take to protect themselves, such as using multisig wallets, hardware 2FA, and dedicated machines solely for crypto transactions.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png) 

**As the crypto crimewave shows no sign of slowing, on-chain investigation companies such as Chainalysis are in high demand.**

We asked Erin Plante, Senior Director of Investigations at Chainalysis, for their take on state-sponsored hacking in DeFi:

>"The attribution of Lazarus to the Ronin attack underlines two trends in hacking. The first is the productivity of DPRK-affiliated threat actors and their exploitation of cryptocurrency. This is a national security concern considering they have stolen billions of dollars' worth of crypto and the UN has connected this activity to funding their nuclear program. The second trend is the need for better security for DeFi protocols. Almost 97% of all cryptocurrency stolen in the first three months of 2022 has been taken from DeFi protocols, up from 72% in 2021 and just 30% in 2020."

**[This detailed report](https://blog.chainalysis.com/reports/north-korean-hackers-have-prolific-year-as-their-total-unlaundered-cryptocurrency-holdings-reach-all-time-high/?utm_campaign=twitter&utm_source=OrganicSocial&utm_content=Thread), published by Chainalysis in January, sums up the various crypto heists attributed to North Korean hackers, starting all the way back in 2017.**

_With the ~$600M loot from the Ronin incident, 2022 has already been more profitable for DPRK than last year, and we’re not even halfway through._

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/04/bigphish-hacktvl.png)

**But how will they launder the loot?**

In addition to the unexpected use of CEXs, a route usually avoided due to KYC requirements, the report details the group’s increasing use of mixing services; they appear to be operating more cautiously with time, as crypto steadily moves into the mainstream.

**The typical laundering process is as follows:**

- ERC-20 tokens and altcoins are swapped for Ether via DEX
- 
- Ether is mixed
- 
- Mixed Ether is swapped for Bitcoin via DEX
- 
- Bitcoin is mixed
-  
- Mixed Bitcoin is consolidated into new wallets
-  
- Bitcoin is sent to deposit addresses at crypto-to-fiat exchanges based in Asia —potential cash-out points

**Chainanalysis also suggests that Lazarus may be sitting on funds, waiting for law enforcement attention to die down among [an ever-growing exploit list.](https://rekt.news/leaderboard/)**

Either way, it takes time:

> ”Of DPRK’s total holdings, roughly $35 million came from attacks in 2020 and 2021. By contrast, more than $55 million came from attacks carried out in 2016—meaning that DPRK has massive unlaundered balances as much as six years old.”

**For such a large amount, though, it seems that the hackers are not willing to wait.**

The Ronin funds are well on their way to being laundered, with [Elliptic estimating](https://www.elliptic.co/blog/540-million-stolen-from-the-ronin-defi-bridge) that some $107M are already in the process.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/04/bigphish-lazalaunder.png)

**While the involvement of CEXs is somewhat non-standard for DeFi laundering, the clear preference for Tornado Cash comes as no surprise.**

With all this law enforcement attention, and the key part that mixers play in washing stolen funds, Tornado Cash was bound to come under the spotlight sooner or later.

Thursday’s news also prompted [a statement](https://twitter.com/TornadoCash/status/1514904975037669386) from the privacy tool _(and [hacker favourite](https://twitter.com/ASvanevik/status/1513425755249721344))_ Tornado Cash, that an [on-chain list of sanctioned addresses](https://etherscan.io/address/0x40c57923924b5c5c5455c48d93317139addac8fb) _(maintained by Chainalysis and including the [Ronin exploiter](https://etherscan.io/address/0x098b716b8aaf21512996dc57eb0615e2383e2f96))_ would be unable to access the dapp.

However, given that the underlying smart contracts are immutable, not having access to a UI is unlikely to deter anyone capable of pulling off a DeFi heist, state-sponsored or otherwise.

Is this just a way to feign compliance and keep regulators at bay? Surely it can’t be that easy..

_As their desperation increases, how far will our governments go to stamp out this alternate economy?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
