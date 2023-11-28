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

このコードは、ドメイン駆動設計（以下、DDD）を想定して、ドメイン層として`Node`エンティティと、そのID値を表す`ID`値オブジェクトを定義しています。
`Node`エンティティの`label`プロパティが値オブジェクトになっていないのは、ちょっと目をつぶってください。
コードが長くなってしまうので、意図的に手を抜いています。

このコードをもとに、ファイル分割をしていきましょう。

## Step02: ファイル分割
NodeやIDをそれぞれ別ファイルにしてみましょう。

こんな風に分けて見ました。
```
.
├── Cargo.toml
└── src
    ├── id.rs
    ├── main.rs
    └── node.rs
```

各ファイルの中身はこのような感じです。
構造体や関数を公開するために、`pub`をつけています。
```rust:src/id.rs
#[derive(Debug)]
pub struct ID {
    value: String,
}
impl ID {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
```

`node.rs`からは、`id.rs`をインポートしています。
```rust:src/node.rs
use crate::id::ID;

#[derive(Debug)]
pub struct Node {
    id: ID,
    label: String,
}
impl Node {
    pub fn new(id: ID, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
        }
    }
}
```


このファイルを`main.rs`からインポートします。

こんな感じ。
```rust:src/main.rs
mod node;   // node.rsをインポート
mod id;     // id.rsをインポート

fn main() {
    let node = node::Node::new(id::ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
```

1行目と2行目で、それぞれのファイルをインポートしています。
これで`node`モジュールと`id`モジュールが使えるようになりました。
5行目については、`node`モジュール内の`Node`構造体の`new`スタティック関数を実行する～のような意味になります。
