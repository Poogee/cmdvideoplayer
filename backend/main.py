from flask import Flask
from flask import Flask, redirect, url_for, request
from werkzeug.security import generate_password_hash, check_password_hash
from flask import make_response
from flask import request, flash
import requests
from flask_sqlalchemy import SQLAlchemy
#from data import database, mail, secretKey
import re
from os.path import exists
import cv2
from flask import render_template
from sqlalchemy.orm.attributes import flag_modified
import json
from flask_mail import Mail, Message
from itsdangerous import URLSafeTimedSerializer
from flask_login import login_user, login_required, current_user, LoginManager, logout_user
import base64
from flask import jsonify
from flask import send_file

app = Flask(__name__)


videoframes = {}
videos = ["papichnuts.mp4"]
curframeno = 0

def frames(videono):
    global videos
    global curframeno
    vidcap = cv2.VideoCapture(videos[videono])
    success, image = vidcap.read()
    count = 0
    while curframeno != count:
        cv2.imwrite("frame%d.jpg" % count, image)# save frame as JPEG file
        success, image = vidcap.read()
        print('Read a new frame: ', success)
        count += 1
        curframeno += 1
    
@app.route('/', methods=['GET', 'POST'])
def handshake():
    return "HellO!"

@app.route('/getframecount', methods=['GET', 'POST'])
def getframecount():
    request.get_json(force=True)
    return jsonify({"framecount" : len(videoframes[request.json['videono']])})

@app.route('/gettestframe', methods=['GET', 'POST'])
def gettestframe():
    frames()
    return send_file("mqdefault.jpg")

@app.route('/getframe', methods=['GET', 'POST'])
def getframe():
    request.get_json(force = True)
    print(request.json)
    return send_file(videoframes[request.json["videono"]][request.json["frameno"]])

#@app.route('/getvideoframe', methods=['GET', 'POST'])
#def getframecount():
#    request.get_json(force=True)
#    return videoframes[int(request.json["videono"])]

def framecheck():
    global videos
    global videoframes
    for i in range(len(videos)):
        cap = cv2.VideoCapture(videos[i])
        length = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
        videoframes[i] = []
        if not exists("%dframe1.jpg" % i):
            success, image = cap.read()
            count = 0
            while success:
                cv2.imwrite("%(videono)dframe%(frameno)d.jpg" % {"videono": i, "frameno" : count}, image) # save frame as JPEG file
                videoframes[i].append("%(videono)dframe%(frameno)d.jpg" % {"videono": i, "frameno" : count})
                success, image = cap.read()
                print('Read a new frame: ', success)
                count += 1
        else:
            for j in range(length):
                videoframes[i].append("%(videono)dframe%(frameno)d.jpg" % {"videono": i, "frameno": j})


if __name__ == '__main__':
    framecheck()
    #print(videoframes)
    app.run()