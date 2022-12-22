# Perpetual Protocol CLI for Perp v2 Curie

[<img src="https://img.youtube.com/vi/<HModVxuLO6w>/maxresdefault.jpg" width="50%">](https://youtu.be/HModVxuLO6w)

This tool is to provide a simple, fast and efficient way to interact Perpetual Protocol contracts from your terminal.

This light-weight interface provides you with most tools you can find on the website.

Please take advantage of GitHub's Issues tab if this tool isn't working as expected in any way.

Suggested features are always welcome as well.

## Installation

### Install Binaries on UNIX-Based Systems (Linux/Apple)

Download the latest release of the PERP CLI from the [Release Page](https://github.com/brendanwenzel/perp_cli_curie/releases/tag/v0.1.0)

```bash
wget https://github.com/brendanwenzel/perp_cli_curie/releases/download/$VERSION/$FILE_NAME
sudo install $FILE_NAME /usr/local/bin/perp
rm $FILE_NAME
```

### Using Cargo and Git

Install Rustup with the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Clone this repository and install the app:

```bash
git clone https://github.com/brendanwenzel/perp_cli_curie.git
cd perp_cli_curie
cargo install --path .
```

## Environment Variables

Add a .env file to the same folder as the perp app with the following parameters:

```
RPC_URL=
CHAIN_ID=
PRIVATE_KEY=
```
The other option is to provide environment variables for each session by passing the following prior to using the CLI:

```bash
export RPC_URL="https://alchemylink.com"
export CHAIN_ID="10"
export PRIVATE_KEY="asdf...1234"
```

## Show Positions

```bash
perp position [--trader | -t <trader_address>] [--base-token | -b <base_token_address>] [--limit | -l <block_limit_number>]
```
All flags are optional. Default is 250 blocks as a limit.

### Example
No flags shows all new positions for last 250 blocks.

```bash
perp position

#Output
=====================
====  LONG: vAAVE ====
=====================
- Trader: 0x641...d48a9
- Price: 60.78736190592878
- Size: 17.173882321387264022
- Tx: 0xefbd695...3326532c32fa8

=====================
==== SHORT: vFLOW ====
=====================
- Trader: 0xc2c...fc10a
- Price: 0.9495278688064187
- Size: -358.494375555603868226
- Tx: 0x8b260ba14d...9e8e755a315ae044

=====================
==== SHORT: vBTC ====
=====================
- Trader: 0x25d...cef7da
- Price: 17122.630252547333
- Size: -0.061000235891275911
- Tx: 0x880bc37...f39c030b7f856a421ed
...
```

## Show Portfolio
This will show the portfolio for any given trader address.

If you do not supply a trader address, the address attached to the private key being used will be the default address.

```bash 
perp portfolio <trader_address>
```
### Example

```bash
Trader Address: 0xf39fd...92266
Account Value: 225124.820137 USD
Unrealized PnL: -9416.638379665977041880 USD

Available Balances
==================
- OP ETH: 0.411394464907995343
- Free Collateral: 175018.105396 USD

========================
========  vSOL  ========
========================
Index Price: 13.640000000000000000

*** Taker ***
- Position Size: -2556.924227618262286570
- Avg Entry Price: 14.690074234094554 USD
- Open Notional: 37561.406714667154610231
- Unrealized PnL: 2684.960249954056
- Leverage: 1.0026694139054007
- Liquidation Price: 85.002611627501634476

*** Maker ***
- Position Size: 0.587509237781495891
- Position Value: 91.216824433208514246 USD
- Unrealized PnL: 16.79680157012627
- Pending Fees: 0.799433824576825180
- Open Notional: -8.783175566786667

*** Total ***
- Position: -2556.336718380480790679
- Open Notional: 37552.623539100363124480
- Position Value (USD): 40263.163766191334

========================
========  vETH  ========
========================
Index Price: 1265.380000000000000000

*** Taker ***
- Position Size: -181.727726712916533141
- Avg Entry Price: 1233.4770803832303 USD
- Open Notional: 224156.985770529875530161
- Unrealized PnL: -5797.645057460468
- Leverage: 1.0004462972397306
- Liquidation Price: 2269.178318149840292140
```

## Show AMM information 

```bash 
perp amm [<pool_address> | <base_token_address> | <base_token_symbol>] [--short | -s] # --short flag prints all AMM Symbols and Addresses
```

You can filter by a specific pool address, base token address or base token symbol. This will return only that pool.

### Example

```bash
perp amm

#Output
========================
=====  vAAVE/vUSD  =====
========================
- Pool Address: 0x6c0bC93A4208EB1648AF4ED44Cb3b4df9547B42B
- Index Price: 61.450000000000000000
- vAAVE Reserves: 6406.310266117458407044
- vUSD Reserves: 360289.082560358184615367
- Price Feed: 0x7d462952c003b80fe16bbe826e4ae34cfc4aebb9
========================
=====  vAPE/vUSD  =====
========================
- Pool Address: 0x05B552C0a787c228624b389D51eB4277e1F0F348
- Index Price: 4.082976010000000000
- vAPE Reserves: 50827.521509027556523666
- vUSD Reserves: 293385.801298161842658501
- Price Feed: 0xbe7dc0896f0f0580640266ee5228942e15561331
========================
=====  vATOM/vUSD  =====
========================
- Pool Address: 0xb98e6912aE77c643957eD51dCF755895eC4BC6b4
- Index Price: 9.981000000000000000
- vATOM Reserves: 27583.491396459428978083
- vUSD Reserves: 281299.339505670902566023
- Price Feed: 0xbbfa0478ad6e5d5040cd21f7aca64e56ff3426e6
```

```bash 
perp amm --short

# Output
- vAAVE/vUSD: 0x6c0bC93A4208EB1648AF4ED44Cb3b4df9547B42B
- vAPE/vUSD: 0x05B552C0a787c228624b389D51eB4277e1F0F348
- vATOM/vUSD: 0xb98e6912aE77c643957eD51dCF755895eC4BC6b4
- vAVAX/vUSD: 0x14Bc698Fdc368f2487d3eaD12DFF458E7c272047
- vBNB/vUSD: 0xf4d40ebCBf7063D4ff56C6Df0179a86287C648dE
- vBTC/vUSD: 0xC64f9436f8Ca50CDCC096105C62DaD52FAEb1f2e
...
```

## List All Base Tokens

Simple list of base tokens available. See more details with the "amm" command.

You can get back one address by using the base token symbol. ie vBTC, vETH, etc

```bash
perp tokens [--symbol | -s <base_token_symbol>]

#Output with symbol "perp tokens --symbol vBTC"
vBTC: 0x86f1e0420c26a858fc203a3645dd1a36868f18e5

#Output with no symbol
vPERP: 0x9482aafdced6b899626f465e1fa0cf1b1418d797
vCRV: 0x7161c3416e08abaa5cd38e68d9a28e43a694e037
vFTM: 0x2db8d2db86ca3a4c7040e778244451776570359b
vETH: 0x8c835dfaa34e2ae61775e80ee29e2c724c6ae2bb
vATOM: 0x5f714b5347f0b5de9f9598e39840e176ce889b9c
vLINK: 0x2f198182ec54469195a4a06262a9431a42462373
...

```

## Deposit Collateral
Deposit any collateral token. It's recommended to hold USDC as the primary collateral.

### Options
No arguments/Flags will provide names and addresses of accepted collateral

```bash
perp deposit [<base_token_address> <amount> | --eth <amount>]
```

### Examples
```bash
perp deposit

#Output
WETH: 0x4200000000000000000000000000000000000006
USDC: 0x7f5c764cbc14f9669b88837ca1490cca17c31607
OP: 0x4200000000000000000000000000000000000042
USDT: 0x94b008aa00579c1307b0ef2c499ad98a8ce58e58
FRAX: 0x2e3d870790dc77a83dd1d18184acc7439a53f475
```

```bash
perp deposit --eth 1.39183

#Output 
Deposited 1.39183 ETH 
Transaction: 0x7c...4ddb9
```

```bash
perp deposit 0x7f5c764cbc14f9669b88837ca1490cca17c31607 1937.212

#Output
Deposited 1937.212 USDC
Transaction: 0x7c...4ddb9
```

## Withdraw Collateral
Withdraw collateral tokens.

### Options
No arguments/Flags will provide names and addresses of accepted collateral

```bash
perp withdraw [<base_token_address> <amount> | --eth <amount>]
```

### Examples
```bash
perp withdraw

#Output
WETH: 0x4200000000000000000000000000000000000006
USDC: 0x7f5c764cbc14f9669b88837ca1490cca17c31607
OP: 0x4200000000000000000000000000000000000042
USDT: 0x94b008aa00579c1307b0ef2c499ad98a8ce58e58
FRAX: 0x2e3d870790dc77a83dd1d18184acc7439a53f475
```
```bash
perp withdraw --eth 1.529

#Output 
Withdrew 1.39183 ETH 
Transaction: 0x7c...4ddb9
```

```bash
perp withdraw 0x7f5c764cbc14f9669b88837ca1490cca17c31607 1937.212

#Output
Withdrew 1937.212 USDC
Transaction: 0x7c...4ddb9
```

## Open a Position
```bash
perp open [--long | --short] <token> [--input | --output] <amount>
```

You must specify either long or short. 

<token> can be either the base token address OR the base token symbol with or without the v, but must be capitalized properly. For example, vETH or ETH will work. eth or veth will NOT work. 

You must choose whether to specify the input amount or output amount.

For example, if you are longing, input will specify the vUSD in and output will specify the base token amount out.

The opposite applies for shorting. Input will specify the base token amount in and output will specify the vUSD out.

All orders are currently setup to serve as "Market Orders" with no limit price nor slippage settings.

### Examples
```bash
perp open --long BNB --input 5000

#Output
========================
== New LONG on vBNB ==
========================

Transaction: 0xd2ff...2f7
Position Size: 20.524591260706146 vBNB
Avg Price: 243.36660041375887 USD
Fee Paid: 5 USD
```

## Close a Position
```bash
perp close <token>
```
You must specify which market you want to close with the base token. You can get the list of base tokens with addresses with the "tokens" command.

<token> can be either the base token address OR the base token symbol with or without the v, but must be capitalized properly. For example, vETH or ETH will work. eth or veth will NOT work. 

### Examples
```bash
perp close 0xb6599bd362120dc70d48409b8a08888807050700

#Output
========================
== CLOSING vBNB ==
========================

Transaction: 0x64ecedded6ce7d73595af9a4ba8280a74c1d1c36be78f57a5699a1abe9be16ad
Position Size: -38.41538448145886 vBNB
Avg Price: 243.6099477837393 USD
Fee Paid: 9.358369807620463 USD
Profit: 0.29129 USD
```

## Quit Market
Close all positions in the specific market ONLY if the market is closed and no longer active. If market is active, please use the "close" command.

```bash
perp quit <base_token_address>

#Output
Closed all vBTC positions for 1948.234556321223459 USD
```

#### To Do

- Buffer the print line statements to output all at the same time
- Add Market Price with slot0 function to AMM pairs data
- Add limit/bound orders to open command
- Threading non-dependent contract calls