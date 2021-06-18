"""
Cli

Copyright (C) 2021 Netpro Project RepoSync
"""
import core  # pylint: disable=E1101


def cli() -> None:
    """
    cli
    """
    core.fop("README.md", ".cache")  # pylint: disable=E1101


if __name__ == "__main__":
    cli()
