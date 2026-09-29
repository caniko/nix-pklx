{lib}: let
  # Escape original characters in one pass, preserving UTF-8 and literal
  # backslash sequences. Nix strings cannot contain NUL.
  hexDigits = "0123456789abcdef";
  hex = n: "${builtins.substring (builtins.div n 16) 1 hexDigits}${builtins.substring (n - (builtins.div n 16) * 16) 1 hexDigits}";
  controls = map (n: "00${hex n}") (lib.range 1 31);
  string = text: "\"${lib.replaceStrings
    (["\\" "\""] ++ map (code: builtins.fromJSON ''"\u${code}"'') controls)
    (["\\\\" "\\\""] ++ map (code: "\\u{${code}}") controls)
    text}\"";
  identifier = name:
    if lib.hasInfix "`" name || lib.hasInfix "\n" name || lib.hasInfix "\r" name
    then throw "toPkl: property names cannot contain backticks or newlines"
    else "`${name}`";
  fields = attrs: lib.concatStringsSep "\n" (lib.mapAttrsToList (name: item: "${identifier name} = ${value item}") attrs);
  value = item:
    if item == null
    then "null"
    else if builtins.isBool item
    then
      (
        if item
        then "true"
        else "false"
      )
    else if builtins.isInt item || builtins.isFloat item
    then builtins.toJSON item
    else if builtins.isString item
    then string item
    else if builtins.isList item
    then "new Listing {\n${lib.concatMapStringsSep "\n" value item}\n}"
    else if builtins.isAttrs item
    then "new {\n${fields item}\n}"
    else throw "toPkl: unsupported Nix type ${builtins.typeOf item}";
in {
  inherit string fields value;
}
