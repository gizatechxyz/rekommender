---
title: Not So Safe
date: 3/13/2025
tags:
  - Safe
  - ByBit
  - DPRK
excerpt: North Korean hackers stole $1.4B from Bybit’s signers by exploiting a simple vulnerability in Safe’s system. A single yaml.load execution bypassed high-end security, turning a supposedly impenetrable system into one of the industry’s biggest disasters.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/not-so-safe-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/not-so-safe-header.png)



_North Korean hackers didn't need a zero-day exploit or billion-dollar quantum computer to pull off history's largest crypto heist._

  
**The culprits? Not some basement-dwelling script kiddies, but TraderTraitor, a North Korean hacker unit operating under the Lazarus Group umbrella.**

They socially engineered a developer into running a malicious Docker project and turned Web3's promises of security into digital ash.

  

You are one yaml.load away from losing everything – a [bitter lesson Bybit learned](https://rekt.news/bybit-rekt/) when $1.4 billion vanished from their Safe multisig, the industry's supposedly unbreakable standard.

  

Safe's security reputation shattered in a heartbeat.

  

A decade of being crypto's Fort Knox crumbled not because of some revolutionary hack, but a boring config file mistake that Kim Jong Un's cyber goons exploited with surgical precision.

  

Laugh-cry at the irony: billions guarded by battle-tested smart contracts got jacked because someone screwed up a YAML file.

  

**Web3 keeps flexing triple-audited code and mathematically perfect protocols while the billions they protect sit behind doors with Web2 locks that might as well be made of cardboard.**

  

_When a simple syntax error can cost more than the GDP of some nations, maybe it's time to ask: is our precious self-custody just an elaborate magic show where we pretend not to see the trap doors?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)
_Credit: [Safe](https://x.com/safe/status/1897663514975649938), [Ben Zhou](https://x.com/benbybit/status/1894768736084885929), [Lukas Schor](https://x.com/SchorLukas/status/1897675266819854411), [Verichains](https://docsend.com/view/rmdi832mpt8u93s7/d/xc2rkprqm799pymq), [Sygnia](https://docsend.com/view/rmdi832mpt8u93s7/d/rwecw3rumhqtgs6a), [koeppelmann](https://x.com/koeppelmann/status/1897666394118091227), [Banteg](https://x.com/bantg/status/1897669762253000994), [Andrew Mohawk](https://t.me/ETHSecurity/124234)_

  

**On Wednesday, February 26th, [ByBit CEO Ben Zhou dropped a bombshell](https://x.com/benbybit/status/1894768736084885929) that redefined the hack narrative.**

  

[Two independent forensic reports](https://x.com/benbybit/status/1894768736084885929) from Sygnia and Verichains confirmed what many feared - this wasn't a ByBit breach at all.

  

Those [initial reports implicated Safe's infrastructure](https://docsend.com/view/s/rmdi832mpt8u93s7) as the attack vector, shifting focus from Bybit's security practices to a compromised component in the Safe system itself.

  

Fast forward to March 6th, [Safe's investigation](https://x.com/safe/status/1897663514975649938), conducted in collaboration with [Mandiant](https://x.com/Mandiant), confirms what we already knew while painting an even more disturbing picture of the attack's sophistication.

  

The smoking gun? A [compromised Safe{Wallet} developer machine](https://x.com/safe/status/1897663514975649938) that became TraderTraitor’s  trojan horse into crypto's most trusted vault.

  

The North Korean hackers didn't just penetrate any system - they executed a meticulously planned [19-day operation](https://x.com/SchorLukas/status/1897675266819854411) to surgically infect the very interface Bybit's signers used to manage billions.

  

_Forget the "blockchain is unhackable" platitudes. This elite hacking crew [slipped malicious JavaScript into the Safe{Wallet} website](https://x.com/safe/status/1897663514975649938), rigging the game so only Bybit's Cold Wallet signers would trigger the trap._

  

**Every other Safe user remained blissfully unaware of the digital time bomb ticking away in their interface.**

  

When Bybit's signers initiated what they thought was a routine transfer, the malicious code swapped their [legitimate transaction with a delegate call to the attacker's contract](https://t.me/ETHSecurity/124186) - essentially handing over the keys to a $1.4 billion kingdom with a single click.

  

"The attack specifically targeted Bybit by injecting malicious JavaScript... designed to activate only when certain conditions were met." [Verichains reported](https://docsend.com/view/rmdi832mpt8u93s7/d/xc2rkprqm799pymq).

  

**[Mandiant's investigation indicates](https://x.com/safe/status/1897663514975649938) that a Safe developer’s laptop was compromised, allowing attackers to hijack AWS session tokens and bypass MFA protections - creating a single point of failure in a system designed to eliminate exactly that kind of risk.**

  

Bybit paid the $1.4 billion price tag for Safe's lax security practices.

  

For those keeping score at home, this was no opportunistic smash-and-grab.

  

[Mandiant's forensic investigation](https://x.com/safe/status/1897663514975649938) revealed the initial compromise occurred on February 4th, with the attacker first accessing Safe's AWS environment the very next day.

  

_[Wayback Machine snapshots](https://x.com/safe/status/1897663514975649938) showed the malicious code was injected two days before execution (snapshots not included in report)._

  

**The DPRK hackers had been casually browsing Safe's AWS closet, tried on their infrastructure, and walked out wearing $1.4 billion worth of ETH like a new outfit – a haul larger than the GDP of several small nations and more than double the previous record crypto heist.**

  

And what did they do after the heist?

  

Like professionals taking pride in their craft, [they covered their tracks](https://docsend.com/view/rmdi832mpt8u93s7/d/rwecw3rumhqtgs6a). "Two minutes after the malicious transaction was executed and published, new versions of the JavaScript resources were uploaded to Safe{Wallet}'s AWS S3 bucket.

  

**These updated versions had the malicious code removed," [Sygnia noted](https://docsend.com/view/rmdi832mpt8u93s7/d/rwecw3rumhqtgs6a).**

  

_Now the industry faces uncomfortable questions: If North Korea can compromise the gold standard of crypto custody by hacking a single developer laptop, what the hell are your funds secured with?_

  

### Inside the Heist  
  
_[Mandiant's forensic investigation](https://x.com/safe/status/1897663514975649938) reads like a failed movie pitch - too absurd for fiction, too real for crypto._

  

**The following timeline is based on [Mandiant’s forensic investigation](https://x.com/safe/status/1897663514975649938) - buckle up for the play-by-play of how Kim Jong Un's keyboard warriors made off with history's biggest bag…**

  

**February 2, 2025:** While VCs were busy explaining how their underwater token positions are "actually long-term strategic investments," the hackers registered the innocent-looking domain getstockprice[.]com through Namecheap. A digital breadcrumb trail revealed this was just one of several domains set up for the attack.

  

**February 4, 2025:** The trap was sprung. Developer1's MacOS workstation at Safe fell victim to social engineering, running a Docker project named MC-Based-Stock-Invest-Simulator-main. The payload connected to the attacker's freshly registered domain, giving TraderTraitor their first foothold.

  

_If this sounds eerily familiar, it should - [Mandiant confirmed this wasn't TraderTraitor’s  first rodeo](https://x.com/safe/status/1897663514975649938): "Similar stock-themed Docker projects have been utilized by UNC4899 in previous heist investigations."_

  

**Just months earlier, they'd pulled the same stock-simulator bullshit on another exchange dev who probably thought he was gonna retire early.**

  

**February 5, 2025:** No time for a coffee break - within 24 hours, North Korea's keyboard warriors were already balls deep in Safe's AWS environment. First attempt? Registering their own MFA device. Shocker - it failed.

  

_Did Kim's hackers pack up and go home? Hell no. These aren't script kiddies - they're state-sponsored professionals with nuclear missiles and nothing better to do._

  

**Plan B was way slicker: hijack Developer1's active AWS session tokens directly from the infected MacBook.**

  

And here's the chef's kiss - they synced their work schedule with the dev's, ensuring they only operated while legitimate sessions were hot. Work-life balance, Pyongyang style.

  

**February 5-17, 2025:** For twelve goddamn days, these digital ghosts ran reconnaissance in Safe's AWS playground.  
  
Picture the world's most dangerous hackers rummaging through your infrastructure while your security team is circle-jerking about blockchain immutability.

  

_Safe's intrusion detection might as well have been a sleeping guard dog farting in its dreams._

  

**Then came the digital Pearl Harbor…**

  

**February 19, 2025:** D-Day minus two. Malicious JavaScript slipped into Safe's website quicker than a rug pull at a DeFi launch.

  

The Wayback Machine caught it red-handed - digital evidence that would later twist the knife in Safe's bleeding reputation.

  

**February 21, 2025, 14:13:35 UTC:** Boom time. Bybit's signers waltz right into the trap, signing what they think is boring routine shit while actually handing over the keys to the $1.4B kingdom.

  

**February 21, 2025, 14:15:13 UTC:** Just 98 seconds later - poof! - evidence gone like cocaine at a dev conference afterparty. Clean as a whistle.

  

_If the Wayback Machine hadn't been creeping on Safe's site like your ex on Instagram, this smoking gun would've vanished into the digital ether._

  

**The cherry on this shit sundae? The whole catastrophe stemmed from a vulnerability so basic it makes zero-days look like quantum computing.**

  

The [humble yaml.load function](https://x.com/koeppelmann/status/1897666394118091227) - crypto's equivalent of leaving your house keys under the doormat while you go on vacation.

  

The function was originally designed to handle YAML data, but without proper safeguards, it would deserialize untrusted data, allowing attackers to run arbitrary code.

  

"You are one yaml.load away from losing everything," [blockchain dev Banteg warned](https://x.com/bantg/status/1897669762253000994) after the fact.

  

**Too late for Safe, which just cemented its place in crypto history - not as the gold standard of custody, but as the cautionary tale of how even the most battle-tested security can crumble from [a vulnerability that's been known and warned about for years](https://github.com/yaml/pyyaml/commit/0cedb2a0697b2bc49e4f3841b8d4590b6b15657e).**

  

_If North Korea can walk away with $1.4 billion by recycling an ancient PyYAML exploit, how many other well-documented vulnerabilities are still out there waiting to be cashed in?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)



_$1.4 billion vaporized because a developer ran a sketchy Docker project._

  

**Not due to complex cryptography failures, zero-day exploits, or quantum computer breakthroughs.**

  

Just garden-variety phishing targeting the wetware – the human – with access to the keys behind the keys.

  

This entire industry, built on revolutionary cryptographic innovations, just discovered its Achilles' heel: a vulnerability that’s been exploitable for years and could be triggered by simple misconfigurations.

  

Safe's decade-long reputation as Fort Knox 2.0 disintegrated when North Korea proved you don't need to breach the smart contract when you can just compromise the pixels displaying it.

  

_The technical autopsy reveals even more damning details. The attackers' JavaScript specifically targeted Bybit's cold wallet address, lying dormant until the perfect moment._

  

**When Bybit's signers initiated what they thought was a standard transaction, the code silently transformed it from a normal call to a delegatecall, essentially giving the attacker's contract god-mode permissions. Game over.**

  

Multisigs, hardware wallets, and cold storage mean nothing when your front-end is feeding you digital hallucinations authored by Kim Jong Un's hacker army.

  

Self-custody zealots can preach key management till they're blue in the face, but when a single yaml.load execution can bypass $1.4 billion worth of security measures, maybe it's time to admit we've been measuring the wrong metrics all along.  
  
_And just hours ago, [dev Banteg dropped a PyYAML pull request](https://github.com/yaml/pyyaml/pull/851) that rips out the insecure "Loader" alias that TraderTraitor weaponized against Bybit, finally killing the attack vector that powered history's largest crypto heist._
  
**The specific exploit? A simple line of code: data = yaml.load(response.text, Loader=yaml.Loader).**

  

Might be too little too late, but it is refreshing to see at least one backdoor slammed shut.  
  

While [Safe announced today](https://x.com/safe/status/1897993923773243860?=) that their "entire stack - including all networks, and the Safe API - is now fully restored and ready for use."  
  
While the investigation continues, many of us still want to know: exactly how did Safe's employee get played?

  

That mystery is still being unraveled.

  

**As security researcher [Andrew Mohawk wryly noted](https://t.me/ETHSecurity/124234), "Never let a good crisis go to waste. If you couldn't get security tickets prioritized, now is the time!"**

  

_When the next billion-dollar hack drops, will we still be pretending the problem is users not being careful enough?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)









