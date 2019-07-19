from urlnormalizer import normalize_url
from bs4 import BeautifulSoup
from urllib.parse import urljoin
import pathlib
import cgi


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
    with session.get(url, stream=True) as resp:
        print(resp.headers)
        if "download" in resp.headers.get("Content-Disposition", ""):
            return do_download(p, resp)
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
    visited = set()
    to_visit = {"https://forum.embroideres.com/files/"}
    while to_visit:
        url = to_visit.pop()
        if url in visited:
            continue
        visited.add(url)
        base_url, html = request(session, url)
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
        {
            "__cfduid": "dd9f8d78e90c441b6384d384e4104a3901560475637",
            "ips4_device_key": "3b67d27fba133dd33ed3ceab568b2bd5",
            "ips4_guestTime": "1560475664",
            "ips4_hasJS": "true",
            "ips4_IPSSessionFront": "eef480c41312c5e067506050f1739b37",
            "ips4_ipsTimezone": "Pacific/Auckland",
            "ips4_loggedIn": "1",
            "ips4_login_key": "c07b0cde446019ae6646e1d9b410ed0b",
            "ips4_member_id": "88211",
        }
    )

    search(session)
