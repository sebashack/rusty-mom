from datetime import datetime
import sys
import time
import threading

from flask import Flask, jsonify

sys.path.insert(1, "lib")

from momlib import MoMClient, Pusher, Subscriber


app = Flask(__name__)
app.config["messages_list"] = []
mutex = threading.Lock()


@app.route("/messages")
def get_messages():
    with mutex:
        messages = app.config["messages_list"]
        app.config["messages_list"] = []  # limpia la lista de mensajes para la próxima solicitud
    return jsonify(messages)


if __name__ == "__main__":
    app.run()
