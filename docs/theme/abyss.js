(function (highlighter) {
    'use strict';

    highlighter.registerLanguage('abyss', function (language) {
        return {
            name: 'Abyss',
            aliases: ['a'],
            keywords: {
                keyword: 'if else while ret break cont struct Self import from as and or not',
                type: 'u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 bool unit',
                literal: 'true false',
            },
            contains: [
                language.COMMENT('--', '$'),
                {
                    className: 'string',
                    begin: /"/,
                    end: /"/,
                    illegal: /\n/,
                    contains: [language.BACKSLASH_ESCAPE],
                },
                {
                    className: 'string',
                    begin: /'/,
                    end: /'/,
                    illegal: /\n/,
                    contains: [language.BACKSLASH_ESCAPE],
                },
                {
                    className: 'number',
                    variants: [
                        { begin: /\b0[xX][0-9a-fA-F]+(?:_[0-9a-fA-F]+)*\b/ },
                        { begin: /(?:\b[0-9]+)?\.[0-9]+\b/ },
                        { begin: /\b[0-9]+(?:_[0-9]+)*\b/ },
                    ],
                    relevance: 0,
                },
                {
                    className: 'operator',
                    begin: /::|:=|<<=?|>>=?|[+*/%&|^=-]=|!=|<=|>=|[+*/%&|^~=:<>.-]/,
                    relevance: 0,
                },
            ],
        };
    });
}(hljs));
