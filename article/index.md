# rustのmoduleと仲良くなる

## はじめに
rustでは、フォルダやファイルが[モジュールとして扱われます](https://doc.rust-jp.rs/book-ja/ch07-05-separating-modules-into-different-files.html)。
rustのモジュールは個人的に癖があるなと思ったので、備忘録を兼ねてまとめておきます。
ステップバイステップで書いていこうと思っています。

リポジトリはこちらです。
https://github.com/TakedaTakumi/rust-module-sample

筆者はTypeScriptをよく使っているので、その辺の用語が混ざっているかもしれませんが、ご容赦ください。

## Step01: モジュールを使わない。
まずは、モジュールを使わないで、コードを書いてみます。
`src/main.rs`に以下のコードを書きます。

```rust
#[derive(Debug)]
struct ID {
    value: String,
}
impl ID {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct Node {
    id: ID,
    label: String,
}
impl Node {
    fn new(id: ID, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
        }
    }
}

fn main() {
    let node = Node::new(ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
```

このコードは、ドメイン駆動設計（以下、DDD）として、`Node`エンティティと、そのID値を表す`ID`値オブジェクトを定義しています。
`Node`エンティティの`label`プロパティが値オブジェクトになっていないのは、ちょっと目をつぶってください。
コードが長くなってしまうので、意図的に手を抜いています。

このコードをもとに、ファイル分割をしていきましょう。

