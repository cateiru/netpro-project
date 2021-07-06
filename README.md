# Repo Sync

from Network Programming Project

## 機能

- ソケット通信によりディレクトリ（or ファイル）をリアルタイムで同期する。
- Gitの仕組みを活用して履歴から変更を適用することが可能。
- 会議などで複数人で資料共有をプライベートネットワーク環境で使用想定。

## 使用言語

- Python
- Rust

## 使い方

### セットアップと実行

以下が必要です。

- python 3.8.x
- cargo latest

```bash
# Rustのビルド
./build.sh

# Pythonの依存関係インストール
pipenv install

# 仮想環境を構築して実行する
pipenv shell
    > repo-sync -a [address] -f [file] -p [port]
    > repo-server -p [port]
    > repo-git show
    > repo-git apply -h [hash] -f [file]
```

### コマンド解説

#### repo-sync

サーバーと接続してファイルを同期します。

- `-a [str]`, `--address=[str]`
  - 接続するサーバーのアドレス
- `-p [int]`, `--port=[int]`
  - 接続のために使用するポート
  - サーバーで指定したポートと同じにする必要があります
- `-f [str]`, `--file=[str]`
  - 同期するファイルのパス
  - 相対or絶対で指定可能

#### repo-server

サーバーをセットアップします。
repo-syncを使用するには必ず1つのサーバーを実行してください。

- `-p [int]`. `--port=[int]`
  - 使用するポート
  - repo-syncでのポートと同じにしてください
- `-a [str]`, `--address [str]`
  - サーバーのアドレス
  - デフォルトはsocket.gethostname

#### repo-git show

変更履歴を表示します。

- `--pager=[bool]`, `-p [bool]`
  - ページャーを使用するか選択します。デフォルトはTrueです。

#### repo-git apply

repo-git showで表示した履歴のハッシュ値を指定してその履歴を適用（revert）します。

- `--hash=[str]`, `-h [str]`
  - 適用するハッシュ値。
- `--file=[str]`, `-f [str]`
  - 履歴を適用するファイルのパス
  - 絶対or相対

## License

[MIT](LICENSE)
