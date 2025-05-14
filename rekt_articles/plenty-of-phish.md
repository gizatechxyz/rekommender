---
title: Plenty of Phish
date: 12/06/2023
tags:
  - Phishing
  - Scams
excerpt: The tide may be turning, but crypto’s waters remain as treacherous as ever. Our latest phishing roundup covers the latest tackle and the most tempting lures. Will you take the bait?
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/plentyofphish-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/plentyofphish-header.png)

_The tide may be turning, but crypto’s waters remain as treacherous as ever._

**In the high-stakes world of crypto phishing, where each bite could be worth upwards of a million dollars, innovative strategies and updated equipment may provide the edge needed to reel in a victim.**

_Be they [bored zoomers](https://www.theblock.co/post/235022/phishing-frenzy-school-kids-are-stealing-millions-of-dollars-of-nfts-to-buy-roblox-skins) funding a Roblox habit or [nation-state level](https://rekt.news/big-phish/) threat actors, those conducting phishing campaigns show no signs of letting up._

As we wrote in our [last phishing roundup](https://rekt.news/gone-phishing/):

>crypto is providing especially bountiful waters lately

**Since then, a slew of incidents have targeted minnows and whales alike, aided by an ever-expanding range of delivery mechanisms, and novel ways of avoiding detection.**

_Until it’s too late._

Inexperienced newbies and risky FOMO-chasers are a staple for simple scams, but now veteran users and even team multisigs are falling victim.

_A good phisherman is always keen to try out the latest tackle, and get their hands on the most tempting lures…_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

_Scam-as-a-service is clearly a lucrative business._

**The infamous Inferno Drainer decided they'd made enough to [retire recently](https://twitter.com/realScamSniffer/status/1728928120370270676), after having siphoned off a total of $70M from over 100k victims.**

_But out-of-the-box wallet drainers continue to evolve._

**A new approach plays cat-and-mouse with efforts to flag suspicious addresses, relying on the CREATE2 opcode to deploy a fresh contract address for each phished signature.**

This ensures that a potential victim’s wallet UI is unable to alert them of any previously known suspicious activity on the address, as it is not associated with past scams. [ScamSniffer](https://twitter.com/realScamSniffer/status/1723627800828195030) explains:

>With create2, the Drainer can easily generate temporary new addresses for each malicious signature.
>
>After the victim signs the signature, the Drainer creates a contract at that address and transfers the user's assets.
>
>The motivation is to bypass wallet security checks.

As detailed in the [blog post](https://drops.scamsniffer.io/post/wallet-drainers-starts-using-create2-bypass-wallet-security-alert/), in the case of malicious signature phishing, the freshly-approved address is deployed and assets are transferred within the same transaction.

This vector has been used in the wild recently, including [over $900k lost](https://twitter.com/realScamSniffer/status/1723666398482944080) (via an [open-sourced](https://twitter.com/realScamSniffer/status/1723666408817709142) drainer contract), as well as during the [front-end compromise](https://twitter.com/realScamSniffer/status/1729781637062275486) of Velodrome (_round [one](https://twitter.com/VelodromeFi/status/1729771762752135463) of [two](https://twitter.com/VelodromeFi/status/1730782369185927225)_), in which [over $100k](https://t.me/investigations/73) was lost.

---

**Another technique also uses CREATE2, but to pre-generate large numbers of potential addresses for use in address-poisoning attacks.**

These addresses can then be used to contaminate transaction histories with spam transactions to addresses which appear similar to genuine past transfers, in the hopes that a copy-paste error leads to sending funds to the scammer instead.

Generating contract addresses via CREATE2 (instead of standard EOA addresses) has the advantage of not needing to top up gas for each successfully used address, nor having to store private keys for each of the millions of addresses generated.

_This kind of attack has long lain in wait, hoping for a fat finger to slip._

However, a recent spate of attacks on Safe multisig wallets has seen over [$2M lost in a week](https://twitter.com/realScamSniffer/status/1731236292166590507), to an experienced attacker who has netted over $5M in four months.

**One such victim was the Florence Finance team who [admitted](https://twitter.com/FinanceFlorence/status/1730166092947976559) to an ‘operational oversight’ resulting in a [$1.45M loss](https://twitter.com/PeckShieldAlert/status/1730045158509727968) from their multisig ([tx](https://etherscan.io/tx/0x67a7a5f258378f953e1561a25f0e264d88ac229d4c6ec871b67a2b399137fed8)).**

_It’s hard to believe that multiple signers would fall for such a well-known vector, however, this case involves an extra layer of deception._

It appears that, while Etherscan has been [hiding](https://twitter.com/etherscan/status/1645406189692526593) these spam txs since April, the ‘History’ tab of the Safe UI could be [tricked into displaying](https://drops.scamsniffer.io/post/multiple-safe-wallets-lose-2-million-to-address-poisoning-attacks/) the fake token as genuine, by using the Unicode for ‘USDC’ as the token tracker.

The UI appears to have since been [updated](https://twitter.com/realScamSniffer/status/1732067648064966916) to hide such spoofed transactions and avoid further incidents.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

_Innovation is not just for the scripts themselves, however. Enticing delivery mechanisms are the bait that keep the victims biting._

**Greed is the main motivator, especially when presented as FOMO-inducing alpha.**

A [fake staking programme](https://twitter.com/PeckShieldAlert/status/1731508733036126394) hidden within a Snapshot proposal, [hiding malicious URLs](https://slowmist.medium.com/new-scam-alert-beware-of-phishing-urls-disguised-as-transfer-addresses-95f094427364) in supposed transfer addresses, and even attempting to appear legitimate by plugging [VC backing](https://twitter.com/spreekaway/status/1729295083160195517).

_Guess it paid off for Blast’s ‘deposit now, build later’ [strategy](https://twitter.com/Blast_L2/status/1730362088583835913)…_

**Scare tactics can also prove to be powerful traps, relying on panic taking over and clouding judgment.**

Accounts impersonating web3 security researchers ([Peckshield](https://twitter.com/PeckShieldAlert/status/1726070865614491913), [ZachXBT](https://twitter.com/zachxbt/status/1724860554865500184) and [others](https://twitter.com/officer_cia/status/1724705638595371487)) managed to drain over $300k by prompting worried users to ‘revoke approvals’ via a malicious link, based on supposed incidents at Uniswap and OpenSea.

**And muddy waters make it ever more difficult to know who to trust.**

Twitter’s verification shitshow is worse than ever. [Shameless](https://twitter.com/zachxbt/status/1729376109261680842) influencers take advantage of perceived legitimacy whilst remaining [negligent](https://twitter.com/zachxbt/status/1732094407367680372) of even basic protections for their followers. Those that are eventually [banned](https://twitter.com/0xfoobar/status/1732603628911780215) had free reign for months to make their millions.

_Even [mainstream](https://twitter.com/zachxbt/status/1725625175318663672) accounts can be untrustworthy when it comes to securing their profile._

**Deeper lures may lead to a bigger catch, but more work is required, and chances of [detection](https://twitter.com/AlexMasmej/status/1731446788136292833) are higher.**

More persistent individuals are [targeting](https://slowmist.medium.com/analysis-of-north-korean-hackers-targeted-phishing-scams-on-telegram-872db3f7392b) known figures in the crypto community, posing as [investors](https://twitter.com/hosseeb/status/1732584894579318908), and attempting to deliver payloads [via Calendly links](https://twitter.com/SlowMist_Team/status/1730110239444447334) or [whilst organising](https://twitter.com/unpacker/status/1731475020189434010) an online meeting.

_Alternatively, would-be security researchers [dressing up a DDoS attack](https://twitter.com/samczsun/status/1730366496063512675) as a critical security incident, can be a way to extort projects for high fees to 'fix' the vulnerability._

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**With seemingly endless ways to get rekt, as the markets pick up we’re bound to see a corresponding uptick in victims.**

When front-end attacks seem to come almost every week (_[Frax](https://twitter.com/fraxfinance/status/1719497560543658073), [SpookySwap](https://twitter.com/SpookySwap/status/1725677274718994543) and [Trader Joe](https://twitter.com/TraderJoe_xyz/status/1725628622524883448) have all been affected recently_), standard advice such as bookmarking URLs just doesn’t cut it.

When it comes to excitable apes chasing new opportunities, maintaining an in-wallet address book and vetting each new contract address against a block explorer and project docs is perhaps a tall order.

Although some wallets contain warnings for flagged addresses, new interactions, etc., the above examples show that it’s still necessary to remain vigilant. However, [some](https://www.pocketuniverse.app/insurance) appear to be putting their money where their mouth is.

**Despite being one of the obvious problem areas to work on during the bear market, UX/UI has [remained stagnant](https://twitter.com/moo9000/status/1729436636709069146) and not exactly newbie-friendly.**

Much talk of account abstraction has led to no clear winners and, as hype takes over, incoming retail will likely stick with the same tools as last cycle.

_Until they learn the hard way._

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

Tracing those responsible occasionally does [yield results](https://tether.to/en/following-investigations-by-tether-okx-and-the-us-department-of-justice-tether-voluntarily-freezes-225m-in-stolen-usdt-linked-to-international-crime-syndicate/), but even when deposited directly to exchanges, some don’t seem to care. [Others](https://www.dlnews.com/articles/people-culture/how-crypto-firm-used-insider-trading-scam-to-steal-millions/) still appear to be in on the scam themselves.

**Even when courts are presented with an open and shut [case](https://twitter.com/Platypusdefi/status/1629448306186412033), they can’t be relied upon to punish those responsible.**

_Existing legal systems have clearly not caught up on all the innovative ways crypto allows us to steal from one another._

**But when the legacy system, which often equates this industry with criminal activity, [fails to prosecute](https://twitter.com/VCharpiat/status/1730702638902063615) a clear example, it feels somewhat [hypocritical](https://twitter.com/tayvano_/status/1730759739204829303).**

It is, however, heartening to remember that, for all the innovation on display, those responsible [aren’t always geniuses](https://twitter.com/0xngmi/status/1726712676242620500); October’s [FTM wallet](https://rekt.news/nightmare-ftm/) hacker [could have](https://fantom.foundation/blog/security-researcher-awarded-1-7-million-by-fantom-foundation/) sent the ERC-20 token to zero, worth $170M at the time.

_Almost as big an oversight as ‘reassigning’ a wallet with an admin token to the FTM contract to a team member in the first place…_

When not funding [almost half](https://cointelegraph.com/news/north-korean-hackers-stolen-crypto) of the supreme leader’s military budget, the proceeds of phishing campaigns often end up on the [roulette table](https://twitter.com/josephfcox/status/1725152894691749993), where ironically it finds it’s way back to Lazarus, [anyway](https://rekt.news/stake-rekt/).

**As the bear gives way, there’s sure to be plenty more phish in the sea.**

While stoic hopium turns into green-candle induced euphoria, and the FOMO begins to set in…

_…will you manage to avoid taking the bait?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
