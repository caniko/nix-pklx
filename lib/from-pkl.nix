{ lib }:

# Helpers for consuming Pkl values in Nix.
let
  inherit (builtins) match;

  # Map Pkl full unit names (serialized form) to short form.
  # Pkl Duration serializes as e.g. "5days", "3hours", "30minutes", "100ms".
  normalizeUnit = u: {
    days = "d";
    day = "d";
    hours = "h";
    hour = "h";
    minutes = "min";
    minute = "min";
    seconds = "s";
    second = "s";
    millis = "ms";
    micros = "us";
    nanos = "ns";
    us = "us";
    ms = "ms";
    ns = "ns";
    s = "s";
    min = "min";
    h = "h";
    d = "d";
  }.${u} or u;

  # Parse a single "<value><unit>" term and return seconds/bytes.
  parseTerm = multipliers: str:
    let
      m = match "([0-9]+)[ ]*([a-zA-Z]+)" str;
    in
      if m == null then 0
      else
        let
          value = builtins.fromJSON (builtins.head m);
          rawUnit = builtins.head (builtins.tail m);
          unit = normalizeUnit rawUnit;
          scale = multipliers.${unit} or (throw "unknown unit: ${rawUnit} (normalized: ${unit})");
        in value * scale;

  # Parse a multi-term duration string like "5days 3hours 30min".
  # The terms are separated by spaces.
  parseMultiTerm = multipliers: str:
    let
      parts = builtins.filter builtins.isString (builtins.split " " str);
    in builtins.foldl' (acc: term: acc + (parseTerm multipliers term)) 0 parts;
in {
  # Parse a Pkl Duration string (e.g. "5days 3hours") into seconds.
  # Supported units: ns, us, ms, s, min, h, d
  parseDuration = durationStr:
    let
      unitMultiplier = {
        ns = 0.000000001;
        us = 0.000001;
        ms = 0.001;
        s = 1;
        min = 60;
        h = 3600;
        d = 86400;
      };
    in
      if builtins.isString durationStr
      then parseMultiTerm unitMultiplier durationStr
      else 0;

  # Parse a Pkl DataSize string (e.g. "10mb") into bytes.
  parseDataSize = dataSizeStr:
    let
      unitMultiplier = {
        b = 1;
        kb = 1000;
        kib = 1024;
        mb = 1000000;
        mib = 1048576;
        gb = 1000000000;
        gib = 1073741824;
        tb = 1000000000000;
        tib = 1099511627776;
      };
    in
      if builtins.isString dataSizeStr
      then parseTerm unitMultiplier dataSizeStr
      else 0;
}
