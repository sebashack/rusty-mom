import json
import requests


class MoMClient:
    _http_host = None
    _http_port = None

    def __init__(self, http_host, http_port):
        self._http_host = http_host
        self._http_port = http_port

    def create_queue(self, queue_label):
        response = requests.post(f"{self.root()}/queues/{queue_label}")
        print(f"Response Status: {response.status_code}")
        print(f"Response body: {response.text}")

    def delete_queue(self, queue_label):
        response = requests.delete(f"{self.root()}/queues/{queue_label}")
        print(f"Response Status: {response.status_code}")
        print(f"Response body: {response.text}")

    def create_channel(self, queue_label, topic="__none__"):
        headers = {"Content-type": "application/json"}
        response = requests.put(
            f"{self.root()}/queues/{queue_label}/channels/{topic}", headers=headers
        )
        data = response.json()

        print(f"Response Status: {response.status_code}")
        print(f"Response body: {data}")

        return data

    def root(self):
        return f"http://{self._http_host}:{self._http_port}"
