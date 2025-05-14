---
title: 1Inch - Rekt
date: 3/13/2025
rekt:
  amount: 5000000
  audit: N/A
  date: 3/5/2025
tags:
  - 1Inch
  - Rekt
excerpt: One hacker transformed 1inch resolver contracts into a $5 million ATM through an integer underflow exploit - all with a negative 512 value. Attacker pocketed $450K as a "bounty" for exposing two years of an undetected vulnerability.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/1inch-rekt-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/1inch-rekt-header.png)




_Five million dollars vanished when 1inch's deprecated code returned from the dead to haunt its creators._

  

**A few market makers, or resolvers, watched helplessly as a calldata corruption vulnerability transformed their contracts into ATMs for a crafty attacker who needed nothing more than basic arithmetic and an integer overflow.**

  

The _settleOrder function, supposed to be long retired, was a relic from 1inch's earlier architecture where vulnerabilities could silently thrive - a perfect attack vector hiding in plain sight.

  

[Nine audit teams](https://github.com/1inch/1inch-audits/tree/master/Fusion%20mode%20and%20Token-plugins) missed it. Two years passed without incident.

  

Then one hacker with a calculator and a dream discovered that setting an interaction length to negative 512 could underflow memory pointers and redirect suffix data - transforming a simple integer trick into positive millions.

  

The exploit's elegance was brutal in its simplicity - cracking a nine-times-audited vault with nothing but a negative number and the audacity to try it.

  

Mid-heist, our digital bank robber took a moment to slide a note across the counter: "Can I have bounty?"

  

**Because in crypto's backwards universe, robbers expect tip jars at the scenes of their crimes.**

  

_Still think those security guarantees are worth the PDFs they're printed on?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)
_Credit: [1Inch](https://blog.1inch.io/vulnerability-discovered-in-resolver-contract/), [Slow Mist](https://x.com/SlowMist_Team/status/1897945772307759483), [Decurity](https://blog.decurity.io/yul-calldata-corruption-1inch-postmortem-a7ea7a53bfd9), [shoucccc](https://x.com/shoucccc/status/1897954751205327040)_

  

**On March 6th, 1inch announced the discovery of a vulnerability in their obsolete Fusion v1 resolver contracts, correctly stating that 'No end-user funds were at risk.' But their message didn't mention that about $5 million had already been stolen from resolvers' funds.**

  

Just another day patching legacy code.

  

But something critical was missing from their initial postmortem - about $5 million in actual stolen funds.

  

The full extent of the theft didn't come to light until security firm [SlowMist conducted their own investigation](https://x.com/SlowMist_Team/status/1897945772307759483) the next day. They [uncovered what 1inch conveniently missed](https://x.com/SlowMist_Team/status/1897958914114879656): 2.4 million USDC and 1,276 WETH already gone - digital smoke where money used to be.  
  
_The [attacker's wallet funded through Tornado Cash](https://etherscan.io/tx/0xe2580257c7485120ff0167f2015e9b1ccecf691166832c1436a30127ef93522e) began its methodical assault on what should have been battle-tested contracts._  
  

**Attacker Address (Exploit Deployer):**

[0xa7264a43a57ca17012148c46adbc15a5f951766e](https://etherscan.io/address/0xa7264a43a57ca17012148c46adbc15a5f951766e)

**Attack Contract:**
[0x019bfc71d43c3492926d4a9a6c781f36706970c9](https://etherscan.io/address/0x019bfc71d43c3492926d4a9a6c781f36706970c9)

  
**Funds Receiver:**
[0xbbb587e59251d219a7a05ce989ec1969c01522c0](https://etherscan.io/address/0xbbb587e59251d219a7a05ce989ec1969c01522c0)

  

**Victim addresses:**

  

**TrustedVolumes:** 
[0xb02f39e382c90160eb816de5e0e428ac771d77b5](https://etherscan.io/address/0xb02f39e382c90160eb816de5e0e428ac771d77b5)

  
**1inch Settlement V1:**  
[0xa88800cd213da5ae406ce248380802bd53b47647](https://etherscan.io/address/0xa88800cd213da5ae406ce248380802bd53b47647)

  

**The attack unfolded through a series of meticulously crafted transactions:**

[0x04975648e0db631b0620759ca934861830472678dae82b4bed493f1e1e3ed03a](https://etherscan.io/tx/0x04975648e0db631b0620759ca934861830472678dae82b4bed493f1e1e3ed03a)

[0xb5c94efa0c8fd8f5c8cc2826e374a99620b01061d395b59b8f45dddc9fce1c60](https://etherscan.io/tx/0xb5c94efa0c8fd8f5c8cc2826e374a99620b01061d395b59b8f45dddc9fce1c60)

[0xb16bbf03d324b66685c94d62dbe31c739ee23c114b3915d169c74cd7c98eec8c](https://etherscan.io/tx/0xb16bbf03d324b66685c94d62dbe31c739ee23c114b3915d169c74cd7c98eec8c)

[0x3947e5a4d98104e313e08ee321673e1183db3d6ff8b7207f3eabb36f71436c1d ](https://etherscan.io/tx/0x3947e5a4d98104e313e08ee321673e1183db3d6ff8b7207f3eabb36f71436c1d)

[0x9ce5187c7160f531189e4765f21af5975dc2a62d961fb61ae09866d082918256](https://etherscan.io/tx/0x9ce5187c7160f531189e4765f21af5975dc2a62d961fb61ae09866d082918256)

[0xb0688eb1f46c28f36d7397366146fced23d3f8da7e08b760a5f612ce134ee9d2](https://etherscan.io/tx/0xb0688eb1f46c28f36d7397366146fced23d3f8da7e08b760a5f612ce134ee9d2)

[0x62734ce80311e64630a009dd101a967ea0a9c012fabbfce8eac90f0f4ca090d6](https://etherscan.io/tx/0x62734ce80311e64630a009dd101a967ea0a9c012fabbfce8eac90f0f4ca090d6)

  

Regular users scrolled through their untouched balances while market makers like TrustedVolumes watched their funds vanish faster than exchange executives during a bear market.

  
_Forget million-dollar exploits and state-of-the-art hacking tools. Our attacker brought a calculator to a gunfight - and won._

  

**Each transaction followed the same playbook:**

  

-   Create a normal order swapping pennies for millions.
    
-   Pad it with null-bytes.
    
-   Set interactionLength to "-512" (0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00).
    
-   Add a fake suffix structure as an interaction.
    
-   Watch as memory pointers underflow and redirect control to TrustedVolumes.
    

  

Like clockwork, resolver after resolver fell as the 0x1944799f function signature ("resolveOrders") became the skeleton key to everyone's vaults.

  

**The attack climaxed with a series of transactions in rapid succession:**

[0xc69b4c8029c70ae468e92af31120ac6b01bb89c6e35d34818413e9942aedebb6](https://etherscan.io/tx/0xc69b4c8029c70ae468e92af31120ac6b01bb89c6e35d34818413e9942aedebb6)

[0xefcb740bf9ec17ed99839ffcc05393fae5ec2d44149aee91ba119f48bc20a1ef ](https://etherscan.io/tx/0xefcb740bf9ec17ed99839ffcc05393fae5ec2d44149aee91ba119f48bc20a1ef)

[0x74bc4d5dc7f8da468788da6087bb9f73465966ab5b8cf9cf1053d98e78a9bf96](https://etherscan.io/tx/0x74bc4d5dc7f8da468788da6087bb9f73465966ab5b8cf9cf1053d98e78a9bf96)

By 5:54 PM UTC, the damage was done.

  

Want the magic number that turned millions of dollars into thin air? 512.

  

Or in hexadecimal: 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00—for those who prefer their nightmares in base 16.

  

_With that single value, the attacker weaponized integer underflow—sending memory pointers spiraling backward, hijacking resolver addresses, and redirecting function calls like a traffic cop on bath salts._

  

**The mechanics would make even the most jaded CTF players slow-clap in appreciation.**

  

Create a normal-looking order to swap a few wei for millions. Pad with zeros. Use the negative interactionLength to corrupt calldata and hijack the resolver address.

  

Resolver security checks? Bypassed.

Protocol safeguards? Sidestepped.

Audit guarantees? Worthless.

  

**All with a four-byte integer trick that would barely earn partial credit in a freshman coding class.**

  

_When the most dangerous security threats come from elementary school math, what exactly are we paying those nine audit teams to find? Advanced calculus?_

  

### The Aftermath  
  

_In DeFi's twisted universe, stealing money is just the beginning of the negotiation._

  

**Our hacker, fresh off a multimillion-dollar heist, sent an [on-chain olive branch](https://etherscan.io/tx/0xf70d0aa5a3692540fc484945e9f50b672e4b752ff366a03d92cd5e1a1b459301): "Can I have bounty?" - the DeFi equivalent of asking for a tip after cracking the vault.**

  

What followed wasn't the expected "go to hell" but instead a surprising twist: [TrustedVolumes slid into the thief's on-chain DMs](https://etherscan.io/idm?addresses=0xbbb587e59251d219a7a05ce989ec1969c01522c0%2C0x1ef9bfb1e7480c01d3d00e9bca5f29625c6c4806&type=1) to start haggling.

  

Five hours later, at 11:55 PM UTC, [negotiations concluded](https://etherscan.io/idm?addresses=0xbbb587e59251d219a7a05ce989ec1969c01522c0%2C0x1ef9bfb1e7480c01d3d00e9bca5f29625c6c4806&type=1) with blockchain's most awkward handshake.

  

By midnight, the attacker started trickling funds back to TrustedVolumes, keeping just enough for what Crypto Twitter would generously call a "white hat fee" - because nothing says "ethical hacker" like stealing millions and returning most of it.

  

By 4:12 AM on March 6th, most funds were returned, with the attacker pocketing roughly $450,000 - a 10% "finders fee" that makes traditional robbery seem inefficient by comparison.  
  
**Funds Returned Transaction 1 ($2.4M USDC):**
[0x99ff2067bfa6f5e30afcefc45477cb5bb661d85890ece002a4a0ce348a3c6e7a](https://etherscan.io/tx/0x99ff2067bfa6f5e30afcefc45477cb5bb661d85890ece002a4a0ce348a3c6e7a)

  
**Funds Returned Transaction 2 (1076 ETH - $2.09M):**
[0xbe270b797de02c382df8c569813837fc0a6bb97809fd8a512b50c87c750bc367](https://etherscan.io/tx/0xbe270b797de02c382df8c569813837fc0a6bb97809fd8a512b50c87c750bc367)

  

The fun part? The hacker's fumble created a secondary comedy of errors.

  

In the chaos of moving stolen funds, they accidentally sent half their haul to the 1inch settlement contract itself - essentially dropping millions into a public swimming pool.

  

This [triggered a mad dash as shoucccc and others raced](https://x.com/shoucccc/status/1897954751205327040) to snatch the exposed funds:

  

"The funny thing is the hacker incorrectly transferred half of the stolen funds to the 1inch settlement contract, making the funds available for everyone to grab, and he spent quite sometime to get funds back. We were trying to compete but the hacker got it first unfortunately."

  

_A multi-million dollar game of finders-keepers, with the original thief scrambling to recover their own stolen goods before someone else stole them again. Only in crypto._

  

**2 days after the hack, [security firm Decurity released](https://blog.decurity.io/yul-calldata-corruption-1inch-postmortem-a7ea7a53bfd9) one of the most comprehensive and lightning-fast postmortem analyses in recent history.**

  

Researcher Omar Ganiev's breakdown was so thorough and timely it made some security firms look like they're still using carrier pigeons.

  

Ganiev distilled the soul-crushing reality: [despite nine audit teams and multiple reviews](https://github.com/1inch/1inch-audits/tree/master/Fusion%20mode%20and%20Token-plugins), the fatal flaw was nothing more than a glorified buffer overflow - a bug too basic for auditors to catch, yet sitting in plain sight for years, waiting for its moment.

  

_The [bug slithered into existence in November 2022](https://blog.decurity.io/yul-calldata-corruption-1inch-postmortem-a7ea7a53bfd9) when 1inch traded Solidity for Yul - a language switch that birthed a memory overflow monster that nine audit teams couldn't spot with a telescope._

  

**This wasn't just a subtle bug - it was yet another red flag in institutional blindness and security theater as highlighted in [Decurity's remarkably candid postmortem](https://blog.decurity.io/yul-calldata-corruption-1inch-postmortem-a7ea7a53bfd9) that exposed their own role in the oversight.**

  

Audit round one kicked off in October 2022 on perfectly clean code.

  

Then November came, and 1inch switched from Solidity to Yul - like swapping your family sedan for a Formula 1 car without checking if anyone knew how to drive it.

  

By December, multiple firms were pawing through the refactored code, but conveniently the _settleOrder function fell into that magical "not in scope" dimension where vulnerabilities go to thrive.

  

_March 2023 brought a glimmer of hope when Decurity actually spotted the integer overflow risk._

  

**But in classic DeFi fashion, 24 hours later a new implementation appeared, and everyone forgot the old infected code was already loose in the wild - like noticing your lab sample has Ebola but forgetting you already mailed copies to everyone on your Christmas list.**

  

For over two years, the bug lounged in production code, sipping cocktails, undisturbed by the parade of security professionals who couldn't see what was hiding in plain sight.

  

No one exploited it not because it was cleverly concealed, but because everyone was too busy admiring the emperor's new audit reports to notice he was actually naked.

  

The real lesson? A vulnerability doesn't expire with age. It just waits for someone who bothers to actually look.

  

**The final tally: TrustedVolumes got most of their $4.5M back minus the 10% 'bounty' the attacker kept ($450K).**

  

The 1inch protocol itself slipped away technically unharmed - unless you count the reputational equivalent of hosting a $5 million bank robbery on your property using keys you should have changed years ago.

  

For their part, [1inch announced a bug bounty program](https://hackenproof.com/programs/1inch-smart-contract?ref=blog.1inch.io) faster than you can say "stable door, missing horse" and [urged resolvers to "audit and update their contracts immediately](https://x.com/1inch/status/1897695349818487002)" - advice that might have been more useful before their resolver partners got sliced.

  

Audits aren't magical protection spells - they're expensive snapshots by people who rarely verify what actually gets deployed after cashing their checks.

  

**The real kicker? We're still learning the same lesson, sometimes the deadliest weapon isn't a zero-day exploit or quantum computing breakthrough - it's a negative number and the audacity to use it.**

  

_When your multi-billion dollar industry's security model can be unraveled by arithmetic that wouldn't stump a fifth-grader, who's really being naive—the hacker who tried it, or the protocol that never thought they would?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

_Multiple audits, multiple firms, two years of smug security guarantees - and still rekt by fourth-grade arithmetic._

  

**Welcome to DeFi's most expensive security theater, where the tickets cost millions and the actors don't know their lines.**

  

1inch's bugs sat in broad daylight like financial IEDs, patiently waiting for someone brave enough to attempt the most dangerous act in crypto: adding negative numbers.

  

Smart contracts don't decay, but human attention does.

  

What nine firms stamped as "secure" in 2023 transformed into a hacker's personal ATM in 2025 - all because no one bothered to check whether yesterday's "safe" code might nuke tomorrow's systems when they meet.

  

**The uncomfortable truth? Projects brandish audit reports like medieval talismans against evil spirits, while your most thorough auditors remain the hackers who examine your code with dollar signs in their eyes and empty wallets in their hands.**

  

The only question is whether you'll pay them before or after they've emptied your treasury.

  

1inch only found religion about security after $5 million had already walked out the door - proving once again that in DeFi, security consciousness arrives exactly one exploit too late.

  

**In a rare display of accountability almost as unusual as a solvent crypto exchange, Decurity confessed to its own role in this circus - admitting they actually spotted the bug in March 2023 but lost track of it when the code was refactored, like a security guard who notices the bank vault is unlocked but gets distracted by a doughnut.**

  

_Will an industry worth billions ever realize that paying hackers after they rob you isn't a security strategy, it's Stockholm Syndrome with a blockchain address?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)









