from urlnormalizer import normalize_url
from bs4 import BeautifulSoup
from urllib.parse import urljoin
import pathlib
import cgi
import json


def request(session, url):
    with session.get(url) as resp:
        return resp.url, parse_html(resp.text)


def parse_html(content):
    return BeautifulSoup(content, "lxml")


def do_download(target_folder, resp):
    print("DO DOWN", resp.url)
    disp_type, params = cgi.parse_header(resp.headers.get("Content-Disposition"))
    if disp_type not in ["download", "attachment"]:
        print("Dist type mismatch", disp_type)
        return
    filename = params["filename"]
    target = (target_folder / filename).resolve()
    if target.parent != target_folder:
        print("resolve mismatch", disp_type)
        return
    with target.open("wb") as out:
        for chunk in resp.iter_content(None):
            out.write(chunk)


def try_download(session, url, can_recurse=True):
    print("TRY DOWN", url)
    a, _, b = url.rpartition("?")
    clean_url = a or b
    _, _, name = clean_url.strip("/").rpartition("/")
    if name == clean_url:
        return
    p = (pathlib.Path("downloads") / name).resolve()
    p.mkdir(parents=True, exist_ok=True)
    (p / 'source.txt').write_text(url)
    with session.get(url, stream=True) as resp:
        print(resp.headers)
        if "download" in resp.headers.get("Content-Disposition", ""):
            do_download(p, resp)
            return
        if not can_recurse:
            return
        html = parse_html(resp.text)
    for a in html.find_all("a"):
        if (
            a.has_attr("href")
            and a.has_attr("data-action")
            and a.attrs["data-action"] == "download"
        ):
            link = normalize_url(urljoin(resp.url, a["href"]))
            print("2nd DOWN", link)
            with session.get(link, stream=True) as resp:
                do_download(p, resp)


def search(session):
    initial_url = "https://forum.embroideres.com/files/"
    with session.cache_disabled():
        base_url, html = request(session, initial_url)
        if not html.find(id='elUserLink'):
            print("The session isn't valid.")
            print(f"Please log in here: {initial_url}")
            print("Then update the session information")
            return

    visited = set()
    to_visit = {initial_url}
    while to_visit:
        url = to_visit.pop()
        if url in visited:
            continue
        visited.add(url)
        base_url, html = request(session, url)
        if not html.find(id='elUserLink'):
            print("The session isn't valid. ")

        for a in html.find_all("a"):
            if a.has_attr("href"):
                target = normalize_url(urljoin(base_url, a["href"]))
                a, _, b = target.rpartition("?")
                clean_url = a or b
                if (
                    clean_url.startswith("https://forum.embroideres.com/files/file/")
                    and clean_url not in visited
                ):
                    to_visit.add(clean_url)
                if "do=download" in target:
                    try_download(session, target)
                    print("\t", target)


if __name__ == "__main__":
    import requests
    import requests_cache

    requests_cache.install_cache("demo_cache")

    session = requests.Session()
    session.cookies.update(
        json.loads(pathlib.Path('session.json').read_text())
    )

    search(session)
