const { transformSync } = require('@swc/core');
const path = require('path');

// 定义测试输入和期望输出
const inputCode = `
    var huge = 13164036458569646875738116129555399058346566568785520866804115739n;
`;

const expectedOutput = `
    var huge = BigInt(4357431240991) + BigInt(6) * (BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991)) + BigInt(2) * (BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991));
`;

const pluginPath = path.resolve(__dirname, '../../target/wasm32-wasip1/release/swc_plugin_bigint_literal.wasm');
const swcOptions = {
    jsc: {
        parser: {
            syntax: 'ecmascript',
        },
        experimental: {
            plugins: [
                [pluginPath, {}]
            ]
        },
        preserveAllComments: true,
        target: 'es5',
        minify: {
            compress: false,
            mangle: false,
        },
    },
    minify: false,
    isModule: false,
};

// 测试用例
test('BigInt literal transformation', () => {
    const output = transformSync(inputCode, swcOptions).code;
    expect(output.trim()).toBe(expectedOutput.trim());
});