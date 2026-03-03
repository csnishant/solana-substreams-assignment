# my_project_solana_project Substreams modules

This package was initialized via `substreams init`, using the `sol-hello-world` template.

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Modules

### `map_my_data`

This module will extract accounts from the Pump.Fun program from the 'blocks_without_votes' foundational module

# Solana Transfer Extraction using Substreams

## Backend Assignment – CLR3 Ventures

This project implements a Rust-based Substreams module that extracts transfer actions from the Solana blockchain for blocks:

385870151 to 385870156

Each transfer is output in the required JSON format:

{"from": "...", "to": "...", "amount": 123456}

---

# 📌 Objective

The goal of this assignment is to:

- Process Solana blockchain blocks
- Extract transfer instructions
- Output each transfer in the exact required JSON format
- Package the module into a valid Substreams SPKG file
- Successfully run the Substreams module against blocks 385870151–385870156

---

# 🧠 Approach

1. Installed Rust (required for Substreams module development)
2. Installed Substreams CLI
3. Obtained a Substreams API token
4. Created a Rust project for the Substreams module
5. Defined protobuf schema for transfer output
6. Implemented a map module in Rust
7. Built the SPKG package
8. Ran and verified the output

---

# 🛠 Tech Stack

- Rust
- Substreams CLI
- Solana blockchain data
- Protobuf

---

# 📂 Project Structure

```
.
├── Cargo.toml
├── src/
│   └── lib.rs
├── proto/
│   └── mydata.proto
├── substreams.yaml
└── my-project-solana-project-v0.1.0.spkg
```

---

# 📦 Protobuf Definition

```proto
syntax = "proto3";

package mydata.v1;

message Transfer {
  string from = 1;
  string to = 2;
  uint64 amount = 3;
}

message Transfers {
  repeated Transfer transfers = 1;
}
```

---

# 🦀 Rust Map Module (Core Logic)

The Rust module:

- Iterates over Solana blocks
- Iterates over transactions
- Filters relevant transfer instructions
- Extracts:
  - Sender address (base58 encoded)
  - Recipient address (base58 encoded)
  - Transfer amount (integer)
- Outputs one JSON object per transfer

---

# ⚙️ Build Instructions

### 1. Build SPKG File

```
substreams build
```

Expected result:

```
Package created successfully at my-project-solana-project-v0.1.0.spkg
```

---

# ▶️ Run Instructions

Run against required block range:

```
substreams run \
  my-project-solana-project-v0.1.0.spkg \
  map_my_data \
  --start-block 385870151 \
  --stop-block 385870157
```

Note:
Substreams uses a half-open range:
[start-block, stop-block)

So stop-block is exclusive.

---

# ✅ Required Output Format

Each transfer is emitted as:

```
{"from": "Eq1c3uNugn3FRpJ99K2q1ZyQrora3jntUqvkMnGroKkB", "to": "4XpSNfoNfireSgdg8hY6Ng3dWrHjewZhmCancypjbcCN", "amount": 325253}
```

If a block contains 5 transfers → 5 separate JSON objects are emitted.

---

# 📊 Verification

Successfully executed against:

Blocks: 385870151–385870156

Results:

- SPKG builds without errors
- Blocks processed successfully
- Transfers extracted correctly
- Output matches required format exactly

---

# 🚀 Scaling Considerations

For large-scale production usage:

- Stream blocks in real-time mode
- Filter only required program IDs
- Implement deduplication if needed
- Export to CSV / database sink
- Deploy as an indexing pipeline

---

# 🎯 Success Criteria Achieved

✔ Valid SPKG file built  
✔ Substreams executed successfully  
✔ Correct block range processed  
✔ Output matches exact required JSON format  
✔ One message per transfer  

---

# 📌 Conclusion

This implementation demonstrates:

- Ability to work with unfamiliar technologies
- Rust-based blockchain data processing
- Substreams module development
- Structured problem-solving using AI tools
- Successful extraction of Solana transfer data

Assignment Complete.