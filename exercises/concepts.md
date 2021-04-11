# Rustの概念

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [Q1](#q1)
- [Q2](#q2)
- [QN](#qn)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Q1

32ビットの非負数値で名称が `MAX_POINTS` であり、値が100000である定数を定義してみましょう。

<details>
<summary>回答例</summary>

Rustでは定数の命名規則として、全て大文字でアンダースコアで単語区切りする必要がある。

```rust
const MAX_POINTS: u32 = 100_000;
```

</details>

## Q2

以下の2つのコードの違いは何でしょうか。

コード1

```rust
let spaces = "    ";
let spaces = spaces.len();
```

コード2

```rust
let mut spaces = "    ";
spaces = spaces.len();
```

<details>
<summary>回答例</summary>

コード1では、不変の文字列型変数として `spaces` を定義している。

そのあとで文字列長を計算し、再度不変の数値型の変数 `spaces` として、新しい変数を生成している。

この方法では、同じ変数に対して異なる値を割り当てることが可能になる。

```rust
let spaces = "    ";
let spaces = spaces.len();
```

コード2では、可変の文字列型として `spaces` を定義している。

そのあとで文字列長を数値型として再代入しようとしているが、`spaces` の型は文字列型のままなので、型の不一致が発生してしまい、下記コードはコンパイルすることはできない。

```rust
let mut spaces = "    ";
spaces = spaces.len();
```

</details>

## Q3

Rustで使用できる組み込みの整数型はどのようなものがあるでしょうか。

<details>
<summary>回答例</summary>

整数はビット数と符号のあるなしで分類できる。

|大きさ|符号付き|符号なし|
|:--|:--|:--|
|8ビット|`i8`|`u8`|
|16ビット|`i16`|`u16`|
|32ビット|`i32`|`u32`|
|64ビット|`i64`|`u64`|
|アーキテクチャ依存|`isize`|`usize`|

デフォルトでは `i32` が使用される。

</details>

## Q4

Rustで使用できる組み込みの浮動小数点型はどのようなものがあるでしょうか。

<details>
<summary>回答例</summary>
</details>

- `f32`
  - 32ビットの浮動小数点。単精度浮動小数点数
- `f64`
  - 64ビットの浮動小数点。倍精度浮動小数点数

デフォルトでは `f64` が使用される。

```rust
fn main {
    // f64
    let x = 2.0;

    let y: f32 = 3.0;
}
```

## Q5

Rustで論理値型を定義するにはどうすればいいでしょうか。

<details>
<summary>回答例</summary>

Rustでは論理値型 (`bool`) として `true` と `false` を使用することができる。

```rust
fn main() {
    let t = true;

    let f: bool = false;
}
```

</details>

## QN

<details>
<summary>回答例</summary>
</details>