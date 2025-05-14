---
title: Recover - Ledger's Latest L
date: 05/16/2023
tags:
  - Ledger
  - Wallet
excerpt: Ledger’s new firmware update has the crypto crowd in crisis. The announcement of the new Recover service has many asking questions about the capabilities of the devices, which run on closed-source code. Trust me bro.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/ledger-header.png
---

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2023/01/ledger-header.png)

Trust me bro.

_Ledger’s new firmware update has the crypto crowd in crisis._

The option to enable an identity-based [Ledger Recovery](https://support.ledger.com/hc/en-us/articles/9579368109597-Ledger-Recover-FAQs?docs=true) service (priced at $10/mo) has many asking questions about the capabilities of the devices, which run on closed-source code.

**The manufacturers of crypto’s premier hardware wallet have [assured](https://twitter.com/Ledger/status/1658458720488046595) users that the backups required for restoring a device are not generated unless a user opts in…**

_…but Ledger doesn’t have the best track record with sensitive user data._

Almost three years ago, the names, addresses and phone numbers of a quarter of a million users were stolen, despite Ledger initially believing that less than 10k users had been affected.

**Six months [later](https://twitter.com/Ledger/status/1340769565639233536), they were published online.**

At the time, [we wrote](https://rekt.news/ledger-rekt/):

>The best case scenario is that Ledger has provided a target list for SIM swappers and phishing campaigns.

**Identity theft is trivial compared to bruteforcing a private key.**

_How long until the first Ledger wallet is ‘recovered’ by a sim-swapper?_

_And what happens when the government or law enforcement demands access to an address?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**Ledger’s latest firmware update allows users to opt into an ID-based key recovery service.**

The subscription-based Ledger Recovery service functions similarly to [Shamir’s Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing), backing-up and splitting the seed phrase into three encrypted fragments. Each of these is sent to a separate “_backup service provider_”, and can be used to restore the seed upon passing an identification check.

While it should be noted that the seed is not backed up unless a user opts into this service, the mere fact that it’s possible has caused uproar.

_And with no open-source code to check, Ledger’s word is all we have…_

**Part of any hardware wallet’s appeal is the understanding that the private keys never leave the device. In November 2022 Ledger [responded](https://twitter.com/Ledger/status/1592551225970548736) to a user's concern about potential private key leaks following firmware updates as follows:** 

>Hi - your private keys never leave the Secure Element chip, which has never been hacked. The Secure Element is 3rd party certified, and is the same technology as used in passports and credit cards. A firmware update cannot extract the private keys from the Secure Element.

It [seems](https://twitter.com/roinevirta/status/1658525016550416384) that Ledger devices, including the Secret Enclave, have been upgradable all along.

**While it may be the case that [trust assumptions remain the same as ever](https://twitter.com/adietrichs/status/1658522444817104898), the idea that the devices were manufactured with the capability to send fragments of seed phrases on to third parties seems worrying enough.**

_But considering Ledger’s history, will users continue to trust them?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png)

**Hardware wallets make it all but impossible for criminals to touch a user’s funds without gaining physical access.**

Phishing attacks may still be viable, something Ledger [helped](https://support.ledger.com/hc/en-us/articles/360015559320-E-commerce-and-Marketing-data-breach-FAQ) with three years ago…

For those more worried that the authorities (rather than crooks) are coming for their coins, the possibility that a potential [backdoor](https://old.reddit.com/r/ledgerwallet/comments/13itm7u/is_there_a_backdoor_yes_or_no/jkbyyfp/) exists is anathema.

However, as Mudit Gupta [points out](https://twitter.com/Mudit__Gupta/status/1658368265687556097), ID-based account recovery is also worryingly insecure in itself. Identity theft for the purposes of sim-swapping is [commonplace](https://www.ic3.gov/Media/Y2022/PSA220208), especially in this industry.

Recovering a lost seed phrase, or access to funds on a lost device both sound like normie-friendly features, appealing to those who find themselves put off by the paranoia of many crypto OGs.

For all the noise made by the idealogues, many casual crypto users may prefer this functionality to the responsibility of undiluted self-custody.

**And with $10/mo price tag, Ledger is certainly banking on it.**

_At least it’s [not on a CEX](https://twitter.com/_pgauthier/status/1658485171669323777), right?_

_…right?_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/08/rekt-outline-conc.png)
