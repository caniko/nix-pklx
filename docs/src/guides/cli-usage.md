# CLI Usage

The `pklx` command-line tool provides several subcommands for working with
Pkl files from Nix.

## `pklx eval`

Evaluate a `.pkl` file and emit a Nix expression:

```bash
pklx eval <file.pkl>
```

With the `--module` flag, the output is wrapped in a NixOS module skeleton:

```bash
pklx eval --module <file.pkl>
```

Write output to a file with `--output`:

```bash
pklx eval <file.pkl> -o config.nix
```

### Examples

Simple values:

```pkl
name = "hello"
count = 42
enabled = true
```

```nix
{ name = "hello"; count = 42; enabled = true; }
```

Nested objects:

```pkl
server = new {
  host = "example.com"
  port = 443
  tls = true
}
```

```nix
{ server = { host = "example.com"; port = 443; tls = true; }; }
```

Class instances — class names are preserved:

```pkl
class User {
  name: String
  age: Int
}

users = new {
  alice = new User { name = "Alice"; age = 30 }
}
```

```nix
{ users = { alice = { __pkl_class = "User"; name = "Alice"; age = 30; }; }; }
```

## `pklx to-pkl`

Convert a JSON-compatible Nix expression to Pkl syntax:

```bash
pklx to-pkl '{"host": "example.com", "port": 443}'
```

## `pklx analyze`

List all transitive local file imports of a `.pkl` file:

```bash
pklx analyze <file.pkl>
```
