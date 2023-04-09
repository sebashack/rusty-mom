from datetime import datetime
from flask import Flask, jsonify, render_template
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
    return render_template("news.html", messages=messages)


def on_message(message):
    with message_lock:
        if message["id"] not in map(lambda m: m["id"], message_list):
            message_list.append(
                {"id": message["id"], "content": message["content"].decode("UTF-8")}
            )


def consume_with_retry(mom_client, queue_label, topic, retry_delay_secs, max_attempts):
    def retry(i):
        k = i
        if i > max_attempts:
            raise Exception("Could not establish connection with MoM Server")
        else:
            print(f"Connecting consumer, attempt {i + 1}")
            try:
                mom_info, channel_info = mom_client.create_channel(queue_label, topic)
                subscriber = Subscriber(mom_info, channel_info)
                k = -1
                subscriber.consume(on_message)
            except Exception as e:
                time.sleep(retry_delay_secs)
                return retry(k + 1)

    return retry(0)


def main():
    mom_client = MoMClient("127.0.0.1", 8082)

    def consume_news():
        consume_with_retry(mom_client, "news-queue", "news", RETRY_DELAY, MAX_ATTEMPTS)

    threading.Thread(target=consume_news).start()

    app.config["messages_list"] = message_list
    app.run()


if __name__ == "__main__":
    main()
