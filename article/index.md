# rustのmoduleと仲良くなる

## はじめに
rustでは、ディレクトリやファイルが[モジュールとして扱われます](https://doc.rust-jp.rs/book-ja/ch07-05-separating-modules-into-different-files.html)。
rustのモジュールは、よくわからず色々調べて回ったところなので、備忘録を兼ねてまとめておきます。
ステップバイステップで書いていこうと思っています。
誰かの役に立てたら嬉しいな。

リポジトリはこちらです。
https://github.com/TakedaTakumi/rust-module-sample

筆者はTypeScriptをよく使っているので、その辺の用語が混ざっているかもしれませんが、ご容赦ください。

## Step01: モジュールを使わない。
まずは、モジュールを使わないでコードを書いてみます。
最初の構成はこんな感じです。
```
.
├── Cargo.toml
└── src
    └── main.rs
```

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
ファイル名がモジュール名になるので、このようになります。

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

## Step03: ディレクトリ分割
次に、ディレクトリ分割をしてみましょう。
今のままでは、エンティティや値オブジェクトが増えるたびに管理が難しくなるので、種類ごとにディレクトリ分けしましょう。

こんな構成にしてみました。
```
.
├── Cargo.toml
└── src
    ├── domain              # 追加
    │   ├── entity          # 追加
    │   │   └── node.rs     ## 移動
    │   └── value_object    # 追加
    │       └── id.rs       ## 移動
    └── main.rs
```

しかし、このままではnodeやidのモジュール（rsファイル）を認識してくれません。
rustでは、ディレクトリもモジュールになるわけですが、モジュールとして認識させるためには、ディレクトリと同名のrsファイルが必要になります。

こうなります。
```
.
├── Cargo.toml
└── src
    ├── domain
    │   ├── entity
    │   │   └── node.rs
    │   ├── entity.rs           # 追加
    │   ├── value_object
    │   │   └── id.rs
    │   └── value_object.rs     # 追加
    ├── domain.rs               # 追加
    └── main.rs
```

それぞれのファイルでは、下位のモジュールをインポートしてエクスポートするように記載します。

TypeScriptでいう、`export AAA from 'XXX'`のようなイメージです。
`pub mod XXX`と書きます。

```rust
// src/domain/value_object.rs
pub mod id;
```
```rust
// src/domain/entity.rs
pub mod node;
```
```rust
// src/domain.rs
pub mod entity;
pub mod value_object;
```

`node.rs`もインポートしているIDの場所が変わったので、修正します。
増えたディレクトリ分だけモジュールを追加しましょう。
```rust
// src/domain/entity/node.rs
use crate::domain::value_object::id::ID;    // ここを修正

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

この調子で`main.rs`も修正しましょう。

```rust
// src/main.rs
mod domain;
use domain::{entity::node::Node, value_object::id::ID};

fn main() {
    let node = Node::new(ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
```

構造体を使用するたびにモジュールまで書いているとやってられないので、`use`を使ってインポートしています。
モジュールに共通部分がある場合、このような書き方もできます。

さて、ファイル分割と、ディレクトリ分割が上手くいったので、ここで終わらせてしまってもいいんですが、上のコード、ちょっと気になりませんか？
私はとっても気になります！

この部分です。
```rust
use domain::{entity::node::Node, value_object::id::ID};
```
`domain`や`entity`, `value_object`まではいいんですが、`node`や`id`って、なんか冗長だなぁと思うんですよ。
いらんだろ、と。

[公式（の非公式和訳）](https://doc.rust-jp.rs/book-ja/ch07-05-separating-modules-into-different-files.html)には詳しく書いてなかったんですが、方法があります！  
（あとで読み返して気付いたけど、たぶん最後の「まとめ」のところに書いてあることが該当するのかな、と思ってる。読解力……！）

次のステップでは、モジュール構成をスッキリさせましょう。

## Step04: モジュール構成をスッキリさせる

`entity.rs`などで使用している`pub mod XXX`という書き方は、XXXというモジュールをインポートして公開するという意味でした。
しかし、今回は`node`などのモジュールは公開したくありません。
では、どうすればいいのか。
`pub use XXX`を使いましょう。

ディレクトリ構成はStep03のままで、各ファイルを修正します。

こんな風に修正しましょう。

```rust
// src/domain/entity.rs
mod node;

pub use node::Node;
```
こうするとこで、`node`モジュールは非公開にしつつ、`Node`構造体を公開することができます。
`value_object.rs`も同様に修正します。

```rust
// src/domain/value_object.rs
mod id;

pub use id::ID;
```

これだけで、インポートしているコードはこうなります。
```rust
// src/domain/entity/node.rs
use crate::domain::value_object::ID;

#[derive(Debug)]
pub struct Node {
    #[allow(dead_code)]
    id: ID,
    #[allow(dead_code)]
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
```rust
mod domain;
use domain::{entity::Node, value_object::ID};

fn main() {
    let node = Node::new(ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
```

不要なモジュールが消えて`use`の部分がスッキリしましたね！
```rust
use domain::{entity::Node, value_object::ID};
```

これで、ディレクトリやファイル分割は自由にできるようになったかと思います。



