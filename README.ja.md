# R.U.R

![Build and Test](https://github.com/akeboshi/rur/workflows/Build%20and%20Test/badge.svg)

Rust Universal Robots library

NOT "Rossum's Universal Robots"


ユニバーサルロボット社のロボットCBシリーズ , eシリーズのロボットをRustからコントロールするためのソフトウェア及び・Rust向けのライブラリです。

URScriptをロボットへ直接送信したり、Dashboard ServerにPlay,Load命令を実行させることが可能です。

## 使い方

スクリプトを送信する

```shell
rur send-script -r ur_ip_address test.urscript
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

--------

## コントロールボックスの対象バージョン

e-Series : v 5.8

## 開発環境

OS: Ubuntu 18.04
e-Series v5.8

## 開発環境構築

公式にLinuxでの開発方法についてのドキュメントは存在するが対応バージョンがUbuntu 14.04
現行の18.04のUbuntuだと起動に成功しなかったので、URSimをVirtualBox上に構築する方法を用いた

### VirtualBoxのインストール

他のサイト等におまかせ

### URSimイメージのダウンロード

以下のリンクから必要なイメージをダウンロード

- [CBシリーズ](https://www.universal-robots.com/download/?option=69992#section16597)
- [CBシリーズイメージDL](https://s3-eu-west-1.amazonaws.com/ur-support-site/69993/URSim_VIRTUAL-3.13.0.10253.rar)
- [eシリーズ](https://www.universal-robots.com/download/?option=69988#section41570)
- [eシリーズイメージDL](https://s3-eu-west-1.amazonaws.com/ur-support-site/69989/URSim_VIRTUAL-5.8.0.10253.rar)

### URSimの解凍

URSimはrarで圧縮されているのでunrarを入れる必要がある

```bash
sudo apt install unrar
unrar x URSim_VIRTUAL-x.xxx.rar
```

### VirtualBoxへの追加

[仮想マシン]->[追加]で解凍した.vbox拡張子のファイルを選択する

### ポートフォワーディングの設定

[設定] -> ネットワーク -> [高度]

ポートフォワーディング
    29999 と 30001 ~ 30004までをすべて同ポートにフォワーディングする
必要であれば 30011 ~ 30014も同様に

### URSimの起動

デスクトップ上のアイコンをダブルクリックすれば起動します。
