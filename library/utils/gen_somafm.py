import json
import sys
import urllib3

CHANNELS_JSON = "https://api.somafm.com/channels.json"


class GenerateSomaFM():

    def generate(self, output):
        self.get_channels()
        self.save(output)

    def get_channels(self):
        http = urllib3.PoolManager()
        r = http.request("GET", CHANNELS_JSON)
        self.channels = json.loads(r.data.decode("utf-8"))["channels"]

    def channel_to_toml(self, channel):
        return f"""[{channel["id"]}]
        name = "{channel["title"]}"
        description = "{channel["description"]}"
        url = "{channel["playlists"][0]["url"]}"
        tags = ["radio", "somafm"]\n
        """.replace("    ", "")

    def save(self, output):
        with open(output, 'w') as f:
            for chan in self.channels:
                toml = self.channel_to_toml(chan)
                f.write(toml)


if __name__ == "__main__":
    somafm = GenerateSomaFM()
    somafm.generate(sys.argv[1])
