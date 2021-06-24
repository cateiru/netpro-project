from setuptools import find_packages, setup


setup(
    name="RepoSync",
    version="1.0.0",
    description="Sync files in private network.",
    license="MIT License",
    url="https://github.com/yuto51942/netpro-project",
    install_requires=["click"],
    packages=find_packages("repo_sync"),
    entry_points={
        'console_scripts': [
            'repo-sync=repo_sync.cli:sync_cli',
            'repo-git=repo_sync.cli:git_cli',
        ],
    },
)
