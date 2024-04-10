# bevy_streaming_example

This repo contains an ongoing experiment designed to test different approaches to transferring state while reducing risk of cheating in a multiplayer RTS, and evaluate their effect on latency.

The experiment will be set up to include a few different approaches, along with a baseline. Some possible approaches to include:

- Computation only baseline: Local multiplayer only
- Remote Baseline: Peer to peer w/ matchbox + ggrs
- Server Authoratative - WASM + WebSocket functions on a CDN: same as above, but runs on a function ratehr than a container
- Server Authoratative - Google Cloud Run Container: Send server inputs, server returns currently visible state only
- Server Authoratative - Dedicated: same as above, but runs on a persistant, dedicated host (or hosts)
- Server Side Rendering + Video Stream

The game itself is just going to be a controllable swarm simulation - each play will be able to move their swarms around, and all player's should see the same movement being replicated across. The swarms should act deterministically, and the client should run in browser.

## Licenses
The project is licensed under both MIT and Apache 2. For details please look at the LICENSES folder.
