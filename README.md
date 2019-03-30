atcoder_test

RustでAtCoderの問題を解く際に、コンパイルとサンプルの入出力が正しいかのチェックを行います。

install

```bash
$ git clone path-to-this-repository atcoder_test
$ cd atcoder_test && cargo install --path .
```

uninstall

```bash
$ cargo uninstall atcoder_test
```

usage

まず、以下のようなフォルダ構成にする必要があります。

```
abcxxx
|--Cargo.lock
|--Cargo.toml
|--src
|  |--bin
|  |  |--a.rs
|  |  |--b.rs
|  |  |--c.rs
|  |  |--d.rs
```

例として、ABC121のA問題のテストを行う際には以下のようにします。

```bash
$ atcoder_test abc121 a
```

コンテスト名の"abc121"の部分はと問題名"a"の部分は、問題のURLの生成にも使用します。
コンテスト用のプログラムはdebugモードでコンパイルと実行を行います。

license

The MIT License
