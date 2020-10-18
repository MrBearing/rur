# R.U.R

![Build and Test](https://github.com/akeboshi/rur/workflows/Build%20and%20Test/badge.svg)

Rust Universal Robots library

NOT "Rossum's Universal Robots"

## Target control box version

e-Series : v 5.8

## Developing environment

OS: Ubuntu 18.04

e-Series v5.8 [down load here](https://www.universal-robots.com/download/?option=69986#section41511)

## Usage

send some script

```shell
rur send_script -r ur_ip_address test.urscript
rur send_script -p primary -r ur_ip_address test.urscript
rur send_script -p secondary -r ur_ip_address test.urscript
```

Controling Dashboard Server

```shell
$ rur ds load some_internal_script
$ rur ds play
start program...
```

## How to install

```shell
cargo install rur
```
