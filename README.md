# Blockchain-Rust

This project will

## Backgruond Knowledge
All notes are take from [MIT course](https://www.youtube.com/watch?v=IJquEYhiq_U&list=PLUl4u3cNGP61KHzhg3JIJdK08JLSlcLId&index=2)

`E-cash`: 
- digital repersentation of cash (money)
- Peer to Peer, meaning the is tracked who take it out and who has it in the end
- needs to be online to verify
- can have delay in transaction
- Lack of privacy

`Chaumian e-cash`:
- money provider never sees the secret number, only when it makes the payment to client and keeps track that no later payments uses this previous secret number
- if two same secret numbers appears on the provider side, they will know why initially sends out the secret numbers

`Hash Function`
- it encrptys the input. Given same input it will give output. Very hard to decode
- input data can be any size, output is fixed
- used for names, references, pointers, commitments

`Signature`
- GenerateKeys() -> return privateKey, publicKey pairs takes in only randomness
- Sign(secretKey, message) -> Signs a message given a secretKey returns a signature
- Verify(publicKey, message, signature) -> bool for valid or not

