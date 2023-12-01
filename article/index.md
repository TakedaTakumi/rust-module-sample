# rustのmoduleと仲良くなる

## はじめに
rustでは、フォルダやファイルが[モジュールとして扱われます](https://doc.rust-jp.rs/book-ja/ch07-05-separating-modules-into-different-files.html)。
rustのモジュールは、よくわからず色々調べて回ったところなので、備忘録を兼ねてまとめておきます。
ステップバイステップで書いていこうと思っています。

リポジトリはこちらです。
https://github.com/TakedaTakumi/rust-module-sample

筆者はTypeScriptをよく使っているので、その辺の用語が混ざっているかもしれませんが、ご容赦ください。

## Step01: モジュールを使わない。
まずは、モジュールを使わないでコードを書いてみます。
`src/main.rs`に以下のコードを書きます。

```rust
// src/main.rs
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

実行すると、こうなります。
```bash
Hello, module: Node { id: ID { value: "1" }, label: "Node 1" }
```

このコードは、ドメイン駆動設計（DDD）を想定して、ドメイン層として`Node`エンティティと、そのID値を表す`ID`値オブジェクトを定義しています。
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
    ├── id.rs       # 追加
    ├── main.rs
    └── node.rs     # 追加
```

各ファイルの中身はこのような感じです。
構造体や関数を公開するために、`pub`をつけています。
```rust
// src/id.rs
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

`node.rs`では、`id.rs`をインポートしています。
```rust
// src/node.rs
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
2行目は`mod`と同じく、TypeScriptでいうところの`import`である`use`です。
`mod`との違いは、モジュールではなく、構造体をインポートするということです。
意味としては、「`crate`直下の`id`モジュール内の`ID`構造体を使用可能にする」というところでしょうか。


このファイルを`main.rs`からインポートします。

こんな感じ。
```rust
// src/main.rs
mod node;   // node.rsをインポート
mod id;     // id.rsをインポート

fn main() {
    let node = node::Node::new(id::ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
```

2行目と3行目で、それぞれのファイルをインポートしています。
ここでは、`use`の代わりに、`mod`を使った例を示します。
これで`node`モジュールと`id`モジュールが使えるようになりました。
6行目については「`node`モジュール内の`Node`構造体の`new`スタティック関数を実行する～」のような意味になります。

