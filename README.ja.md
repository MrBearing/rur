# R.U.R

Rust Universal Robots library

NOT "Rossum's Universal Robots"

## コントロールボックスの対象バージョン

e-Series : v 5.8

## 開発環境

OS: Ubuntu 18.04
e-Series v5.8



(現在開発中,以下は妄想です。)

ユニバーサルロボット社のロボットCBシリーズ , eシリーズのロボットをRustからコントロールするためのソフトウェア及び・Rust向けのライブラリです。

URScriptをロボットへ直接送信したり、Dashboard ServerにPlay,Load命令を実行させることが可能です。

## 使い方

スクリプトを送信する

```shell
rur send_script -r ur_ip_address test.urscript
rur send_script -p primary -r ur_ip_address test.urscript
rur send_script -p secondary -r ur_ip_address test.urscript
```

Dashboard Serverの操作系統

```shell
$ rur ds load some_internal_script
$ rur ds play
start program...
```

## インストール方法

cargoを使用してインストールします。以下のコマンドでインストールしてください。

```shell
cargo install rur
```
