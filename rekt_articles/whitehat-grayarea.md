---
title: From White Hat to Gray Area
date: 06/29/2024
tags:
  - Certik
  - Audits
excerpt: Crypto security firm CertiK has been stirring up a hornet's nest lately. The dust hasn’t even settled around the controversy surrounding the recent Kraken exploit and already new allegations are swirling. From accusations of front-running bug bounties to performing superficial audits, CertiK's reputation is being put under the microscope by security researchers.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/whitehatgrayarea-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/whitehatgrayarea-header.png)









_Crypto security firm CertiK has been stirring up a hornet's nest lately._  
  
**CertiK faced backlash when it [exploited a vulnerability on Kraken](https://thedefiant.io/news/cefi/certik-holds-usd3-million-hostage-from-kraken-as-exchange-threatens-employees), siphoning $3 million from the platform in the name of "research."**  
  
The firm [returned the funds](https://thedefiant.io/news/hacks/certik-returns-usd3-million-to-kraken-amid-controversy-for-holding-funds-hostage) after receiving criticism but the damage was already done.  
  
Since the [initial Rekt News story](https://rekt.news/certik-kraken-rekt/), security researcher [Tayvano has uncovered](https://x.com/tayvano_/status/1803576961966678073) a web of suspicious activities surrounding the incident.

  
The dust hasn’t even settled around the controversy surrounding the recent Kraken exploit and already new allegations are swirling.  
  
_From accusations of front-running bug bounties to performing superficial audits, CertiK's reputation is being put under the microscope by security researchers._

  
**With each new accusation, the industry is forced to confront an uncomfortable truth.**

  
_What happens when the very entities entrusted with safeguarding the ecosystem are seen as a threat?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)
_Credit: [The Defiant](https://thedefiant.io/news/cefi/certik-holds-usd3-million-hostage-from-kraken-as-exchange-threatens-employees), [h0wl](https://x.com/h0wlu/status/1805716659338199145), [PopPunk](https://x.com/PopPunkOnChain/status/1805724086733090874), [Tayvano](https://x.com/tayvano_/status/1803576961966678073)_  
  
**In the high-stakes world of blockchain security, trust is everything. But what happens when the guardians themselves come under scrutiny?**

  

CertiK, despite its large position in the crypto security landscape, has been [viewed with skepticism](https://x.com/pcaversaccio/status/1716782089301881132) by security researchers.  
  
The recent Kraken incident wasn't just an isolated misstep, it was the match that ignited a powder keg of long-simmering concerns and criticisms.  
  
_As the dust settles, a series of alarming allegations have emerged, painting a troubling picture of CertiK's practices and ethics._  
  
### Front-Running Bug Bounties: A Breach of Trust

  

_At the heart of the storm surrounding CertiK is [OpenBounty](https://openbounty.shentu.technology/), a bug bounty platform incubated by [Shentu Chain](https://www.shentu.technology/), formerly known as CertiK Chain._

  

CertiK originally founded CertiK Chain, which [rebranded as Shentu Chain](https://shentu-chain.gitbook.io/shentu-chain/chain/faq#what-is-the-relationship-between-certik-and-shentu) in 2021.  
  
While CertiK and the Shentu Foundation are now ostensibly separate, their shared history and ongoing connections raise questions about potential conflicts of interest.

  

**What initially appeared to be a straightforward bug bounty aggregator has become the focal point of serious allegations of ethical misconduct.**

  

_Security researcher [h0wlu first sounded the alarm](https://x.com/h0wlu/status/1805716659338199145), uncovering troubling practices within OpenBounty's operations._  
  
"I created a test account on their platform to check it out, thinking maybe it's just an aggregator, but no. They have submission forms for all these programs and the findings are sent to their API servers," h0wlu reported.

  

This discovery raised immediate red flags. OpenBounty was not merely compiling bug bounty information from various sources, it was actively soliciting vulnerability reports for programs hosted on other platforms, including [ImmuneFi](https://x.com/immunefi), and even for self-hosted programs like [Uniswap](https://x.com/Uniswap) and [Ethereum](https://x.com/ethereum).  
  
_Uniswap for example clearly states in their [Bug Bounty program rules](https://uniswap.org/bug-bounty), that you must report bugs directly to them not via 3rd party._

  

The implications of this practice are severe. By funneling vulnerability reports through their own servers before reaching the affected protocols, CertiK potentially gains advanced knowledge of critical security flaws.

  

This information asymmetry could theoretically be exploited for financial gain or to pressure projects into using CertiK's services.

  

**Adding to the suspicion, [h0wlu noted that the API used by OpenBounty](https://x.com/h0wlu/status/1805716668691222800) is hosted on a subdomain containing "CertiK," further cementing the connection between the two entities.**

  

_PopPunk, co-founder of Gaslite and a vocal critic of CertiK, [expanded on these findings](https://x.com/PopPunkOnChain/status/1805724086733090874), "OpenBounty... appears to be attempting to front-run bug bounty reports.The more suspicious thing is that their website makes requests to a domain with CertiK in the name when you report a bounty.”_

  

This practice isn't just ethically questionable, it potentially violates the terms of service of many major protocols' bug bounty programs.

  

**The controversy deepened when, following these revelations, CertiK appeared to attempt a cover-up.**

  

_"CertiK is now scrubbing blog posts about OpenBounty and changed their API to a non-CertiK domain," [PopPunkOnChain claimed](https://x.com/PopPunkOnChain/status/1805968130440671491)._

  

These allegations, if proven true, strike at the very core of CertiK's credibility as a security firm.

  

The possibility that a trusted auditor could leverage its position for financial gain or to gain an unfair competitive edge raises serious concerns about the integrity and security of individual blockchain projects and, by extension, the ecosystem as a whole.  
  
### Poor Quality Audits: A Pattern of Negligence?  
  
_The allegations against CertiK extend beyond the OpenBounty controversy._

  

**Former clients and security researchers [have come forward with accusations](https://thedefiant.io/news/hacks/former-certik-clients-question-security-firm-s-stronghold-on-protocol-audits) of subpar auditing practices, painting a picture of a firm prioritizing quantity over quality.**

  

[Matías Barrios](https://x.com/0xmatiasbn), an offensive security engineer at [Halborn](https://x.com/HalbornSecurity), alleges that the firm often does "the bare minimum" when auditing protocols.

  

"Instead of running three layers of audits, which includes static analyzers, manual review, and then testing, they only did the first," Barrios [told The Defiant](https://thedefiant.io/news/hacks/former-certik-clients-question-security-firm-s-stronghold-on-protocol-audits).

  

He claims this is CertiK's modus operandi: "They go over the code through some automatic tooling, offer a very simple report, and leave it at that."

  
_The [April 2023 hack of Merlin](https://rekt.news/merlin-dex-rekt/), a Zksync-based DEX, where $1.8 million was drained post-CertiK audit, stands as a stark example of the potential consequences of inadequate security reviews._  
  
**The exploit that resulted in the $1.8 million Merlin hack directly involved the very issue CertiK had marked as resolved after their audit, raising serious concerns about the thoroughness of their security reviews.**  
  
Critics argue that CertiK's dominance in the market is less about quality and more about brand recognition.  
  
"They are so widely used because so many companies simply need the 'CertiK seal of approval,'" Barrios explained.  
  
This reliance on CertiK's reputation, rather than the substance of their audits, raises serious questions about the state of security practices in the crypto industry.  
  
**The CertiK controversy serves as a stark reminder that in the world of blockchain, even the watchdogs need watching.**  
  
_As we move forward, one question looms large, who audits the auditors?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)





**“You were put here to protect us. But who protects us from you?” - [KRS-One](https://genius.com/Boogie-down-productions-who-protects-us-from-you-lyrics)**

  

_The allegations against CertiK paint a troubling picture of a firm that may have strayed far from its mission of safeguarding the blockchain ecosystem._  
  

If proven true, these practices represent not just ethical breaches, but a fundamental betrayal of the trust placed in security auditors.  
  
**CertiK does not seem to be quick to address many of these accusations either.**  
  
Do they even care or just care about getting paid at the end of the day?

  
The potential for a trusted auditor to exploit its position for financial gain or competitive advantage poses a significant threat to the entire blockchain ecosystem.  
  

**How did one firm gain such a stronghold on protocol audits?**  
  
They are a brand name in the space, but many brand names have fallen from grace when trust was broken.  
  
Maybe more people will see “Audited by CertiK” as a warning label moving forward.  
  
**Perhaps it's time to classify the auditors themselves using a severity scale.**  
  
_Where would CertiK rank on this scale?_


![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)









