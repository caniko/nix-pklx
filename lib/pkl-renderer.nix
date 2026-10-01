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
  # Pkl 0.31's Lexer.getKeywordOrIdentifier, including reserved/blank names:
  # https://github.com/apple/pkl/blob/0.31.1/pkl-parser/src/main/java/org/pkl/parser/Lexer.java
  keywords = [
    "_"
    "abstract"
    "amends"
    "as"
    "case"
    "class"
    "const"
    "delete"
    "else"
    "extends"
    "external"
    "false"
    "fixed"
    "for"
    "function"
    "hidden"
    "if"
    "import"
    "in"
    "is"
    "let"
    "local"
    "module"
    "new"
    "nothing"
    "null"
    "open"
    "out"
    "outer"
    "override"
    "protected"
    "read"
    "record"
    "super"
    "switch"
    "this"
    "throw"
    "trace"
    "true"
    "typealias"
    "unknown"
    "vararg"
    "when"
  ];
  identifier = name:
    if lib.hasInfix "`" name || lib.hasInfix "\n" name || lib.hasInfix "\r" name
    then throw "toPkl: property names cannot contain backticks or newlines"
    # Native pklr consumers cannot parse backticks yet. Prefer regular names
    # in their shared ASCII subset; retain quoting where Pkl requires it.
    else if builtins.match "[A-Za-z_][A-Za-z_0-9]*" name != null && !(builtins.elem name keywords)
    then name
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
