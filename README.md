# SWC Plugin BigInt Literal

A SWC compiler plugin that transforms BigInt literals in JavaScript code. It converts BigInt literals into `BigInt()` function calls and decomposes large numbers into arithmetic expressions.

## Features

- Supports transformation of all numeric literal formats:
  - Decimal (e.g., `123n`)
  - Hexadecimal (e.g., `0xffn`)
  - Binary (e.g., `0b1111n`)
  - Octal (e.g., `0o777n`)
- Handles numbers both within and beyond the safe integer range
- Preserves original number representation format
- Supports negative numbers
- Supports transformation within expressions

## Transform Examples

```javascript
// Basic transformations
123n          -> BigInt(123)
0xffn         -> BigInt(255)
0b1111n       -> BigInt(15)
0o777n        -> BigInt(511)

// Negative numbers
-42n          -> -BigInt(42)
-0xffn        -> -BigInt(255)

// Expressions
1n + 2n       -> BigInt(1) + BigInt(2)

// Large numbers
9007199254740992n -> BigInt(9007199254740991) + BigInt(1)

// Very large numbers
730750818665451215712927172538123444058715062272n -> 
    BigInt(9007199254740991) * BigInt(9007199254740991) * BigInt(9007199254740991) + BigInt(1)
```

## How it Works

1. For numbers within JavaScript's safe integer range ([-2^53 + 1, 2^53 - 1]):
   - Directly converts to `BigInt()` calls
   - Preserves the original number format

2. For numbers beyond the safe integer range:
   - Decomposes into arithmetic expressions using safe integers
   - Uses base-MAX_SAFE_INTEGER representation
   - Generates optimal expressions to minimize operations
   - Ensures all intermediate values are within safe range

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
           ["swc-plugin-bigint-literal", {}]
         ]
       }
     }
   }
   ```

## Development Requirements

- Rust
- wasm32-wasip1 target
- Node.js >= 14.0.0

## License

MIT
