from pathlib import Path
import sys

root = Path(__file__).resolve().parent.parent

def literal(source):
    return '"' + source.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n').replace('\r', '\\r').replace('\t', '\\t') + '"'

files = sorted(path for directory in ('core', 'std') for path in (root / directory).rglob('*.a'))
lines = ['import Str from "std/string"', '', 'embedded_source :: fn(path Str) Str {']
for path in files:
    name = path.relative_to(root).as_posix()
    lines.append(f'    if path.eq(Str.of({literal(name)})) {{ ret Str.of({literal(path.read_text())}) }}')
lines.extend(['    Str.empty()', '}', '', 'embedded_runtime :: fn() Str {', f'    Str.of({literal((root / "runtime/prelude.c").read_text())})', '}', ''])
content = '\n'.join(lines)
destination = root / 'compiler/embedded.a'
if '--check' in sys.argv:
    if not destination.exists() or destination.read_text() != content:
        raise SystemExit('Embedded sources are stale; run python3 scripts/embed_sources.py')
elif not destination.exists() or destination.read_text() != content:
    destination.write_text(content)
