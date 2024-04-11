# Rust 競プロ環境

## 準備

- wsl vscode rust
  - extension
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    - [LLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
    - [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- cargo-compete とかインストール

```
$ cargo install cargo-compete
$ # エラーに合わせて適当に
$ rustup install 1.42.0-x86_64-unknown-linux-gnu
$ sudo apt install libssl-dev
```

- `cargo compete login` とかログイン

## 使用例

wsl

```sh
ac-rs$ cargo run -- init abc334
```

- 勝手に `compete/abc334` で vscode 開くので解く

ここから vscode のターミナル

```sh
abc334$ cargo run -- terminal
$ sub a # a 問題をテストして提出 勝手に b 問題のソースが開く
$ ts a # テスト
$ run a # 実行
$ o a # ソースを開く
```

[cmd.sh](cmd.sh) 参照

- デバッグは Debug a とかの configuration が勝手にできてるので選んで F5

### memo

- ターミナルから実行したとき、標準入力の最後に EOF (Ctrl+D) を入れないと進まない
- `source cmd.sh` は、このリポジトリ以下を開いたとき勝手に実行されるように .bashrc に書いた

.bashrc

```
# いい書き方わからず
if [ $(pwd | grep -c "$REPOS/ac-rs") -gt 0 ]; then
    source $REPOS/ac-rs/cmd.sh
fi
```

- 初回はビルドに時間がかかるけど target に残ってるから次から速い
