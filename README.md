A Simple Noscript Example
====

that filters msg with `Computers & Internet` topic.

---
required:

- `wasm-pack` installed https://github.com/rustwasm/wasm-pack 
- download dataset from https://huggingface.co/datasets/yahoo_answers_topics
- place `train.csv` file in the root directory of this repo
- `cargo run --bin Noscript` will train the model and give it a test
  
## Code Structure

- `src/bayes.rs` is the bayes filter algorithm, mostly adapt from https://github.com/zenoxygen/bayespam
- `src/lib.rs` is the exported code that will be running under the Noscript runtime in the browser.
- `src/runtime.rs` is the standard Noscript runtime interface, which should be implemented and provided by nostr client.

## Test

```
wasm-pack build --target web
yarn && yarn serve
```

check browser `http://localhost:8080/` see console

### Deploy Noscript

create a `.secret` file and paste your Nostr private key

```
// generate and sign event
cargo run --bin deploy
```

## What is Noscript?

Noscript means `Nostr Script`, which refers to the idea that we can put code on Nostr protocol(save the code in a special kind of Event, sign it and send it to the relay, basically), a Noscript is a piece of code to be running in a standard environment provided by Nostr client so there is no central control but user-generated alternatives, it can be used to provide custom feeds/recommend algorithms and so much more.

right now we are experimenting Noscript with wasm runtime under the Nostr web client [flycat.club](https://github.com/digi-monkey/flycat-web)
