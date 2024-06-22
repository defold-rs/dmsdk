import io
import os
import requests
import shutil
import sys
from pathlib import Path
from zipfile import ZipFile

def main(version: str):
    crate_dir = Path(os.path.dirname(os.path.realpath(__file__)))
    headers_dir = crate_dir / "dmsdk"
    shutil.rmtree(headers_dir)

    sdk_url = f"https://github.com/defold/defold/releases/download/{version}/defoldsdk_headers.zip"

    print(f"Downloading {sdk_url}...")

    response = requests.get(sdk_url)
    with ZipFile(io.BytesIO(response.content), "r") as zip:
        for file_name in zip.namelist():
            if file_name.startswith("defoldsdk/sdk/include"):
                contents = zip.read(file_name)
                path = headers_dir / file_name.removeprefix("defoldsdk/sdk/include/dmsdk/")
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(contents)
                print(path)


if __name__ == "__main__":
    main(sys.argv[1])