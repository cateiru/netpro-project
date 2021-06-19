from os import name
from setuptools import find_packages, setup


def read_requirements():
    with open("requirements.txt") as f:
        content = f.read()
        requirements = content.split("\n")

    return requirements

setup(
    name="RepoSync",
    version="1.0.0",
    description="Sync files in private network.",
    license="MIT License",
    url="https://github.com/yuto51942/netpro-project",
    install_requires=read_requirements(),
    packages=find_packages("repo_sync"),
    entry_points={
        'console_scripts': [
            'repo-sync=repo_sync:cli',
        ],
    },
)
