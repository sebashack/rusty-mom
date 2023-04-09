from datetime import datetime
from flask import Flask, jsonify
import sys
import threading
import time

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber


RETRY_DELAY = 2
MAX_ATTEMPTS = 5

app = Flask(__name__)
message_list = []
message_lock = threading.Lock()


@app.route("/news")
def get_messages():
    messages = app.config["messages_list"]
    return jsonify(messages)


def on_message(message):
    with message_lock:
        message_list.append(message)


def main():
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_info, channel_info = mom_client.create_channel("news-queue", topic="news")

    def consume_news():
        subscriber = Subscriber(mom_info, channel_info)
        subscriber.consume(on_message)

    threading.Thread(target=consume_news).start()

    app.config["messages_list"] = message_list
    app.run()


if __name__ == "__main__":
    main()
