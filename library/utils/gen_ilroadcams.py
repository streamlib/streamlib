import re
import sys
import urllib3

CAMS_SEARCH_URL = "https://www.iroads.co.il/umbraco/Surface/CamerasLobby/FilterCameras"


class GenerateILCams():

    def generate(self, output):
        self.get_channels()
        self.save(output)

    def get_channels(self):
        http = urllib3.PoolManager()
        r = http.request("POST", CAMS_SEARCH_URL, fields={
            "tabName": "all-cameras",
            "itemsPerPage": "2000"
        })
        res = r.data.decode("utf-8")
        pattern = r"https://5c328052cb7f5.streamlock.net/live/([A-Z0-9]*).stream/playlist.m3u8"
        matches = re.findall(pattern, res)
        self.channels = matches

    def channel_to_toml(self, name):
        url = f"https://5c328052cb7f5.streamlock.net/live/{name}.stream/playlist.m3u8"
        return f"""[{name.lower()}]
        url = "{url}"
        tags = ["roadcams", "israel", "iroads"]\n
        """.replace("    ", "")

    def save(self, output):
        with open(output, 'w') as f:
            for chan in self.channels:
                toml = self.channel_to_toml(chan)
                f.write(toml)


if __name__ == "__main__":
    somafm = GenerateILCams()
    somafm.generate(sys.argv[1])
