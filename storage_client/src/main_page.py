import streamlit as st
import streamlit_authenticator as stauth
from streamlit_extras.switch_page_button import switch_page

import yaml
import streamlit as st
from yaml.loader import SafeLoader
import streamlit.components.v1 as components

from hasher import Hasher
from authenticate import Authenticate

import yaml
from yaml.loader import SafeLoader
from protos.data_pb2 import MsgMetrics

from network import Network
import time

#z = Network()
#while True:
    #z.check_credentials("user", Hasher(['pwd']).generate()[0])
#    z.send_credentials("user", Hasher(['pwd']).generate()[0], "ale", "pwd", "test@test")
#    z.send_metrics('ed8273cf-af1e-4cc4-9e11-cd96e3a2f514', str(int(time.time())), '10001', '10', '10', '20')
#    z.set_permission('user', '351a5a51-b069-4b6e-8960-46f4418809a0', 'Y', 'Y')
    #z.send_liveness('5792879c-93ee-4f87-8735-c7ea56595168')
    #z.delete_file('eecf3670-2324-4bae-a588-83698ec4547c', 'user')
    #time.sleep(10)

def page1(authenticator):
    authenticator.login('Login', 'main')
    if st.session_state["authentication_status"]:
        switch_page("user page")
    elif st.session_state["authentication_status"] is False:
        st.error('Username/password is incorrect')
    elif st.session_state["authentication_status"] is None:
        st.warning('Please enter your username and password')

    if st.toggle('Reset Password'):
        try:
            if authenticator.reset_password(st.session_state["username"], 'Reset password'):
                st.success('Password modified successfully')
        except Exception as e:
            st.error(e)
   
    if "Register User" not in st.session_state:
        st.session_state["Register User"] = False

    if st.button("Register User"):
        st.session_state["Register User"] = not st.session_state["Register User"]

    if st.session_state["Register User"]:
        try:
            if authenticator.register_user('Register user', preauthorization=False):
                st.success('User registered successfully')
        except Exception as e:
            st.error(e)

st.set_page_config(layout="centered")

#with open('config.yaml') as file:
#    config = yaml.load(file, Loader=SafeLoader)

authenticator = Authenticate(
    cookie_name='random_cookie_name',
    key='random_signature_key',
    cookie_expiry_days=30
#    config['credentials'],
#    config['cookie']['name'],
#    config['cookie']['key'],
#    config['cookie']['expiry_days'],
#    config['preauthorized'],
)
st.session_state.authenticator = authenticator
page1(authenticator)
