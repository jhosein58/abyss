import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { dirname, extname, join, relative, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { runInNewContext } from 'node:vm';

const docsDirectory = dirname(fileURLToPath(import.meta.url));
const repositoryDirectory = dirname(docsDirectory);
const bookDirectory = join(docsDirectory, 'book');
const workDirectory = join(docsDirectory, '.verification');

function filesIn(directory) {
    return readdirSync(directory, { withFileTypes: true }).flatMap(entry => {
        const path = join(directory, entry.name);
        return entry.isDirectory() ? filesIn(path) : entry.isFile() ? [path] : [];
    });
}

function sourceHashes() {
    const paths = readdirSync(repositoryDirectory, { withFileTypes: true })
        .filter(entry => entry.isFile())
        .map(entry => join(repositoryDirectory, entry.name));
    paths.push(...filesIn(join(repositoryDirectory, 'compiler')));
    paths.push(...filesIn(join(repositoryDirectory, 'std')));
    return Object.fromEntries(paths.sort().map(path => [
        relative(repositoryDirectory, path),
        createHash('sha256').update(readFileSync(path)).digest('hex'),
    ]));
}

function run(command, args, cwd) {
    const result = spawnSync(command, args, {
        cwd,
        encoding: 'utf8',
        timeout: 120000,
        maxBuffer: 4 * 1024 * 1024,
    });
    assert.ifError(result.error);
    assert.equal(result.status, 0, `${command} failed:\n${result.stdout}\n${result.stderr}`);
    return result.stdout;
}

function checkBook() {
    assert.ok(existsSync(bookDirectory), 'Run mdbook build docs before verification');
    const summary = readFileSync(join(docsDirectory, 'src/SUMMARY.md'), 'utf8');
    const chapters = [...summary.matchAll(/\]\(([^)]+\.md)\)/g)].map(match => match[1]);
    assert.equal(chapters.length, 8);
    for (const chapter of chapters) {
        assert.ok(existsSync(join(docsDirectory, 'src', chapter)), `Missing chapter: ${chapter}`);
        assert.ok(existsSync(join(bookDirectory, chapter.replace(/\.md$/, '.html'))));
    }

    const pages = filesIn(bookDirectory).filter(path => extname(path) === '.html');
    const identifiers = new Map(pages.map(path => [path, new Set(
        [...readFileSync(path, 'utf8').matchAll(/\bid="([^"]+)"/g)].map(match => match[1]),
    )]));
    let linkCount = 0;
    for (const page of pages) {
        const content = readFileSync(page, 'utf8');
        for (const match of content.matchAll(/<(?:a|link|script|img|iframe|source)\b[^>]*?\b(?:href|src)="([^"]+)"/g)) {
            const url = match[1].replace(/&amp;/g, '&');
            if (/^(?:[a-z][a-z\d+.-]*:|\/\/)/i.test(url)) continue;
            const [resource, fragment] = url.split('#');
            const pathPart = resource.split('?')[0];
            const target = pathPart ? resolve(dirname(page), decodeURIComponent(pathPart)) : page;
            assert.ok(!relative(bookDirectory, target).startsWith('..'), `Link escapes book: ${url}`);
            assert.ok(existsSync(target), `Broken resource in ${relative(bookDirectory, page)}: ${url}`);
            if (fragment && extname(target) === '.html') {
                assert.ok(identifiers.get(target)?.has(decodeURIComponent(fragment)),
                    `Broken fragment in ${relative(bookDirectory, page)}: ${url}`);
            }
            linkCount += 1;
        }
        if (/class="language-(?:abyss|a)"/.test(content)) {
            const scripts = [...content.matchAll(/<script src="([^"]+)"/g)].map(match => match[1]);
            const highlightIndex = scripts.findIndex(path => /^highlight[.-]/.test(path));
            const abyssIndex = scripts.findIndex(path => /^theme\/abyss[.-]/.test(path));
            const bookIndex = scripts.findIndex(path => /^book[.-]/.test(path));
            assert.ok(highlightIndex >= 0 && abyssIndex > highlightIndex && bookIndex > abyssIndex,
                `Incorrect highlighting script order: ${page}`);
        }
    }
    console.log(`Book: ${chapters.length} chapters, ${linkCount} local links and resources verified`);
}

function checkHighlighting() {
    const context = { console };
    runInNewContext(readFileSync(join(docsDirectory, 'theme/highlight.js'), 'utf8'), context);
    runInNewContext(readFileSync(join(docsDirectory, 'theme/abyss.js'), 'utf8'), context);
    const highlighter = context.hljs;
    highlighter.debugMode();
    assert.ok(highlighter.getLanguage('abyss'));
    assert.equal(highlighter.getLanguage('a'), highlighter.getLanguage('abyss'));

    const categories = {
        keyword: 'if else while ret break cont struct Self import from as and or not',
        type: 'u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 bool unit',
        literal: 'true false',
        number: '0 1_024 0xAB_CD 0XFF .5 0.25',
        operator: ':: := : & * . + - / % == != < > <= >= = << >> += -= *= /= %= &= |= ^= <<= >>= | ^ ~',
    };
    for (const [category, words] of Object.entries(categories)) {
        for (const word of words.split(' ')) {
            const rendered = highlighter.highlight('abyss', word, true).value;
            assert.ok(rendered.startsWith(`<span class="hljs-${category}">`),
                `Missing ${category} highlighting for ${word}: ${rendered}`);
        }
    }
    const comment = highlighter.highlight('abyss', '-- if u32 true 42', true).value;
    assert.ok(comment.startsWith('<span class="hljs-comment">'));
    assert.ok(!comment.includes('hljs-keyword'));
    const quoted = highlighter.highlight('abyss', '"if -- true"', true).value;
    assert.ok(quoted.startsWith('<span class="hljs-string">'));
    assert.ok(!quoted.includes('hljs-comment'));
    const escaped = highlighter.highlight('a', String.raw`"quoted \"word\" -- text"`, true).value;
    assert.ok(!escaped.includes('hljs-comment'));
    assert.equal(highlighter.highlight('abyss', 'u32_buffer true_value', true).value, 'u32_buffer true_value');

    let snippetCount = 0;
    for (const path of filesIn(join(docsDirectory, 'src')).filter(path => path.endsWith('.md'))) {
        const text = readFileSync(path, 'utf8');
        for (const match of text.matchAll(/```(abyss|a)\n([\s\S]*?)\n```/g)) {
            const result = highlighter.highlight(match[1], match[2], true);
            assert.ok(!result.errorRaised, `Highlight error in ${path}`);
            assert.equal(result.illegal, false, `Illegal highlight in ${path}`);
            snippetCount += 1;
        }
    }
    console.log(`Highlighting: both aliases, token categories, and ${snippetCount} book snippets verified`);
}

function checkExample() {
    mkdirSync(join(workDirectory, 'tmp'), { recursive: true });
    for (const name of ['main.a', 'arithmetic.a']) {
        writeFileSync(join(workDirectory, name), readFileSync(join(docsDirectory, 'examples', name)));
    }
    writeFileSync(join(workDirectory, 'prelude.c'), readFileSync(join(repositoryDirectory, 'prelude.c')));
    run(join(repositoryDirectory, 'abyssc'), [], workDirectory);
    run('gcc', ['-std=gnu99', '-O2', 'tmp/out.c', '-o', 'tmp/example'], workDirectory);
    const output = run(join(workDirectory, 'tmp/example'), [], workDirectory);
    assert.equal(output, 'documentation examples passed\n');
    console.log('Abyss example: compiled to C, compiled with GCC, and passed runtime checks');
}

const initialHashes = sourceHashes();
try {
    checkBook();
    checkHighlighting();
    checkExample();
} finally {
    assert.deepEqual(sourceHashes(), initialHashes, 'A file outside docs changed during verification');
}
