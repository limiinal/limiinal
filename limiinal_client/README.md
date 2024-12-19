# Client Specific Docs

## GUI-specific development
By default, the backend does not run if no arguments are given.

## Backend-specific development
Firstly, `--backend-enable` is required in order for the backend to be launched.

### Current state of the backend
Mostly just using the hole_punching tut, program arguments do not differ besides the need for `--backend-enable`

For the moment to do basic communication (order here is important):
> RUST_LOG=info cargo run -- --backend-enable --secret-key-seed 1 --mode listen --relay-address /ip4/195.114.14.137/tcp/4001/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN

And the dialer:

>  RUST_LOG=info cargo run -- --backend-enable --secret-key-seed 2 --mode dial --relay-address /ip4/195.114.14.137/tcp/4001/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN --remote-peer-id 12D3KooWPjceQrSwdWXPyLLeABRXmuqt69Rg3sBYbU1Nft9HyQ6X

Then it *should just work*. Updates will go here.
