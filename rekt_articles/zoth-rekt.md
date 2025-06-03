---
title: Zoth - Rekt
date: 3/24/2025
rekt:
  amount: 8400000
  audit: N/A
  date: 3/21/2025
tags:
  - Zoth
  - Rekt
  - Admin Privileges
excerpt: Admin keys stolen, $8.4M drained in minutes through a malicious contract upgrade. Zoth suffers two hacks in three weeks - first for logic, now for keys. Auditing code is easy. Auditing the humans behind it? That’s where protocols bleed out.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/zoth-rekt-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/zoth-rekt-header.png)



_Admin privileges - DeFi's favorite skeleton key for digital heists._

  

**Zoth watched $8.4 million vanish into digital mist when their contract authority fell into the wrong hands, turning a real-world asset protocol into real-world losses on March 21st.**

  

A carefully orchestrated contract swap, executed with surgical precision, transformed Zoth's vaults into an express lane for outbound funds.

  

Just three weeks after their March 1st $285k bloodletting, Zoth's March 21st dance with disaster proves some lessons cost more to learn than others.

  

**From contract compromise to complete liquidation in minutes - DAI harvested, ETH acquired, attackers vanished.**

  

_When your admin keys become someone else's skeleton key, who's really in control of your protocol's vault?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)
_Credit: [John Doe](https://x.com/0xtroll/status/1903014129457332346), [SlowMist](https://x.com/SlowMist_Team/status/1903020756830974217), [Cyvers](https://x.com/CyversAlerts/status/1903021017460600885), [Zoth](https://x.com/zothdotio/status/1903024419028734265), [Securrtech](https://x.com/Securrtech/status/1903034664169951534), [SolidityScan](https://blog.solidityscan.com/zoth-hack-analysis-80ba3ac5076b)_

  

**When the digital knives come out, on-chain sleuths sharpen theirs.**

  

[John Doe was first on the scene](https://x.com/0xtroll/status/1903014129457332346), catching the exploit in real-time and flagging the attack before [looping in SlowMist](https://x.com/0xtroll/status/1903015590446367187) to sound the alarm.

  

Security firms swarmed the blockchain wreckage.

  

[SlowMist confirmed the exploit](https://x.com/SlowMist_Team/status/1903020756830974217) - admin keys bled out, leaving the contract wide open for a precision swap that sealed its fate.

  

_[Cyvers confirmed the kill shot](https://x.com/CyversAlerts/status/1903021017460600885) moments later - pointing to the smoking proxy contract "USD0PPSubVaultUpgradeable," upgraded by the attacker's digital fingerprints just before the slaughter began._

  

**The attack unfolded with mechanical efficiency - $8.4 million USD0++ tokens withdrawn, swapped for DAI, transferred to another address, all within minutes of the proxy contract upgrade.**

  

[Zoth's team finally surfaced](https://x.com/zothdotio/status/1903024419028734265), "Our system has experienced a security breach. We're actively investigating the incident and taking all necessary steps to resolve it as swiftly as possible."

  

[Securrtech carved the incident into bite-sized pieces](https://x.com/Securrtech/status/1903034664169951534) - compromised wallet, swapped contract, and funds drained before anyone could blink.

  

_The blockchain breadcrumbs tell the story…_  
  

**Attacker Address:**
[0x3b33c5Cd948Be5863b72cB3D6e9C0b36E67d01E5](https://etherscan.io/address/0x3b33c5cd948be5863b72cb3d6e9c0b36e67d01e5)

  

**Victim Address:**
[0x82f3a0392F58C50fa90542519832471BaE93e43e](https://etherscan.io/address/0x82f3a0392f58c50fa90542519832471bae93e43e)

  

**Attack Transaction:** [0x33bf669d125d11c432ac9b52b9d56161101c072fd8b0ac2aa390f5760fb50ca4](https://etherscan.io/tx/0x33bf669d125d11c432ac9b52b9d56161101c072fd8b0ac2aa390f5760fb50ca4)

  

**Final resting place:**
[0x7b0cd0D83565aDbB57585d0265b7D15d6D9f60cf](https://etherscan.io/address/0x7b0cd0d83565adbb57585d0265b7d15d6d9f60cf#tokentxns)

  

_The attack - brutally effective, embarrassingly simple - another chapter in DeFi's never-ending admin key tragedy._  
  

**First the keys. Then the contract. Then the money.**

  

Zoth's deployer wallet fell first.

  

8.85 million USD0++ tokens ($8.4M) vanished within minutes.

  

_Convert to DAI. Transfer away. Ride off into the sunset._

  

**No complex financial wizardry required - just god-mode admin access and stolen credentials.**

  

When lightning strikes twice, the second bolt always hits harder.

  

[Zoth's March 1st encounter with hackers](https://blog.solidityscan.com/zoth-hack-analysis-80ba3ac5076b) - a mere $285k flesh wound - seems quaint compared to today's $8.4 million slaughter.

  

[Their first exploit](https://blog.solidityscan.com/zoth-hack-analysis-80ba3ac5076b) showcased actual technical skill - manipulating Uniswap V3 liquidity pools to exploit a logic flaw in LTV validation.

  

_The attacker gamed the system to mint ZeUSD without sufficient collateral backing._

  

**SolidityScan - Zoth's own auditor - [published a detailed analysis](https://blog.solidityscan.com/zoth-hack-analysis-80ba3ac5076b) of that earlier breach, warning of validation vulnerabilities that remained wide open.**

  

Yet three weeks later, Zoth's death came not through complex financial wizardry, but through the most pedestrian of exploits - compromised admin credentials.

  

Same protocol. Different attack vectors.

  

Same result - users' funds redistributed to attackers' wallets.

  

_[An update from Zoth suggests](https://x.com/zothdotio/status/1903447852418035986?s=52&t=CwvE47heqMMRKEsuVYOjuQ) this wasn’t just an opportunistic smash-and-grab._

  

**The attacker stalked their prey for weeks, funding wallets and deploying contracts in multiple failed attempts before finally breaking through.**

  

Asset issuers locked down 73% of Zoth’s TVL right after the breach, preventing an even bigger disaster.

  

They have onboarded Crystal Blockchain BV to investigate and will share a detailed report in the coming weeks.

  

_The money’s gone, but Zoth isn’t ready to call it a loss just yet._

  

**Protocols don’t beg, but they do bargain.**

  

[Zoth & Securr are putting up a $500k bounty](https://securr.tech/zothprogram/) - help track the $8.4M, and they’ll cut you in.

  

Follow the breadcrumbs, submit your findings, and if the funds get frozen, you’ll walk away with 10% of the take.

  

**Yet as the bounty beckons, two hacks in three weeks can't be chalked up to mere misfortune.**  
  
_Is it just bad luck or a glaring sign of systemic weakness?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)



_Admin key compromises - DeFi's broken record that protocols keep dancing to._

  

**No contract audit in existence could have saved Zoth from its $8.4 million digital execution.**

  

The protocol's code wasn't the problem - the humans holding the keys were.

  

A growing graveyard of protocols have been sacrificed at the altar of lax key management.

  

**The security theater continues - audits performed, vulnerabilities patched, while admin keys sit exposed like loaded guns on playground benches.**

  

Perhaps protocols should start auditing the people who work for them - especially those whose fingerprints touch admin privileges.

  

**With each exploit, the script remains unchanged - one compromised key, one malicious contract upgrade, one unstoppable cascade of vanishing funds.**

  

_Trustless finance, they said. So why do protocols treat admin keys like party favors?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)









