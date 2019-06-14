from ..__main__ import cli
from .parse import search

@cli.command()
@click.pass_context
def embroideres(ctx):
    search(ctx.obj['session'])

if __name__ == "__main__":
    embroideres()
