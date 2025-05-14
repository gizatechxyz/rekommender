---
title: Gone Phishing
date: 09/22/2023
tags:
  - Phishing
  - Scams
excerpt: Phishing is a year-round sport. But crypto is providing especially bountiful waters lately. You’d have thought those of us still around would know better by now… What lurks in the murky depths?
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/gone-phishing-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/gone-phishing-header.png)

**Phishing is a year-round sport.**

_But crypto is providing especially bountiful waters lately._

From [compromised](https://twitter.com/Balancer/status/1704281611326357567) front-ends to ‘pig butchering’ [scams](https://twitter.com/realScamSniffer/status/1704627576973795642), an eight-figure on-chain [blunder](https://twitter.com/realScamSniffer/status/1699605356740305198) to hacked celebrities (both [crypto](https://twitter.com/officer_cia/status/1700663821558325464) and [non-crypto](https://twitter.com/WazzCrypto/status/1702820716343624168)), and even government agencies getting [duped](https://www.forbes.com/sites/thomasbrewster/2023/08/24/dea-accidentally-sends-50000-in-drug-proceeds-to-crypto-scammer/)... the last few weeks have been filled with news of scammers hitting the jackpot via a number of common vectors.

Phishing for allowances, social engineering, sim-swapping, wallet drainers, disguised malware, and address poisoning are all strategies which have been running roughshod across the cryptosphere lately.

**With all the normies having presumably abandoned crypto for the time being, many still fall into the traps laid by an endless succession of serial scammers.**

_You’d have thought those of us still around would know better by now…_

But when it comes to the [experts](https://rekt.news/big-phish/), it seems nobody is safe.

_What lurks in the murky depths?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**Going for big game**

Phishing attacks can be executed using a variety of social engineering techniques, some more targeted than others.

The Lazarus Group’s [never-ending](https://twitter.com/tayvano_/status/1703603575954583985) crypto crime spree (most recently involving attacks on [Atomic Wallet](https://rekt.news/atomic-wallet-rekt/), [AlphaPo](https://rekt.news/alphapo-rekt/), [Stake](https://rekt.news/stake-rekt/) and [CoinEx](https://rekt.news/coinex-rekt/), totalling over $250M) generally focuses on employees of custodial projects, where access to just a few private keys could reel in [hundreds of millions](https://rekt.news/ronin-rekt/).

These ‘spearphishing’ methods are often highly-sophisticated schemes lasting for extended periods, and some examples of attack vectors are discussed in our [article from last year](https://rekt.news/big-phish/).

**Harpooning the whales**

Just [yesterday](https://twitter.com/realScamSniffer/status/1704627576973795642), a user [lost 4.5M USDT](https://etherscan.io/tx/0x5e422a69170faf409ee0b976ee2773b6fc68938df1a3af0ee23dc66b914b865f) through what appears to be a [‘fake mining’ scam](https://support.metamask.io/hc/en-us/articles/11324125739163-Fake-mining-voucher-scams), which have so far netted over $300M according to Tayvano’s [Dune dashboard](https://dune.com/tayvano/sha-zhu-pan).

This type of scam, known as ‘pig-butchering’, involves continued contact with the victim, often over long periods. Leverage may be gained via blackmail, romance, or simply building trust, in order to convince the mark to transfer funds, often dressed up as a lucrative investment opportunity.

_But just as some scams take time, others are over in the blink of an eye…_

Earlier this month, an [experienced](https://twitter.com/WuBlockchain/status/1699653066696589506) DeFi [user](https://etherscan.io/address/0x13e382dfe53207e9ce2eeeab330f69da2794179e) lost $24M just minutes after [signing](https://etherscan.io/tx/0xbb4fe89c03d8321c5bfed612fb76f0756ac7e99c1efaf7c4d99d99f850d4de53) increaseAllowance messages which allowed the hacker to transfer the user’s stETH (worth [$15.6M](https://etherscan.io/tx/0xcbe7b32e62c7d931a28f747bba3a0afa7da95169fcf380ac2f7d54f3a2f77913)) and rETH (worth [$8.6M](https://etherscan.io/tx/0xb91d7b1440745aa07409be36666bc291ecc661e424b21b855698d488949b920f)) directly to their own wallet.

The function, since [deprecated](https://github.com/OpenZeppelin/openzeppelin-contracts/issues/4583), was [criticised](https://twitter.com/bantg/status/1699765906887327874) for its limited (_legitimate_) usefulness whilst simultaneously presenting risks in its (_illegitimate_) usefulness for phishing scammers.

It’s unknown how the payload was delivered to the above user, but distributing links via hacked, or falsified, Twitter accounts has been particularly successful under Elon Musk’s stewardship of the platform…

**Casting a wide net**

_While extra effort may be worth it for high-net-worth individuals, large scale trawling still picks up plenty of small fry._

**Less than two weeks ago, Vitalik Buterin’s Twitter was [hacked](https://twitter.com/officer_cia/status/1700663821558325464) to promote a link to a fake ‘commemorative NFT’ for an upcoming Ethereum upgrade.**

In Buterin’s case, access was gained via a SIM-swap of his phone number, which Twitter [requires](https://twitter.com/TimBeiko/status/1700667485165605202) for verification when signing up for Verified status, and allows an attacker to bypass 2FA, reset passwords, and take control of an account.

The ploy took in [almost $700k](https://twitter.com/zachxbt/status/1700650151298089374) for the [hacker](https://etherscan.io/address/0x4eF6f0d3f94fF609ACef88068b1FC66a1184b3f3), plus a 33% cut for the developers of the drainer service used (in this case [Pink Drainer](https://twitter.com/BoringSecDAO/status/1700724667391307863)).

_Last month, [ZachXBT’s tally](https://twitter.com/zachxbt/status/1694326221511794706) stood at over $13M lost to recent SIM swap attacks on high-profile crypto accounts. Since then, similar attacks have also hit [Gitcoin](https://twitter.com/yyctrader1/status/1697289894027210842), [Ordinals Wallet](https://twitter.com/MistTrack_io/status/1699611085572284829) and [Polymarket](https://twitter.com/IsraelPMbah/status/1705146964063519223)._

**Wallet drainers are a Scam-as-a-Service, [malware](https://twitter.com/Alchemyst0x/status/1625575810823294976) which can be used by those who carry out social engineering campaigns, without needing technical knowledge. Profits are then [shared](https://blocksecteam.medium.com/demystifying-profit-sharing-in-inferno-drainer-2e8a9afb974b) between scammers who broadcast the links via hacked Twitter or Discord accounts, and the drainer developers.**

The tools, first developed for relieving NFT holders of their collections, have expanded to examine a potential victims wallet before [cherry picking](https://twitter.com/BoringSecDAO/status/1700724669719191941) assets:

>Pink's drainer script is sophisticated, and it will target you with any number of attacks depending on which would be the most profitable.

[Pink Drainer](https://dune.com/scamsniffer/pinkdrainer-stats) (with almost $9M stolen so far) is the latest in a long line of ever-evolving, out-of-the-box malware used by crypto scammers. Previous iterations include the original [Monkey Drainer](https://aml.slowmist.com/events/monkey_drainer_statistics/) ($16.5, now [retired](https://t.me/MonkeyDrainer/317)), [Venom](https://dune.com/scamsniffer/venom-drainer-stats) ($27M, which was recommended by its predecessor) and [Inferno](https://dune.com/scamsniffer/inferno-drainer) ($42M), among others.

**Initilally focused on less tech-savvy, FOMO-driven NFT bros, drainers have [adapted](https://twitter.com/MetaSleuth/status/1689839294272397312) to target ETH and ERC20 tokens as JPEGs have fallen out of favour.**

_It looks to have been a shrewd move, too. Given the SEC’s latest [move](https://www.sec.gov/news/press-release/2023-178), PFP collections might be [on the way out](https://twitter.com/Dogetoshi/status/1701967471257792555)…_

**Shooting phish in a barrel**

_Crypto Twitter is not the only place to cast a lure, going straight to the source can also be a profitable strategy…_

**On Wednesday, Balancer published a [warning](https://twitter.com/Balancer/status/1704281611326357567) to users (_the [second](https://rekt.news/balancer-rekt/) time in a month_) that the protocol’s front-end had been compromised, leading to at [least $238k](https://twitter.com/zachxbt/status/1704286832844828914) lost.**

These attacks, which rely on users’ trust in the transactions served by a project’s official website, also affected [Curve](https://rekt.news/curve-finance-rekt/) last year and [Badger DAO](https://rekt.news/badger-rekt/) (whose users, [including Celsius](https://rekt.news/celsius-rekt/), lost a total of $120M) in 2021.

Hackers use social engineering strategies on domain registrars in order to take control of the protocol’s official UI, inserting code which presents their own (malicious) contracts for users’ approval.

Approvals may be harvested over extended periods (nearly two weeks in Badger’s case) and allow hackers drain funds directly from users’ wallets.

**Specialised tackle**

Rather than serving drainers to users directly, be it via Twitter or a project’s front-end, disguising malware as genuine crypto apps can lead to a good catch.

A Metamask clone in an app store might be enough to fool non-crypto-natives, which is what Mark Cuban [claims](https://www.dlnews.com/articles/people-culture/mark-cuban-loses-870k-to-a-crypto-scam/) happened to him [last weekend](https://twitter.com/WazzCrypto/status/1702820716343624168).

The loss of [almost $900k](https://twitter.com/BeosinAlert/status/1702904027493929151) was not Cuban’s first rodeo in DeFi, however (he [got rekt](https://rekt.news/iron-finance-rekt/) LPing Iron Finance’s ill-fated stablecoin in 2021), and [further losses](https://protos.com/mark-cuban-narrowly-avoids-2-5m-loss-in-900k-crypto-hack/) on Polygon were avoided.

_There’s no honour amongst degens, either._

[SlowMist warns](https://twitter.com/SlowMist_Team/status/1705151983542559125) of malicious open-source MEV bot code which funnels funds directly to the [hacker’s address](https://etherscan.io/address/0xfBcf33613A2609C050525395ec6885F6538fEC60) when used by wannabe MEVoors. The ploy already looks to have accrued almost 50k in just a couple of days.

**Convincing lures**

_Finally, an on-chain dragnet approach known as [address poisoning](https://support.metamask.io/hc/en-us/articles/11967455819035-Address-poisoning-scams) uses spoofed addresses to snare potential victims._

**This one even had the [DEA on the hook](https://www.forbes.com/sites/thomasbrewster/2023/08/24/dea-accidentally-sends-50000-in-drug-proceeds-to-crypto-scammer/) for over $50k.**

The scam relies on using a vanity address tool, such as Profanity (_ideally not the original version, which led to a [$160M loss](https://rekt.news/wintermute-rekt-2/) for ‘sophisticated actor’ Wintermute last year_) to create an address with a beginning and end which matches that of a recent transfer.

Then, this address is inserted into a victim’s transaction log via a [spoofed](https://medium.com/etherscan-blog/spoof-tokens-on-ethereum-c2ad882d9cf6) transfer of a fake token, a [zero-value token transfer](https://info.etherscan.com/zero-value-token-transfer-attack/) (now [hidden](https://twitter.com/etherscan/status/1645406189692526593) by Etherscan) or even, if deemed worth it, a real transfer of small amounts of a genuine token.

The hope is that, when setting up a future transfer, victims will copy-paste the similar looking address from their transaction history, given that usually only the first and last characters are shown.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**Much of the above techniques ultimately relates to the crypto world’s relationship to existing web2 infrastructure.**

_As well as the innovation, tenacity and sheer manpower of organised scammers, of course._

**[Botted](https://twitter.com/bneiluj/status/1702621262160208111) accounts with thousands of followers, now with added [blue checkmarks](https://twitter.com/zachxbt/status/1695929732158636422), appear as top replies under every crypto-related tweet, however jarringly irrelevant.**

While these bots may have always been there, previously buried by genuine interactions, the added visibility of (_the now [laughably](https://twitter.com/Shayan86/status/1701028462603935836) named_) ‘Verified’ status, makes them all but impossible to ignore, and more difficult to remove when reported.

The ease of SIM swapping, which is all but impossible to prevent even when [aware](https://twitter.com/jconorgrogan/status/1695042749731357175) it is happening, [threatens](https://twitter.com/tayvano_/status/1700878691046420527) the areas of the crypto industry which must still rely on the legacy web’s way of doing things.

And it sounds like there’s [plenty more to come](https://twitter.com/vxunderground/status/1705047695084175648)…

>This is T-Mobile's 8th breach since 2018
>
>This is the 3rd breach this year

**If regulators truly want to [protect the public](https://rekt.news/grudgematch-sec/) from the risks associated with crypto, tightening up accountability for providers would be an [easy win](https://twitter.com/spreekaway/status/1695104942300618943), surely welcomed by crypto-natives and crypto-sceptics alike.**

_However, the problem doesn’t stop with social media accounts…_

Registrars [handing over](https://twitter.com/Balancer/status/1704552288201883809) control of front-ends, Google ads [disguising](https://twitter.com/realScamSniffer/status/1693968263167046100) phishing links as [official URLs](https://twitter.com/0xngmi/status/1694360865154167289), and endless customer data [leaks](https://rekt.news/ledger-rekt/) - especially recent examples from the likes of [Nansen](https://twitter.com/nansen_ai/status/1705137387838574904) and (FTX’s bankruptcy claims agent) [Kroll](https://www.theblock.co/post/247356/ftx-claimant-data-compromised-in-kroll-cybersecurity-incident), providing a pre-filtered list of individuals which can be targeted in crypto scams.

_Each of these errors provide scammers with the tools they need to ensure a good haul._

---

**Crypto is a stormy sea.**

Although many retail users have left the market, [ruthless organised scamfarms](https://www.dlnews.com/articles/regulation/pig-butchering-scams-and-the-people-who-fight-back/) (_as well as [bored teens](https://www.theblock.co/post/235022/phishing-frenzy-school-kids-are-stealing-millions-of-dollars-of-nfts-to-buy-roblox-skins) on summer break_) are doubling down on extracting value, even in a bear market.

_The dream of getting rich quick, which has gradually drifted away since the days of bull market euphoria, has given way to paranoia._

Reading stories of intricate spear-phishing campaigns designed for a specific individual over an extended period, we may begin to wonder if we would even notice it happening to us…

**But, more dangerous for most are the various wider-net approaches which take advantage of users’ greed, FOMO, lack of technical knowledge, or simple human error.**

_Will you get reeled in, anon?_

_Or will you stay off the hook?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
