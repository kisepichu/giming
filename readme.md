# Rust 競プロ環境

## 準備

- wsl vscode rust
  - extension
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    - [LLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
    - [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- cargo-compete とか atcoder-tools とかインストール
  - atcoder-tools は言われた条件に加えて `MarkupSafe` がバグってるので注意 `pip install MarkupSafe==2.0.1` 互換性つけて

```
$ cargo install cargo-compete
$ # エラーに合わせて適当に
$ rustup install 1.42.0-x86_64-unknown-linux-gnu
$ sudo apt install libssl-dev

$ pip install MarkupSafe==2.0.1 # 他のソフトでもっと上使ってるときは注意
$ pip install atcoder-tools
```

- `cargo compete login` とか atcoder-tools 一回使ってログイン
- テンプレート [atcoder-tools/template/template.rs](atcoder-tools/template/template.rs) 書く

## 使用例

wsl
```sh
ac-rs$ source init.sh abc334
```

- 勝手に `compete/abc334` で vscode で開くので解く

ここから vscode のターミナル
```sh
abc334$ source cmd.sh
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
