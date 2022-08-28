# reqores

Oversimplified http request/response abstraction layer for `bot_any` packages.

## Goal

- Simple & Small: Due to size limitation from Cloudflare Workers.

## Package Structure

- reqores: The main abstraction layer just containing Rqeuest/Response traits for both client & server.

## FAQ

### Why not [`http-types`](https://crates.io/crates/http-types)?

It has too complex overwhelming API.
We don't need async-body. streaming response, etc.

### What is the meaning of the name

Req(uest) or Res(ponse).
But actually the name has been inspired inspired by the animation, [Lycoris Recoil](https://lycorisrecoil.com/).

## Acknowledgement

- The structure has been heavily inspired by [kiwiyou/telbot](https://github.com/kiwiyou/telbot/)