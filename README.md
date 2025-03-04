# SWC Plugin BigInt Literal

这是一个 SWC 编译器插件，用于优化 JavaScript 代码中的 BigInt 字面量。当 BigInt 字面量在安全整数范围内时，插件会将其转换为 `BigInt()` 函数调用的形式。

## 功能特性

- 支持所有数字字面量格式的转换：
  - 十进制 (例如: `123n`)
  - 十六进制 (例如: `0xffn`)
  - 二进制 (例如: `0b1111n`)
  - 八进制 (例如: `0o777n`)
- 保持原始数字表示形式
- 支持负数
- 支持在表达式中的转换

## 转换示例

```javascript
// 十进制
123n          -> BigInt(123)

// 十六进制
0xffn         -> BigInt(0xff)

// 二进制
0b1111n       -> BigInt(0b1111)

// 八进制
0o777n        -> BigInt(0o777)

// 负数
-42n          -> BigInt(-42)
-0xffn        -> BigInt(-0xff)

// 表达式
1n + 2n       -> BigInt(1) + BigInt(2)
```

## 限制说明

1. 安全整数范围
   - 只处理在 JavaScript 安全整数范围内的 BigInt 字面量
   - 范围：[-2^53 + 1, 2^53 - 1]，即 [-9007199254740991, 9007199254740991]
   - 超出此范围的 BigInt 字面量将保持原样不变

2. 不会处理的情况：
   ```javascript
   // 超出安全整数范围的数字保持不变
   9007199254740992n   // 保持原样
   -9007199254740992n  // 保持原样
   
   // 计算结果超出范围的表达式中的数字也会被转换
   // （注意：这可能导致计算结果不一致）
   9007199254740990n + 2n  -> BigInt(9007199254740990) + BigInt(2)
   ```

## 使用方法

1. 安装插件：
   ```bash
   npm install --save-dev swc-plugin-bigint-literal
   ```

2. 配置 `.swcrc`：
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

## 注意事项

1. 性能考虑
   - 对于在安全整数范围内的 BigInt 运算，使用 `BigInt()` 函数可能比字面量形式性能更好
   - 但对于超出范围的数字，保持字面量形式更合适

2. 兼容性
   - 确保你的运行环境支持 BigInt
   - 需要 ES2020 或更高版本

## 开发环境要求

- Rust
- wasm32-wasip1 目标
- Node.js >= 14.0.0

## 许可证

MIT
