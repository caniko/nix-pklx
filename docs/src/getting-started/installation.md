# Installation

## From the Nix flake

Add nix-pklx as a flake input:

```nix
inputs = {
  nix-pklx.url = "git+https://github.com/caniko/nix-pklx.git";
};
```

Then build the CLI:

```bash
nix build ".#pklx"
```

Or enter the dev shell:

```bash
nix develop
```

## From source

```bash
git clone https://github.com/caniko/nix-pklx.git
cd nix-pklx
nix develop
cargo build --release
```

The binary is at `./target/release/pklx`.
