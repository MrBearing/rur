# R.U.R

(現在開発中,以下は妄想です。)

ユニバーサルロボット社のロボットCBシリーズ , eシリーズのロボットをRustからコントロールするためのソフトウェア及び・Rust向けのライブラリです。

URScriptをロボットへ直接送信したり、Dashboard ServerにPlay,Load命令を実行させることが可能です。

## 使い方

スクリプトを送信する

```shell
rur send_script ur_ip_address test.urscript
rur send_script -p primary ur_ip_address test.urscript
rur send_script -p secondary ur_ip_address test.urscript
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
