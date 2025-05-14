---
title: Thanks Squarespace
date: 07/31/2024
tags:
  - Squarespace
  - Rekt
excerpt: Squarespace's security lapse led to widespread crypto domain hijackings, exposing Web3 vulnerabilities tied to Web2 infrastructure. The incident highlights the crypto industry's security challenges & reliance on centralized systems it aims to replace.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/thanks-squarespace-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/thanks-squarespace-header.png)


_In the world of cutting-edge blockchain technology and billion-dollar decentralized finance platforms, one might expect ironclad security._  
  
**However, recent events have shattered this illusion, exposing vulnerabilities that continue to plague the industry.**  
  
Squarespace's security lapse led to widespread crypto domain hijackings, exposing Web3 vulnerabilities tied to Web2 infrastructure.  
  
[CoinList was the first victim](https://x.com/_mwc/status/1811432212824481970) of the [recent Squarespace fiasco](https://krebsonsecurity.com/2024/07/researchers-weak-security-defaults-enabled-squarespace-domains-hijacks/) on July 9, detecting and thwarting an attack that involved unauthorized access to their Squarespace account and attempts to compromise critical third-party services.  
  
This early skirmish provided a foreboding glimpse of the broader assault that would unfold over the ensuing days, ultimately engulfing numerous prominent platforms within the crypto community.

**Between July 9-12, 2024, the crypto community was rocked by a series of domain hijackings that exploited a vulnerability not in complex smart contracts, but in the mundane world of Web2 domain management.**

  

Prominent platforms including [Celer Network](https://x.com/CelerNetwork/status/1811394743794114866), [Compound Finance](https://x.com/compoundfinance/status/1811624013841727702), [Pendle Finance](https://x.com/pendle_fi/status/1811540948431831415), and [dozens of other](https://gist.github.com/0xngmi/789e297f3107d3c28c56da7acf11828d) crypto protocols found their websites compromised through their shared hosting provider, [Squarespace](https://x.com/squarespace).

  

These incidents weren't sophisticated blockchain hacks, but rather a simple exploit of traditional web infrastructure.

  

**As millions in assets hung in the balance, the crypto world was forced to confront an uncomfortable truth: for all its revolutionary technology, the industry remains perilously reliant on the same centralized systems it seeks to replace.**

  

_In its rush to build the future of finance, has the crypto industry neglected the fundamental security lessons of the past?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)



_Credit: [Michael Coates](https://x.com/_mwc/status/1811432212824481970), [Security Alliance](https://securityalliance.notion.site/A-Squarespace-Retrospective-or-How-to-Coordinate-an-Industry-Wide-Incident-Response-fead693b66c14543a48283d85aec19ad), [0xngmi](https://gist.github.com/0xngmi/789e297f3107d3c28c56da7acf11828d), [alp1n3.eth](https://paragraph.xyz/@alp1n3.eth/inferno-drainer-malicious-javascript-analysis-from-the-squarespace-domain-hijacking), [Krebs on Security](https://krebsonsecurity.com/2024/07/researchers-weak-security-defaults-enabled-squarespace-domains-hijacks/)_

  

**The recent Squarespace domain hijackings highlight a critical vulnerability in the crypto industry's transition from Web2 to Web3.**

  

As blockchain projects strive to create decentralized systems, they often find themselves straddling two worlds, with one foot in the decentralized promise of Web3, and the other foot still firmly planted in the centralized infrastructure of Web2.

  

This hybrid state, sometimes referred to as Web 2.5, has created a unique set of security challenges.

  

_The attack vector used in the Squarespace incident, compromising domain management systems is not new._

  

**In recent years, we've seen numerous attacks targeting the Web2 infrastructure that many blockchain projects rely on.**

  

From DNS hijacking to social engineering attacks on hosting providers, hackers have repeatedly demonstrated that the weakest link in many crypto projects isn't the blockchain itself, but the traditional web systems that surround it.

  

Adding another layer of complexity to this situation is [Squarespace's acquisition of Google Domains](https://support.google.com/domains/answer/13689670?hl=en#:~:text=On%20June%2015%2C%202023%2C%20Google,customer%20accounts%20from%20Google%20Domains.) in June 2023 (closed on September 7, 2023).

  

This acquisition, which saw Squarespace take over management of millions of domains, was followed by a migration process that may have inadvertently introduced new vulnerabilities.

  

_While it's still unclear how this migration directly contributed to the recent attacks, it underscores the potential risks associated with large-scale changes to critical internet infrastructure._

  

**As we delve deeper into the Squarespace incident, it becomes clear that the attack's success hinged not on sophisticated hacking techniques, but on exploiting basic security oversights.**

  

Underscoring a critical vulnerability in the crypto world, the human factor. This is something we highlighted as a concern in [Crypto’s Achilles’ Heel](https://rekt.news/cryptos-achillesheel/).

  

The attackers capitalized on weak security defaults in Squarespace accounts, particularly focusing on email forwarding settings as a key point of entry.

  

The attack vector employed in the Squarespace incident followed a meticulously planned sequence.

  

_Attackers first gained unauthorized access to Squarespace accounts, exploiting vulnerabilities possibly introduced during the recent migration from Google Domains._

  

**Once inside, they manipulated email forwarding settings, redirecting all communications to attacker-controlled addresses.**

  

This setup allowed them to intercept critical information, including password reset links for third-party services.

  

With this foothold established, the attackers initiated a form of DNS hijacking. They targeted password resets for important third-party services, focusing on individuals likely to have admin access.

  

By intercepting these reset links, they could potentially take control of these services, gaining the ability to either directly drain funds or modify websites to include malicious code that could compromise users.

  

_The consequences of this attack were immediate and far-reaching._

  

**What began appearing as an isolated attack quickly snowballed into a sector-wide crisis over the following days.**

  

By July 12th, the crypto community was in turmoil as dozens of prominent DeFi protocols, blockchain infrastructure providers and Web3 projects reported compromised websites.

  

The situation evolved into a game of whack-a-mole, with new victims emerging hourly, each potentially exposing millions in digital assets to risk and eroding user trust across the ecosystem.

  

[Squarespace's response](https://status.squarespace.com/incidents/cw2wf55bps15) to the incident was widely criticized as inadequate.

  

_Despite being alerted to the [breach by CoinList on July 9](https://x.com/_mwc/status/1811432212824481970), the company was slow to acknowledge the problem or provide guidance to affected users._

  

**This lackluster response has raised concerns about the company's security practices and its ability to safeguard the millions of domains under its management following the Google Domains acquisition.**

  

In light of these events, security researchers have issued urgent recommendations for both affected and potentially vulnerable domains.  
  

Following is a brief breakdown of the key security solutions presented by [samczsun](https://x.com/samczsun), [tayvano](https://x.com/tayvano_) and [AndrewMohawk](https://x.com/AndrewMohawk):

  

**Short-term measures for Squarespace accounts:**

-   Enable 2FA for domain owner accounts.
    
-   Use unique, new passwords.
    
-   Remove unnecessary contributors.
    
-   Ensure all remaining contributors have 2FA and unique passwords.
    

  
  

**Short-term measure for Google Workspace:**

-   Disable reseller access if Google Workspace was bought via Google Domains.
    

  
  

**Long-term it is recommended to use a high-security domain registrar with features like:**

-   Hardware token MFA support.
    
-   Granular control over domain manager permissions.
    
-   Accessible audit trails.
    
-   Notifications for critical actions.
    

  

**Suggested high-security registrars:**

-   For non-enterprise users: [Cloudflare Registrar](https://developers.cloudflare.com/registrar/), [Amazon Route 53](https://aws.amazon.com/route53/), [dnsimple](https://dnsimple.com/).
    
-   For enterprise-level: [MarkMonitor](https://www.markmonitor.com/), [CSC](https://www.cscdbs.com/).
    

  

**Implement defense-in-depth strategies:**

-   Use multiple layers of security across infrastructure.
    
-   Example: CoinList's use of [YubiKeys](https://www.yubico.com/) and monitoring systems.
    

  

**Regularly re-evaluate attack surface:**

-   Reassess potential threat vectors, especially after changes.
    
-   Be aware that web2 providers may not consider crypto-specific threats.
    

  
For more details, check out a Squarespace retrospective, or how to coordinate an Industry-Wide incident response [by the Security Alliance](https://securityalliance.notion.site/A-Squarespace-Retrospective-or-How-to-Coordinate-an-Industry-Wide-Incident-Response-fead693b66c14543a48283d85aec19ad).  
  
Here is a [list of domains](https://gist.github.com/0xngmi/789e297f3107d3c28c56da7acf11828d) associated with Squarespace at risk provided by [0xngmi](https://x.com/0xngmi).  
  
The web3 security community got the word out and the exploits slowed down, but dYdX did not get the memo, as their [domain was hijacked on July 23rd](https://x.com/dydx/status/1815780835473129702?s=46&t=DLwbX9Nw4QECiyZQ0av-fg), despite being on the list.  
  
For those eager to dive into the technical nitty-gritty, [alp1n3.eth has provided](https://paragraph.xyz/@alp1n3.eth/inferno-drainer-malicious-javascript-analysis-from-the-squarespace-domain-hijacking) an in-depth "Inferno Drainer Malicious JavaScript Analysis from the Squarespace Domain Hijacking," focusing on the exploit that hit Compound Finance.

  
**This recent fiasco serves as a wake-up call for the crypto industry, emphasizing the need for robust security practices that bridge the gap between Web2 and Web3 technologies.**  
  
_Can the crypto world truly achieve its decentralized vision while still tethered to the very systems it aims to replace?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)



**Squarespace stands as a stark reminder of the perils of Web2 complacency in a Web3 world.**

  

_Their lackluster response and apparent disregard for the unique security needs of crypto projects have left a trail of compromised domains and shattered trust._

  
Perhaps they'll offer [$10 Uber Eats gift cards as compensation](https://techcrunch.com/2024/07/24/crowdstrike-offers-a-10-apology-gift-card-to-say-sorry-for-outage/) like Crowdstrike allegedly did, that should cover the damages, right?  
  
**This debacle serves as a wake-up call for the entire crypto industry, no matter how advanced your blockchain, you're only as secure as your weakest centralized link.**  
  
_Squarespace had almost a year to handle the migration from Google Domains._  
  
Did they spend more time choosing fonts for their error pages than securing millions of domains?  
  
It's high time for crypto projects to take a hard look at their infrastructure and implement security practices that match the high-stakes nature of their operations.  
  
As we push towards a decentralized future, can we truly revolutionize finance while still relying on the rickety scaffolding of Web2?  

When will we finally learn that too many central points of failure is a trap?
  
**Is it time for the crypto world to build its own secure foundations from the ground up?**  
  
_After all, if we can't trust a simple domain registrar, how can we expect the masses to trust the future of finance?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)









