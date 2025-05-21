# 🧮 Solana Counter Program (Rust + Anchor)
This is a simple on-chain counter program built using Anchor, a framework for developing smart contracts on Solana. The program allows initializing a counter, incrementing, and decrementing 
its value on the Solana blockchain.

# 🚀 Features
- Initialize a counter account

- Increment the counter

- Decrement the counter

### Built with Rust using Anchor framework

# 🛠️ Program Functions

- `initialize` -: Initializes a new counter account and sets the count to 0.
- `increment`  -: Increments the counter value by 1.
- `decrement`  -: Decrements the counter value by 1.

# 📚 Account Structures

- `Initialize` -:
   Accounts required:
    - `counter_account`: New account to store counter state (space: 8 + 8 bytes)
    - `signer` : The account paying for initialization
    - `system_program` : Solana system program

- `Change` -:
   Accounts required:
     - `counter_account` -: The mutable counter state account
     - `user` -: The signer authorized to update the counter
  


