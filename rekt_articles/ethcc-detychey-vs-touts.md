---
title: EthCC - De Tychey vs Touts
date: 06/23/2022
tags:
  - EthCC
excerpt: Ticket Tout Tied up in Conference Crime Drama. A self-proclaimed group of “crypto enthusiasts” managed to trick the system for EthCC, purchasing 200 tickets with a plan to sell them on. But EthCC fought back, and now it's the scalpers who are out of pocket.
banner: https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-header.png
---
![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-header.png)
**TICKET TOUT TIED UP IN CONFERENCE CRIME DRAMA** 

**Even during a bear market, crypto conference tickets sell out fast.** 

_Would-be attendees who miss the tickets OTC, have to lobby for their lives doing deals behind the scenes._ 

_But this is “web 3”, where nothing is as simple as it seems, and it pays to bend the rules: even more with MEV._

**A self-proclaimed group of “crypto enthusiasts” managed to trick the system for EthCC, purchasing 200 tickets with a plan to sell them on.** 

But when the conference organisers stripped the NFTs of their metadata, the greedy ticket touts found themselves out of pocket.

**$68,000 for 200 useless tickets.**

**rekt.** 

_Here’s how it happened:_

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/09/rekt-investigates-linebreak.png)

**This year, EthCC used an NFT ticketing system. The NFTs contain metadata which Unlock Protocol interprets and validates in order to allow the holder access to the conference.**

The tickets are non-transferable. Attempting to call the transfer function simply throws an error, then reverts the transaction.

EthCC tickets were sold in 5 batches. On Thursday the 23rd of March, the 3rd batch of 308 tickets sold out in just 3 minutes.

**10 tickets were sold via credit cards**

**98 tickets were sold via USDC**

_And 200 tickets were scalped in USDC by a single user_

[[source]](https://medium.com/ethcc/ethcc-secondticket-wave-the-foam-of-a-scalper-ee8d8c602492)

**The above figures make it easy to see why the EthCC organisers were so concerned.** 

However, since the tickets are by nature not transferable, the scalper had to find a workaround, in order to get any utility from holding so many tickets.

To do this, the scalper created a dedicated Gnosis Safe contract for each ticket, with the intention of changing the owner of said contract, and therefore bypassing the non-transferability of the ticket. If the scalpers had done more research, they would have found that this plan was already flawed, as you can see with this example from 3 months ago, where [this early bird purchaser tries to resell for 5 ETH](https://opensea.io/assets/matic/0xd0a031d9f9486b1d914124d0c1fcac2e9e6504fe/258), and the purchase attempts fail.

The scalpers were so sure that they were onto a winner, that they went ahead and created a collection on OpenSea to try and start reselling them ([This can be viewed and reported here](https://opensea.io/collection/wrapped-ethcc-2022)). 

**Even though the tickets couldn’t be transferred, the EthCC team went one step further to defend themselves from the scalping scammers.**

By using the ticket refund mechanism, EthCC interacted with the contract function “expireAndRefundFor” with the method[1] input as 0 for each of the NFTs.

_In other words, EthCC used the pre-implemented function meant for refunding users but with the refund input as zero._

**Scalpers - REKT.**

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-function.png)

**The following charts detail the contract which was used to purchase the tickets. They are taken from the scalpers side of the story, [which can be read here](https://medium.com/@0x84003239/ethcc-stole-our-200-tickets-bf0904b9354a).**

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-charts.png)

**The scalpers used an ERC721 NFT contract ([Wrapped EthCC Tickets](https://polygonscan.com/token/0x76284b7b2f7c363779fd7338a41a202e4c8cd43a)) with a few custom tweaks:**

>On every mint: It creates a gnosis safe, buys an EthCC ticket from Unlock Protocol and sends it to the gnosis safe. Then it mints an NFT with the tokenId equal to the tokenId of the EthCC ticket NFT.

>On burn: The contract transfers ownership of the gnosis safe to the owner of the burnt wrapped NFT

>These NFT wrappers are transferable contracts for underlying non-transferrable NFTs, and they are redeemable for the underlying whenever the owner wishes.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-chart2.png)

Although this scalping method may seem clever at first glance, the minted tickets actually have no registration metadata since the normal sales funnel was by-passed. The EthCC bouncer at the door would not have been able to check and validate the ticket of the abused buyer.

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2021/03/rekt-linebreak.png) 

**De Tychey vs the touts**

_The boss man at EthCC, Jerome de Tychey, received the following DMs from the aspiring swindlers._

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-chat1.png)

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-chat2.png)

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-chat3.png)

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-chat4.png)

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-chat5.png)

![](https://raw.githubusercontent.com/RektHQ/Assets/main/images/2022/06/ethcctout-chat6.png)

**EthCC has since refunded half the amount, and that’s where the latest disagreement arises.**

The scalpers say they are entitled to a full refund, but EthCC claim that they have to retain the $34,000 as a provision to pay their VAT and corporate taxes. 

_However, EthCC told us that if they don't have to pay taxes on this sale, they'll still keep the $34,000 and donate it to Gitcoin._

Regardless of whether the scalpers are refunded or not, this story proves the often debated utility of NFTs. Yes the sale system was not perfect, but thanks to the NFT functionality, EthCC was able to disable the tickets, therefore improving the conference experience for the more honest attendees.

**Now EthCC has made a profit, and the “customer”, despite not breaking any rules, has lost $34K. If this was web 2.0, there would be no debate, but it’s still the wild west in web 3.0, where nobody is quite certain what the rules are.**

_Should EthCC refund the rest of the money?_

>Have your say in [our Telegram group](https://t.me/Rekt_HQ).
