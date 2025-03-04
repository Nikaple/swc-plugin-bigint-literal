# SWC Plugin BigInt Literal

A SWC compiler plugin that optimizes BigInt literals in JavaScript code. When a BigInt literal is within the safe integer range, the plugin transforms it into a `BigInt()` function call.

## Features

- Supports transformation of all numeric literal formats:
  - Decimal (e.g., `123n`)
  - Hexadecimal (e.g., `0xffn`)
  - Binary (e.g., `0b1111n`)
  - Octal (e.g., `0o777n`)
- Preserves original number representation format
- Supports negative numbers
- Supports transformation within expressions

## Transform Examples

```javascript
// Decimal
123n          -> BigInt(123)

// Hexadecimal
0xffn         -> BigInt(0xff)

// Binary
0b1111n       -> BigInt(0b1111)

// Octal
0o777n        -> BigInt(0o777)

// Negative numbers
-42n          -> BigInt(-42)
-0xffn        -> BigInt(-0xff)

// Expressions
1n + 2n       -> BigInt(1) + BigInt(2)
```

## Limitations

1. Safe Integer Range
   - Only processes BigInt literals within JavaScript's safe integer range
   - Range: [-2^53 + 1, 2^53 - 1], i.e., [-9007199254740991, 9007199254740991]
   - BigInt literals outside this range remain unchanged

2. Untransformed Cases:
   ```javascript
   // Numbers outside safe integer range remain unchanged
   9007199254740992n   // remains as is
   -9007199254740992n  // remains as is
   
   // Numbers in expressions that result in values outside the range will still be transformed
   // (Note: This may lead to inconsistent calculation results)
   9007199254740990n + 2n  -> BigInt(9007199254740990) + BigInt(2)
   ```

## Usage

1. Install the plugin:
   ```bash
   npm install --save-dev swc-plugin-bigint-literal
   ```

2. Configure `.swcrc`:
   ```json
   {
     "jsc": {
       "experimental": {
         "plugins": [
           ["swc-plugin-bigint-literal.wasm", {}]
         ]
       }
     }
   }
   ```

## Important Notes

1. Performance Considerations
   - For BigInt operations within the safe integer range, using `BigInt()` function may be more performant than literal notation
   - For numbers outside the range, keeping the literal notation is more appropriate

2. Compatibility
   - Ensure your runtime environment supports BigInt
   - Requires ES2020 or later

## Development Requirements

- Rust
- wasm32-wasip1 target
- Node.js >= 14.0.0

## License

MIT
