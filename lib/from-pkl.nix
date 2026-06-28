{ lib }:

# Helpers for consuming Pkl values in Nix.
{
  # Parse a Pkl Duration string (e.g. "5days 3hours") into seconds.
  # Supported units: ns, us, ms, s, min, h, d
  parseDuration = durationStr:
    let
      # Extract value and unit pairs from the duration string
      # e.g. "5days 3hours" → [{value=5; unit="days"}, {value=3; unit="hours"}]
      parts = builtins.split "([0-9.]+)\\s*([a-zA-Z]+)" durationStr;
      # Map unit suffix to seconds multiplier
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
    0;  # FIXME: implement properly

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
    0;  # FIXME: implement properly
}
