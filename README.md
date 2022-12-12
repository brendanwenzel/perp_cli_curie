# Perpetual Protocol CLI for v2 Curie on Optimism

This tool is to provide a simple, fast and efficient way to interact Perpetual Protocol contracts from your terminal.

This light-weight interface provides you with most tools you can find on the website.

Please take advantage of GitHub's Issues tab if this tool isn't working as expected in any way.

Suggested features are always welcome as well.

## Installation



## Slow Positions

```bash
perp position [--trader | -t <trader_address>] [--base_token | -b <base_token_address>] [--limit | -l <block_limit_number>]
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

```bash 
perp portfolio <trader_address>
```
### Example

```bash
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

#### To Do

- Buffer the print line statements to output all at the same time
- Add addresss JSON with reqwest to always return new addresses
- Add configuration file to setup CLI options
  - Network Toggle
    - Mainnet
    - Testnet 
  - Default Address
- Option handling for portfolio
  - perp portfolio 
    - Returns portfolio for default address
  - perp portfolio 0x93992...9493
    - Returns portfolio for user input address
- Add Market Price with slot0 function to AMM pairs data
- 