from flask import Flask
from flask import Flask, redirect, url_for, request
from werkzeug.security import generate_password_hash, check_password_hash
from flask import make_response
from flask import request, flash
import requests
from flask_sqlalchemy import SQLAlchemy
#from data import database, mail, secretKey
import re
from flask import render_template
from sqlalchemy.orm.attributes import flag_modified
import json
from flask_mail import Mail, Message
from itsdangerous import URLSafeTimedSerializer
from flask_login import login_user, login_required, current_user, LoginManager, logout_user
import base64

app = Flask(__name__)


    
@app.route('/', methods=['GET', 'POST'])
def handshake():

    return "HellO!"

if __name__ == '__main__':
   app.run()