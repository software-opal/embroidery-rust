import requests


@click.group()
@click.pass_context
def cli(ctx):
    ctx.ensure_object(dict)
    session = requests.Session()
    ctx.obj["session"] = session


if __name__ == "__main__":
    cli(obj={})
