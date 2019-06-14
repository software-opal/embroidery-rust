from urlnormalizer import normalize_url
from bs4 import BeautifulSoup
from urllib.parse import urljoin


def request(session, url):
    with session.get(url) as resp:
        return resp.url, BeautifulSoup(resp.text, "lxml")


def search(session):
    visited = set()
    to_visit = {"https://forum.embroideres.com/files/"}
    while to_visit:
        url = to_visit.pop()
        if url in visited:
            continue
        visited.add(url)
        base_url, html = request(session, url)
        print(base_url, html)
        for a in html.find_all("a"):
            if a.has_attr("href"):
                target = normalize_url(urljoin(base_url, a['href']))
                print(target)
                if (
                    target.startswith("https://forum.embroideres.com/files/file/")
                    and target not in visited
                ):
                    to_visit.add(target)

if __name__ == "__main__":
    import requests
    search(requests.Session())
