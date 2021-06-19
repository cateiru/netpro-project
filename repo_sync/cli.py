"""
Cli

Copyright (C) 2021 Netpro Project RepoSync
"""
import ipaddress
import click

import core  # pylint: disable=E1101


class IPAddressType(click.ParamType):
    name = 'ipaddress'

    def convert(self, value, param, ctx):
        try:
            return ipaddress.ip_address(value)
        except ValueError:
            message = '{value} is not a valid ip address'.format(value=value)
            self.fail(message, param, ctx)


IP_ADDRESS = IPAddressType()


@click.command()
@click.option('--address', '-a', type=IP_ADDRESS, multiple=True)
@click.option('--file', '-f', type=click.Path())
def cli(address, file) -> None:
    for ip in address:
        click.echo(ip)
        # click.echo(type(ip))
    try:
        click.echo(file)
        content = open(file)
        click.echo(content.read())
        content.close()
    except FileNotFoundError:
        print('error: Noting file!')
    core.fop("README.md", ".cache")  # pylint: disable=E1101


if __name__ == "__main__":
    cli()
