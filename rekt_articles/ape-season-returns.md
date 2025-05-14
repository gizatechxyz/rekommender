---
title: Ape Season Returns
date: 12/21/2023
tags:
  - Ape Szn
  - Ledger
  - Thirdweb
excerpt: The apes are out of their cages once more, but familiar dangers are ever-present. Each innovation spawns new ways to get rekt but, despite the risks, opportunity certainly is out there.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/apeszn-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/apeszn-header.png)

_The apes are out of their cages once more._

**Just over a year ago, the [collapse](http://rekt.news/ftx-yikes/) of FTX (_followed by SBF’s accompanying [meltdown](https://rekt.news/sbf-mask-off/)_) finally put the nail in last cycle’s coffin.**

As a result, many [predicted](https://twitter.com/Timccopeland/status/1731629327245058340), or even [declared](https://twitter.com/ercwl/status/1731495203620479015), the death of the crypto industry, which had so publicly crashed and burned after flirting with mainstream acceptance.

**While crypto’s former heroes confront the [consequences](https://rekt.news/bulls-behind-bars/) of their hubris, reduced to cutting deals in [tinned fish](https://www.businessinsider.com/ftx-ftx-trading-fish-in-prison-for-services-crypto-2023-11) rather than [unsecured credit](https://rekt.news/three-arrows/), the first signs of markets heating up has sent dormant degens into a FOMO frenzy.**

_But familiar dangers are ever-present for apes on the loose._

A seemingly unending stream of often widespread exploits, barefaced ponzi-layering, chains pushed to their limit and an out-of-control phishing landscape all lay in wait for the unsuspecting simian.

**While much has been achieved by teams working hard through the bear market, it seems each innovation has also spawned additional ways to get rekt in the cryptosphere.**

_Has nothing changed?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**Recent widespread security hazards have provided repeat opportunities to pay the [ape tax](https://rekt.news/ape-tax/).**

First, plug-and-play smart contract toolkit Thirdweb [disclosed](https://twitter.com/thirdweb/status/1731841493407576247) a vulnerability related to the use of ERC-2771 alongside multicall which [allowed](https://blog.thirdweb.com/vulnerability-report/) for “_address spoofing via malicious calldata in forwarded requests_”.

Although the specific vulnerability was not disclosed, Thirdweb listed the 27 out-of-the-box contracts affected, giving potential black hats a [convenient](https://twitter.com/pcaversaccio/status/1731933833786720641) shortlist to explore…

According to [ScamSniffer](https://twitter.com/realScamSniffer/status/1733122679803396235), over 500 mainnet and over 1000 BSC token contracts had used the dangerous Thirdweb code. Over the following days [reports](https://twitter.com/PeckShieldAlert/status/1732612048352313520) of [projects](https://twitter.com/AnciliaInc/status/1733194545268211885) being [exploited](https://twitter.com/AnciliaInc/status/1734631087433765071) trickled through.

_Picking-off stragglers is seemingly an [ongoing](https://twitter.com/PeckShieldAlert/status/1737010514801373500) sport._

OpenZeppelin followed the initial disclosure up with a detailed [post-mortem](https://blog.openzeppelin.com/arbitrary-address-spoofing-vulnerability-erc2771context-multicall-public-disclosure), explaining that “_the issue is not particular to the implementations contained in the OpenZeppelin Contracts library_”, but is a problem arising from a particular integration between the two standards.

The report includes the following example attack flow as well as instructions on how to secure affected contracts:

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/apeszn-ozcode.png)

---

**Next came what, at the time, seemed an existential threat to what was once seen as the gold standard of self-custody.**

_Reports of [strange behavior](https://twitter.com/g4sarah/status/1735264310366990672) on multiple front-ends caused rumour, suspicion and fear to spread like wildfire._

An hour later, the issue was [identified](https://twitter.com/blockaid_/status/1735275569586090221) as a wallet drainer ([Angel](https://slowmist.medium.com/cracking-the-code-unveiling-the-deceptive-angel-drainer-phishing-gang-proactive-strategies-to-3ade2bbfd45c)) [injected](https://twitter.com/MatthewLilley/status/1735275960662921638) into Ledger’s [connect-kit code](https://github.com/LedgerHQ/connect-kit/blob/main/packages/connect-kit-loader/src/index.ts#L83C49-L83C68), which is used by many project’s front-ends to connect wallets via the connect-kit loader.

The npm [package](https://cdn.jsdelivr.net/npm/@ledgerhq/connect-kit@1) (_now fixed_) used by the connect-kit had been [updated](https://twitter.com/pcaversaccio/status/1735284580310020191) to include the wallet drainer [script](https://twitter.com/bantg/status/1735279127752540465), which was now being served to users upon connecting their wallets to any of [290 instances](https://sourcegraph.com/search?q=context:global+@ledgerhq/connect-kit&patternType=standard&sm=1&groupBy=repo) of the vulnerable code across the ecosystem.

The malicious versions were [removed](https://www.npmjs.com/package/@ledgerhq/connect-kit?activeTab=versions) and the [issue was patched](https://twitter.com/Ledger/status/1735298142118072512) two hours after the first reports began to circulate, and around [four hours](https://twitter.com/FrankResearcher/status/1735286837088792794) after the drainer script was added (_though [hints](https://twitter.com/FrankResearcher/status/1735286838892404852) had been left an hour earlier still_).

**In the end, what may have been the industry’s most wide-spread and deepest-seated phishing vector [netted just $600k](https://twitter.com/zachxbt/status/1735292040986886648).**

Ledger has since [committed](https://twitter.com/Ledger/status/1737457365526470665) to reimbursing the losses, as well as deprecating Blind Signing within six months, though it remains to be seen if this is [viable](https://twitter.com/tayvano_/status/1737528358639145388)…

_Far greater damage was dealt to Ledger’s (already faltering) reputation, however._

**Unbelievably, a company who’s entire brand is built on security and safety allowed a former employee’s phished credentials to instantly start serving drainer scripts to end users all across the ecosystem.**

It must be said that, despite OpSec failures from within Ledger, DeFi front-ends might want to stay away from web2 [efficiency](https://twitter.com/moo9000/status/1735286626417336412) models when security [should](https://twitter.com/LefterisJP/status/1735389403357319253) be the number one concern.

_Even if individual projects are to take some of the blame, after multiple scandals will Ledger ever be able to [Recover](https://rekt.news/ledger-recover/) their [user’s](https://rekt.news/ledger-rekt/) trust?_

---

**Lastly, over the weekend, [came](https://twitter.com/dingalingts/status/1735994403422928972) the Ape-ocalypse:**

>If you've ever used NFT Trader in the past, revoke approval to their contract ASAP (0x13d8faF4A690f5AE52E2D2C52938d1167057B9af)
>
>So far already 37 BAYC and 13 MAYC have already been drained to this address

**The great ape escape saw assets totalling at [least $4M](https://twitter.com/WazzCrypto/status/1736000599483719891) lost to a [reentrancy attack](https://twitter.com/0xCygaar/status/1736056050816876626), which took advantage of existing approvals via a maliciously crafted self-swap.**

Things took a bizarre turn when the exploiter (_who [claimed](https://etherscan.io/tx/0xc2f91dbab46531732908a317290e18297670d0bb02bb66f94aa883ec448d9391) to be a ‘good person’, whilst [claiming and dumping](https://twitter.com/realScamSniffer/status/1736024082024730864) victims’ APE staking rewards_) decided to ransom the NFTs back to victims:

>...if you want the monkey nft back, then you need to pay me a bounty, which is what I deserve
>
>1 BAYC = 30 ETH 1 MAYC = 6 ETH you need to pay me %10 ETH for my work…
>
>…After you send me the reward, I'll return the monkey to you, with the caveat that you'll need to unauthorize the exploit contract…
>
>...Finally, I wish you all a happy day!

**Luckily, a keen eye [spotted](https://twitter.com/0xf4d3/status/1736061356959260825) a way to [thwart](https://twitter.com/punk9059/status/1736069181798539333) copycat attempts to steal the remaining NFTs, with a [rescue operation](https://twitter.com/0xQuit/status/1736268685579608142) executed, placing some of the stolen goods directly back into victim wallets.**

_Hopefully those who managed to revoke their approvals had already [refreshed](https://twitter.com/functi0nZer0/status/1735999764762607907) their browser cache._

**Shortly after the apes had been returned to their cages, however, another [attack](https://twitter.com/0xfoobar/status/1736190355257627064) saw Flooring Protocol exploited via a [vulnerability](https://twitter.com/0xQuit/status/1736214391505506575) which had been [introduced](https://twitter.com/0xfoobar/status/1736211534768943108) during a recent upgrade.**

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**Despite the [relatively](https://rekt.news/leaderboard/) modest financial damages, these three incidents have served to underline just how [delicate](https://twitter.com/WazzCrypto/status/1735303348574621752) and [interdependent](https://twitter.com/0xChew/status/1734265167423381899) the DeFi ecosystem can be.**

And for users, as FOMO takes over, hopefully they serve as timely [reminders](https://twitter.com/0xQuit/status/1736044127110975886) to stay vigilant and never keep active [approvals](https://twitter.com/santiagoroel/status/1736213178512732650) for high-value assets.

_All part of the joys of self-custody._

But while there are essentially endless ways to get rekt on-chain, or through the [ever-evolving](https://rekt.news/plenty-of-phish/) phishing [campaigns](https://rekt.news/gone-phishing/) relentlessly taking [advantage](https://twitter.com/realScamSniffer/status/1736737084889592050) of web2’s [blindspots](https://twitter.com/0xngmi/status/1736576677440868514), the supposed safety offered by centralised systems is far from certain.

**[Recent](https://rekt.news/heco-htx-rekt/)  [centralised](https://rekt.news/poloniex-rekt/)  [exchange](https://rekt.news/htx-huobi-rekt/)  [hacks](https://rekt.news/coinex-rekt/)  [aside](https://rekt.news/remitano-rekt/), CeDeFi has seemingly been up to the type of dirty tricks which would be unlikely to escape scrutiny on mainnet.**

BSC has long been the apes’ favourite enclosure in this crypto zoo, but a [closed-source hard-fork](https://twitter.com/willmorriss4/status/1734658019236131157) shows that they aren’t playing by the same rules as other chains. And, although [rugging](https://community.venus.io/t/proposal-bnb-bridge-exploiter-account-remediation/3974) the [BNB Bridge](https://rekt.news/bnb-bridge-rekt/) hacker doesn’t seem _morally_ wrong, it does set a dangerous precedent.

**In other news, OKX, whose recently-rekt DEX aggregator [lost $2.7M](https://rekt.news/okx-dex-rekt/) last week, has been alerted to a [critical bug](https://www.coindesk.com/business/2023/12/19/okx-wallet-users-warned-to-update-app-to-avoid-code-vulnerability/) in its wallet which allowed for remote code execution.**

_And rather than [concentrating](https://twitter.com/tayvano_/status/1737143247980605531) on keeping users safe, clout-chasing auditors Certik decided it would be best to [shout](https://twitter.com/CertiK/status/1737110916699959433) about their findings, drawing extra attention from would-be exploiters._

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

_Despite all the risks, opportunity certainly is out there._

**Solana SZN is bringing back bull market levels of [degeneracy](https://twitter.com/NFTwap/status/1736488065181634751), apparently having escaped its previous close-association with SBF and FTX.**

The Jito drop kicked off the mania, as users with as little as [100 points](https://www.jito.network/blog/jto-airdrop-eligibility-and-allocation-specifications/) (e.g. staking 1 SOL x 100 days) received almost 5k [JTO](https://www.coingecko.com/en/coins/jito), worth almost $10k at the time, and >$20k two days later.

Elsewhere, [earning](https://twitter.com/lemiscate/status/1731566380506345583) 4.5% plus ‘points’ for [trusting](https://twitter.com/jarrodWattsDev/status/1727584394796323042) >$800M to a (_Paradigm-backed_) [multisig](https://twitter.com/poordart/status/1728217645743513636), [re-staking](https://twitter.com/0xBalloonLover/status/1736844148844712028) and ponzi-[layering](https://twitter.com/boostonblast/status/1733222449884361202), chain-breaking [inscriptions](https://twitter.com/ercwl/status/1732182391559483603) fever…

There are plenty of places to YOLO any funds remaining after a brutal 18 months, and some of them will undoubtedly prove to have been lucrative options.

_And as retail traders [ape in](https://twitter.com/wilburforce_/status/1734039145268584732) once again, anyone with even a [modest](https://twitter.com/zachxbt/status/1736831970045223204) audience is poised to [rug](https://twitter.com/crypto_bitlord7/status/1736077084676501584) their followers, hoping to claw back some of their bear-market losses._

**In the background, the regulatory [slugfest](https://rekt.news/grudgematch-sec/) continues, but, with ETF hype building, it now feels more like [clutching](https://twitter.com/jerrybrito/status/1737119021483557347) at [straws](https://twitter.com/SamLyman33/status/1737186300246016167) than scorched earth.**

_However, the terrorist financing narrative has contributed to a renewed vigor._

Recent developments, such as Tether’s [u-turn](https://tether.to/en/tether-holds-firm-on-decision-not-to-freeze-tornado-cash-addresses-awaits-law-enforcement-instruction/) overturning their [previous decision](https://tether.to/en/tether-introduces-new-policy-to-strengthen-ecosystem-security/) not to pre-emptively freeze USDT on [OFAC-sanctioned addresses](https://rekt.news/eye-of-the-storm/), as well as working [alongside](https://www.theblock.co/post/267973/tether-weve-onboarded-fbi-secret-service-to-our-platform) the FBI show that, this cycle, US authorities will have more of a foothold before the madness really kicks off.

While this may help crypto’s image as dirty money, other, more generalised, intel-gathering operations are also in the works. 

_Binance’s recent deal will also mean [extensive monitoring](https://www.dlnews.com/articles/regulation/binance-independent-monitor-augurs-new-era-in-crypto/) from within, something that the EU sees [potentially viable](https://etendering.ted.europa.eu/cft/cft-display.html?cftId=12079) even in DeFi._

**If we hope to avoid becoming a financial surveillance tool, we will have to [start](https://twitter.com/0xTogbe/status/1737217721677328490) taking self-policing seriously.**

_But some things never change._

**As we make our Return to the Planet of the Apes, will we continue to ‘[disrupt](https://twitter.com/GwartyGwart/status/1736747337479827631)’ and ‘[innovate](https://twitter.com/0xfoobar/status/1736860200265765143)’, pushing towards the future of finance?**

_Or will we simply build another house of cards, unwittingly setting ourselves up to crash and burn once more?_

_Is there even a difference?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
