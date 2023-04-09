from datetime import datetime
import sys
import threading
import time

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber

RETRY_DELAY_SECS = 2
MAX_ATTEMPTS = 5
PUSH_DELAY_SECS = 5


def create_queue_pusher(mom_client, queue_label, retry_delay_secs, max_attempts):
    def retry(i):
        print(f"Restablishing queue, attempt {i + 1}")
        if i >= max_attempts:
            raise Exception("Could not establish connection with MoM Server")
        else:
            try:
                _, info = mom_client.get_queue_info(queue_label)
                return Pusher(info)
            except:
                time.sleep(retry_delay_secs)
                return retry(i + 1)

    return retry(0)


def main():
    mom_client = MoMClient("127.0.0.1", 8082)
    mom_client.create_queue("news-queue")

    _, mom_info = mom_client.get_queue_info("news-queue")

    def push_news():
        pusher = pusher = create_queue_pusher(
            mom_client, "news-queue", RETRY_DELAY_SECS, MAX_ATTEMPTS
        )
        while True:
            now = str(datetime.now())
            msg = f"News with time {now}".encode("utf-8")
            try:
                pusher.push(msg, "news-queue", topic="news")
            except:
                pusher = create_queue_pusher(
                    mom_client, "news-queue", RETRY_DELAY_SECS, MAX_ATTEMPTS
                )
            time.sleep(PUSH_DELAY_SECS)

    threading.Thread(target=push_news).start()

    while True:
        time.sleep(10)


if __name__ == "__main__":
    main()
