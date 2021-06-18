# Repo Sync

from Network Programming Project

## Feature

- ソケット通信によりディレクトリ（or ファイル）をリアルタイムで同期する。
- Gitの仕組みを活用して履歴を遡る、restore、revertなどが可能。
- 会議などで複数人で資料共有をプライベートネットワーク環境で使用想定

## Use Languages

- Python
- Rust

## Create environments

```bash
pip install pipenv tox

# rust build and compile
./build

# 仮想環境で実行
pipenv install
pipenv shell
> repo-sync

# メイン環境で実行
pipenv install --system --deploy
repo-sync

# テスト
tox
```

## License

[Mit](LICENSE)
