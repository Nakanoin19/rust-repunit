# Repunit
<table>
<thead>
<tr>
<th style="text-align:center">
<a href="README_ja.md">日本語</a>
</th>
<th style="text-align:center">
<a href="README.md">English</a>
</th>
</tr>
</thead>
</table>

**English version is [here](./README.md).**

レピュニットを生成するライブラリ

## インストール方法

このクレートはライブラリであるため、`cargo install`でインストールすることはできません。

あなたのアプリケーションにこのライブラリを使いたいときは`Cargo.toml`にこの文字列を追加してください。

```rust
[dependencies]
repunit
```

## 使い方

### 最初に
```rust
extern crate repunit;
```

### レピュニットの桁数からレピュニットへ変換する
```rust
repunit::convert(桁数)
```
### レピュニットからレピュニットの桁数へ復元する
```rust
repunit::restore(レピュニット)
```
### 例
```rust
extern crate repunit;
fn main() {
    println!("{}",repunit::convert(repunit::restore(111111)));
}

// => 111111
```
```rust
extern crate repunit;
fn main() {
    println!("{}", 3 * repunit::convert(4));
}

// => 3333
```

## 開発者向け

このライブラリをローカルマシンにインストールするには、`cargo install --path .` を実行してください。新しいバージョンをリリースするには、`Cargo.toml` のバージョン番号を更新してから `cargo build --release` を実行します。

## 貢献

バグ報告やプルリクはGitHubで受け付けています。 [https://github.com/NSK-1010/ruby-repunit](https://github.com/NSK-1010/ruby-repunit)

## ライセンス

このgemは、[MIT License](https://opensource.org/licenses/MIT)の条件の下でオープンソースとして利用可能です。