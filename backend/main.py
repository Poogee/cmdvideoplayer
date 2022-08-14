from flask import Flask
from flask import Flask, redirect, url_for, request
from werkzeug.security import generate_password_hash, check_password_hash
from flask import make_response
from flask import request, flash
import requests
from flask_sqlalchemy import SQLAlchemy
#from data import database, mail, secretKey
import re
import cv2
from flask import render_template
from sqlalchemy.orm.attributes import flag_modified
import json
from flask_mail import Mail, Message
from itsdangerous import URLSafeTimedSerializer
from flask_login import login_user, login_required, current_user, LoginManager, logout_user
import base64
from flask import send_file

app = Flask(__name__)


def frames():
    vidcap = cv2.VideoCapture('papichnuts.mp4')
    success, image = vidcap.read()
    count = 0
    while success:
        cv2.imwrite("frame%d.jpg" % count, image)# save frame as JPEG file
        success, image = vidcap.read()
        print('Read a new frame: ', success)
        count += 1

    
@app.route('/', methods=['GET', 'POST'])
def handshake():
    return "HellO!"

@app.route('/gettestframe', methods=['GET', 'POST'])
def gettestframe():
    frames()
    return send_file("mqdefault.jpg")

if __name__ == '__main__':
   app.run()