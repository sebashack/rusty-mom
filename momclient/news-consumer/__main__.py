from datetime import datetime
import sys
import argparse
import json
import threading
import time
from flask import Flask, render_template

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber

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


def create_queue_pusher(mom_client, queue_label, retry_delay_secs, max_attempts):
    def retry(i):
        if i >= max_attempts:
            raise Exception("Could not establish connection with MoM Server")
        else:
            print(f"Connecting pusher to queue, attempt {i + 1}")
            try:
                _, info = mom_client.get_queue_info(queue_label)
                return Pusher(info)
            except:
                time.sleep(retry_delay_secs)
                return retry(i + 1)

    return retry(0)


def main(argv):
    parser = argparse.ArgumentParser(description="File service API gateway")
    parser.add_argument("config", type=str, help="Path to config file")
    args = parser.parse_args(argv[1:])

    config_path = args.config

    conf = json.loads(open(config_path, "r").read())

    RETRY_DELAY = conf["retry_delay"]
    MAX_ATTEMPTS = conf["max_attempts"]
    PUSH_DELAY_SECS = conf["push_delay_secs"]

    mom_client = MoMClient(conf["mom_manager_host"], conf["mom_manager_port"])
    mom_client.create_queue("comments-queue")

    def push_news():
        pusher = create_queue_pusher(
            mom_client, "comments-queue", RETRY_DELAY, MAX_ATTEMPTS
        )
        while True:
            now = str(datetime.now())
            msg = f"Comment with time {now}".encode("utf-8")
            try:
                pusher.push(msg, "comments-queue", topic="comments")
            except:
                pusher = create_queue_pusher(
                    mom_client, "comments-queue", RETRY_DELAY, MAX_ATTEMPTS
                )
            time.sleep(PUSH_DELAY_SECS)

    def consume_news():
        consume_with_retry(mom_client, "news-queue", "news", RETRY_DELAY, MAX_ATTEMPTS)

    threading.Thread(target=push_news).start()
    threading.Thread(target=consume_news).start()

    app.config["messages_list"] = message_list
    app.run(host=conf["host"], port=conf["port"])


if __name__ == "__main__":
    main(sys.argv)
