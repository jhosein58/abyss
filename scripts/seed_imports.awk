/^[[:space:]]*[a-zA-Z_][a-zA-Z_0-9]* :: @import\("(\.\/)?[a-zA-Z_0-9\/]+"\)(\.[a-zA-Z_][a-zA-Z_0-9]*)?[[:space:]]*$/ {
    line = $0
    sub(/^[[:space:]]*/, "", line)
    indent = substr($0, 1, length($0) - length(line))
    split(line, parts, "\"")
    path = parts[2]
    sub(/^\.\//, ".", path)
    gsub(/\//, ".", path)
    split(parts[1], binding, " ")
    member = parts[3]
    sub(/^\)\.?/, "", member)
    sub(/[[:space:]]+$/, "", member)
    if (member == "") {
        print indent "import " path " as " binding[1]
    } else {
        print indent "import " path ": " member (member == binding[1] ? "" : " as " binding[1])
    }
    next
}
{ print }
