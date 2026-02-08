import jwt
import bcrypt
import streamlit as st
from datetime import datetime, timedelta
import extra_streamlit_components as stx

from hasher import Hasher
from network import Network
from validator import Validator
from utils import generate_random_pw
import pandas as pd
import constants as c

import zlib

from exceptions import CredentialsError, ForgotError, RegisterError, ResetError, UpdateError
from utils import get_device_name

class Authenticate:
    """
    This class will create login, logout, register user, reset password, forgot password, 
    forgot username, and modify user details widgets.
    """
    def __init__(self, #credentials: dict, 
                 cookie_name: str, key: str, cookie_expiry_days: float=30.0, 
        preauthorized: list=None, validator: Validator=None, network: Network=None):
        """
        Create a new instance of "Authenticate".

        Parameters
        ----------
        credentials: dict
            The dictionary of usernames, names, passwords, and emails.
        cookie_name: str
            The name of the JWT cookie stored on the client's browser for passwordless reauthentication.
        key: str
            The key to be used for hashing the signature of the JWT cookie.
        cookie_expiry_days: float
            The number of days before the cookie expires on the client's browser.
        preauthorized: list
            The list of emails of unregistered users authorized to register.
        validator: Validator
            A Validator object that checks the validity of the username, name, and email fields.
        """
        #self.credentials = credentials
        #self.credentials['usernames'] = {key.lower(): value for key, value in credentials['usernames'].items()}
        self.cookie_name = cookie_name
        self.key = key
        self.cookie_expiry_days = cookie_expiry_days
        self.preauthorized = preauthorized
        self.cookie_manager = stx.CookieManager()
        self.validator = validator if validator is not None else Validator()
        self.network = self._init_network()

        if 'name' not in st.session_state:
            st.session_state['name'] = None
        if 'authentication_status' not in st.session_state:
            st.session_state['authentication_status'] = None
        if 'username' not in st.session_state:
            st.session_state['username'] = None
        if 'logout' not in st.session_state:
            st.session_state['logout'] = None

    @st.cache_resource
    def _init_network(_arg):
        return Network()
    
    @st.cache_resource
    def _get_devices(_self, username):
        return _self.network.get_devices(username)
    
    def _get_metrics(_self, devices):
        return _self.network.get_metrics(devices)
    
    def _get_chunk(_self, devices):
        if devices:
            return _self.network.get_chunk(devices)
    
    def _get_file_list(_self, username):
        return _self.network.get_file_list(username)
    
    def _get_file(_self, file_id, username):
        return _self.network.get_file(file_id, username)

    def _token_encode(self) -> str:
        """
        Encodes the contents of the reauthentication cookie.

        Returns
        -------
        str
            The JWT cookie for passwordless reauthentication.
        """
        return jwt.encode({'name':st.session_state['name'],
            'username':st.session_state['username'],
            'exp_date':self.exp_date}, self.key, algorithm='HS256')

    def _token_decode(self) -> str:
        """
        Decodes the contents of the reauthentication cookie.

        Returns
        -------
        str
            The decoded JWT cookie for passwordless reauthentication.
        """
        try:
            return jwt.decode(self.token, self.key, algorithms=['HS256'])
        except:
            return False

    def _set_exp_date(self) -> str:
        """
        Creates the reauthentication cookie's expiry date.

        Returns
        -------
        str
            The JWT cookie's expiry timestamp in Unix epoch.
        """
        return (datetime.utcnow() + timedelta(days=self.cookie_expiry_days)).timestamp()

#    def _check_pw(self) -> bool:
        """
        Checks the validity of the entered password.

        Returns
        -------
        bool
            The validity of the entered password by comparing it to the hashed password on disk.
        """
#        return bcrypt.checkpw(self.password.encode(), 
#            self.credentials['usernames'][self.username]['password'].encode())

    def _check_cookie(self):
        """
        Checks the validity of the reauthentication cookie.
        """
        self.token = self.cookie_manager.get(self.cookie_name)
        if self.token is not None:
            self.token = self._token_decode()
            if self.token is not False:
                if not st.session_state['logout']:
                    if self.token['exp_date'] > datetime.utcnow().timestamp():
                        if 'name' and 'username' in self.token:
                            st.session_state['name'] = self.token['name']
                            st.session_state['username'] = self.token['username']
                            st.session_state['authentication_status'] = True
    
    def _check_credentials(self, username: str, password: str, inplace: bool=True) -> bool:
        """
        Checks the validity of the entered credentials.

        Parameters
        ----------
        inplace: bool
            Inplace setting, True: authentication status will be stored in session state, 
            False: authentication status will be returned as bool.
        Returns
        -------
        bool
            Validity of entered credentials.
        """
        pwd, salt = self.network.check_credentials(username)
        if pwd and salt:
            print("Retrieved pwd: " + str(pwd))
            inserted_password = bcrypt.hashpw(password.encode('utf-8'), salt.encode('utf-8'))
            if inserted_password==pwd.encode('utf-8'):
                st.session_state['authentication_status'] = True
                return True
            else:
                print("Wrong password.")
        else:
            print("User not found")
        st.session_state['authentication_status'] = False
        return False

        """
        if self.username in self.credentials['usernames']:
            try:
                if self._check_pw():
                    if inplace:
                        st.session_state['name'] = self.credentials['usernames'][self.username]['name']
                        self.exp_date = self._set_exp_date()
                        self.token = self._token_encode()
                        self.cookie_manager.set(self.cookie_name, self.token,
                            expires_at=datetime.now() + timedelta(days=self.cookie_expiry_days))
                        st.session_state['authentication_status'] = True
                    else:
                        return True
                else:
                    if inplace:
                        st.session_state['authentication_status'] = False
                    else:
                        return False
            except Exception as e:
                print(e)
        else:
            if inplace:
                st.session_state['authentication_status'] = False
            else:
                return False
        """
                
    def login(self, form_name: str, location: str='main') -> tuple:
        """
        Creates a login widget.

        Parameters
        ----------
        form_name: str
            The rendered name of the login form.
        location: str
            The location of the login form i.e. main or sidebar.
        Returns
        -------
        str
            Name of the authenticated user.
        bool
            The status of authentication, None: no credentials entered, 
            False: incorrect credentials, True: correct credentials.
        str
            Username of the authenticated user.
        """
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if not st.session_state['authentication_status']:
            self._check_cookie()
            if not st.session_state['authentication_status']:
                if location == 'main':
                    login_form = st.form('Login')
                elif location == 'sidebar':
                    login_form = st.sidebar.form('Login')

                login_form.subheader(form_name)
                self.username = login_form.text_input('Username').lower()
                st.session_state['username'] = self.username
                self.password = login_form.text_input('Password', type='password')

                if login_form.form_submit_button('Login'):
                    self.files_my = []
                    self.files_shared = []
                    self.devices = []
                    self.metrics = []
                    self.metrics_df = {}
                    self.queriables = []
                    self._check_credentials(self.username, self.password)

        return st.session_state['name'], st.session_state['authentication_status'], st.session_state['username']

    def logout(self, button_name: str, location: str='main', key: str=None):
        """
        Creates a logout button.

        Parameters
        ----------
        button_name: str
            The rendered name of the logout button.
        location: str
            The location of the logout button i.e. main or sidebar.
        """
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if location == 'main':
            if st.button(button_name, key):
                self.cookie_manager.delete(self.cookie_name)
                st.session_state['logout'] = True
                st.session_state['name'] = None
                st.session_state['username'] = None
                st.session_state['authentication_status'] = None
        elif location == 'sidebar':
            if st.sidebar.button(button_name, key):
                self.cookie_manager.delete(self.cookie_name)
                st.session_state['logout'] = True
                st.session_state['name'] = None
                st.session_state['username'] = None
                st.session_state['authentication_status'] = None

#    def _update_password(self, username: str, password: str):
        """
        Updates credentials dictionary with user's reset hashed password.

        Parameters
        ----------
        username: str
            The username of the user to update the password for.
        password: str
            The updated plain text password.
        """
 #       self.credentials['usernames'][username]['password'] = Hasher([password]).generate()[0]

    def reset_password(self, username: str, form_name: str, location: str='main') -> bool:
        """
        Creates a password reset widget.

        Parameters
        ----------
        username: str
            The username of the user to reset the password for.
        form_name: str
            The rendered name of the password reset form.
        location: str
            The location of the password reset form i.e. main or sidebar.
        Returns
        -------
        str
            The status of resetting the password.
        """
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if location == 'main':
            reset_password_form = st.form('Reset password')
        elif location == 'sidebar':
            reset_password_form = st.sidebar.form('Reset password')
        
        reset_password_form.subheader(form_name)
        self.username = username.lower()
        self.password = reset_password_form.text_input('Current password', type='password')
        new_password = reset_password_form.text_input('New password', type='password')
        new_password_repeat = reset_password_form.text_input('Repeat password', type='password')

        if reset_password_form.form_submit_button('Reset'):
            if self._check_credentials(inplace=False):
                if len(new_password) > 0:
                    if new_password == new_password_repeat:
                        if self.password != new_password: 
                            self._update_password(self.username, new_password)
                            return True
                        else:
                            raise ResetError('New and current passwords are the same')
                    else:
                        raise ResetError('Passwords do not match')
                else:
                    raise ResetError('No new password provided')
            else:
                raise CredentialsError('password')
    
    def get_devices(self, location: str='main') -> dict:
        """
        Creates a device widget.

        Parameters
        ----------
        location: str
            The location of the login form i.e. main or sidebar.
        Returns
        -------
        str
            Name of the authenticated user.
        bool
            The status of authentication, None: no credentials entered, 
            False: incorrect credentials, True: correct credentials.
        str
            Username of the authenticated user.
        """
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if not self.username:
            raise CredentialsError('Username is not valid')
        
        if not self.devices:
            self.devices = self._get_devices(self.username)
            self.queriables = self._get_chunk(self.devices)
        return self.devices
    
    def get_metrics(self, location: str='main') -> dict:
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if not self.username:
            raise CredentialsError('Username is not valid')
        
        if self.devices and not self.metrics:
            metrics = self._get_metrics(self.devices)
            data = {
                "peer_id": [msg.peer_id for msg in metrics],
                "peer_name": [get_device_name(msg.peer_id, self.devices) for msg in metrics],
                "uptime_start": [datetime.utcfromtimestamp(int(msg.uptime_start)).date() for msg in metrics],
                "uptime_end": [msg.uptime_end for msg in metrics],
                "disk_read": [int(msg.disk_read) for msg in metrics],
                "disk_write": [int(msg.disk_write) for msg in metrics],
                "throughput": [int(msg.throughput) for msg in metrics]
            }
            self.metrics_df = pd.DataFrame(data)
            self.metrics = metrics
        return self.metrics, self.metrics_df
    
    def get_file_list(self, location: str='main') -> dict:
        """
        Creates a device widget.

        Parameters
        ----------
        location: str
            The location of the login form i.e. main or sidebar.
        Returns
        -------
        str
            Name of the authenticated user.
        bool
            The status of authentication, None: no credentials entered, 
            False: incorrect credentials, True: correct credentials.
        str
            Username of the authenticated user.
        """
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if not self.username:
            raise CredentialsError('Username is not valid')
        
        if not self.files_my and not self.files_shared:
            self.files_my, self.files_shared = self._get_file_list(self.username)
            self.files_my.sort(key=lambda f: f.file_name)
            self.files_shared.sort(key=lambda f: f.file_name)
        return self.files_my, self.files_shared
    
    def get_file(self, file_id:str, location: str='main') -> dict:
        """
        Creates a device widget.

        Parameters
        ----------
        location: str
            The location of the login form i.e. main or sidebar.
        Returns
        -------
        str
            Name of the authenticated user.
        bool
            The status of authentication, None: no credentials entered, 
            False: incorrect credentials, True: correct credentials.
        str
            Username of the authenticated user.
        """
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if not self.username:
            raise CredentialsError('Username is not valid')
        
        file_new = self._get_file(file_id, self.username)
        for elem in self.files_my:
            if elem.file_id == file_new.file_id:
                elem.file_bytes = file_new.file_bytes
        for elem in self.files_shared:
            if elem.file_id == file_new.file_id:
                elem.file_bytes = file_new.file_bytes
        return self._get_file(file_id, self.username)

    def _register_credentials(self, username: str, name: str, surname: str, password: str, email: str, preauthorization: bool) -> bool:
        """
        Adds to credentials dictionary the new user's information.

        Parameters
        ----------
        username: str
            The username of the new user.
        name: str
            The name of the new user.
        password: str
            The password of the new user.
        email: str
            The email of the new user.
        preauthorization: bool
            The preauthorization requirement, True: user must be preauthorized to register, 
            False: any user can register.
        """
        if not self.validator.validate_username(username):
            raise RegisterError('Username is not valid')
        if not self.validator.validate_name(name):
            raise RegisterError('Name is not valid')
        if not self.validator.validate_name(surname):
            raise RegisterError('Surname is not valid')
        if not self.validator.validate_email(email):
            raise RegisterError('Email is not valid')
        
        salt = bcrypt.gensalt()
        hashed_password = bcrypt.hashpw(password.encode('utf-8'), salt)
        return self.network.send_credentials(username, hashed_password.decode('utf-8'), name, surname, email, salt.decode('utf-8'))

    def register_user(self, form_name: str, location: str='main', preauthorization=True) -> bool:
        """
        Creates a register new user widget.

        Parameters
        ----------
        form_name: str
            The rendered name of the register new user form.
        location: str
            The location of the register new user form i.e. main or sidebar.
        preauthorization: bool
            The preauthorization requirement, True: user must be preauthorized to register, 
            False: any user can register.
        Returns
        -------
        bool
            The status of registering the new user, True: user registered successfully.
        """   
        if preauthorization:
            if not self.preauthorized:
                raise ValueError("preauthorization argument must not be None")
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if location == 'main':
            register_user_form = st.form('Register user')
        elif location == 'sidebar':
            register_user_form = st.sidebar.form('Register user')

        register_user_form.subheader(form_name)
        new_email = register_user_form.text_input('Email')
        new_username = register_user_form.text_input('Username').lower()
        new_name = register_user_form.text_input('Name')
        new_surname = register_user_form.text_input('Surname')
        new_password = register_user_form.text_input('Password', type='password')
        new_password_repeat = register_user_form.text_input('Repeat password', type='password')

        if register_user_form.form_submit_button('Register'):
            if len(new_email) and len(new_username) and len(new_name) and len(new_surname) and len(new_password) > 0:
                if new_password == new_password_repeat:
                    self._register_credentials(new_username, new_name, new_surname, new_password, new_email, preauthorization)
                    return True
                else:
                    raise RegisterError('Passwords do not match')
            else:
                raise RegisterError('Please enter an email, username, name, surname and password')

    def register_device(self, form_name: str, location: str='main', preauthorization=True) -> bool:
        """
        Creates a register new device widget.

        Parameters
        ----------
        form_name: str
            The rendered name of the register new user form.
        location: str
            The location of the register new user form i.e. main or sidebar.
        preauthorization: bool
            The preauthorization requirement, True: user must be preauthorized to register, 
            False: any user can register.
        Returns
        -------
        bool
            The status of registering the new user, True: user registered successfully.
        """   
        if preauthorization:
            if not self.preauthorized:
                raise ValueError("preauthorization argument must not be None")
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if location == 'main':
            register_user_form = st.form('Register user')
        elif location == 'sidebar':
            register_user_form = st.sidebar.form('Register user')

        register_user_form.subheader(form_name)
        device_name = register_user_form.text_input('Device name')
        disk_size = register_user_form.text_input('Disk size (MB)')
        #disk_type = register_user_form.multiselect('Disk type', c.STORAGE_TYPE, max_selections=1)
        mount_point = register_user_form.text_input('Mount point')
        country = register_user_form.multiselect('Country', c.COUNTRIES, max_selections=1)
        
        if register_user_form.form_submit_button('Register'):
            if len(device_name) and len(disk_size) and len(mount_point) and len(country):
                return self._register_device(device_name, disk_size, mount_point, country[0])
            else:
                raise RegisterError('Please enter a device name, amount of disk, disk type and country')

    def _register_device(self, device_name: str, disk_size: str, mount_point: str, country: str) -> bool:
        if not self.validator.validate_name(device_name):
            raise RegisterError('Device name is not valid')
        if not self.validator.validate_disk_size(disk_size):
            raise RegisterError('Disk size is not valid')
        if not self.validator.validate_mount_point(mount_point):
            raise RegisterError('Mount point is not valid')
        if not self.validator.validate_name(country):
            raise RegisterError('Country is not valid')

        result = self.network.create_device(device_name, st.session_state['username'], disk_size, mount_point, country)
        if result != False:
            self.devices.append({
                'peer_id': result,
                'device_name': device_name,
                'disk_size': disk_size,
                'mount_point': mount_point,
                'registration_date': str(datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
            })
            self.network.get_chunk(self.devices)
            return True
        raise RegisterError('Device not inserted.')

   # def _set_random_password(self, username: str) -> str:
        """
        Updates credentials dictionary with user's hashed random password.

        Parameters
        ----------
        username: str
            Username of user to set random password for.
        Returns
        -------
        str
            New plain text password that should be transferred to user securely.
        """
    #    self.random_password = generate_random_pw()
    #    self.credentials['usernames'][username]['password'] = Hasher([self.random_password]).generate()[0]
    #    return self.random_password

  #  def forgot_password(self, form_name: str, location: str='main') -> tuple:
        """
        Creates a forgot password widget.

        Parameters
        ----------
        form_name: str
            The rendered name of the forgot password form.
        location: str
            The location of the forgot password form i.e. main or sidebar.
        Returns
        -------
        str
            Username associated with forgotten password.
        str
            Email associated with forgotten password.
        str
            New plain text password that should be transferred to user securely.
        """
  #      if location not in ['main', 'sidebar']:
  #          raise ValueError("Location must be one of 'main' or 'sidebar'")
  #      if location == 'main':
#            forgot_password_form = st.form('Forgot password')
#        elif location == 'sidebar':
  #          forgot_password_form = st.sidebar.form('Forgot password')

#        forgot_password_form.subheader(form_name)
 #       username = forgot_password_form.text_input('Username').lower()

#        if forgot_password_form.form_submit_button('Submit'):
 #           if len(username) > 0:
  #              if username in self.credentials['usernames']:
   #                 return username, self.credentials['usernames'][username]['email'], self._set_random_password(username)
    #            else:
    #                return False, None, None
    #        else:
    #            raise ForgotError('Username not provided')
    #    return None, None, None

   # def _get_username(self, key: str, value: str) -> str:
        """
        Retrieves username based on a provided entry.

        Parameters
        ----------
        key: str
            Name of the credential to query i.e. "email".
        value: str
            Value of the queried credential i.e. "jsmith@gmail.com".
        Returns
        -------
        str
            Username associated with given key, value pair i.e. "jsmith".
        """
    #    for username, entries in self.credentials['usernames'].items():
    #        if entries[key] == value:
    #            return username
    #    return False

    def upload_file(self, file, tab):
        b = file.read()
        file = self.network.upload_file(self.username, file.name, str(file.size), file.type, b)
        if file and file.file_id:
            self.files_my.append(file)
            self.files_my.sort(key=lambda f: f.file_name, reverse=True)
        else:
            tab.warning('File not uploaded.')

    def forgot_username(self, form_name: str, location: str='main') -> tuple:
        """
        Creates a forgot username widget.

        Parameters
        ----------
        form_name: str
            The rendered name of the forgot username form.
        location: str
            The location of the forgot username form i.e. main or sidebar.
        Returns
        -------
        str
            Forgotten username that should be transferred to user securely.
        str
            Email associated with forgotten username.
        """
        if location not in ['main', 'sidebar']:
            raise ValueError("Location must be one of 'main' or 'sidebar'")
        if location == 'main':
            forgot_username_form = st.form('Forgot username')
        elif location == 'sidebar':
            forgot_username_form = st.sidebar.form('Forgot username')

        forgot_username_form.subheader(form_name)
        email = forgot_username_form.text_input('Email')

        if forgot_username_form.form_submit_button('Submit'):
            if len(email) > 0:
                return self._get_username('email', email), email
            else:
                raise ForgotError('Email not provided')
        return None, email

#    def _update_entry(self, username: str, key: str, value: str):
        """
        Updates credentials dictionary with user's updated entry.

        Parameters
        ----------
        username: str
            The username of the user to update the entry for.
        key: str
            The updated entry key i.e. "email".
        value: str
            The updated entry value i.e. "jsmith@gmail.com".
        """
#        self.credentials['usernames'][username][key] = value

#    def update_user_details(self, username: str, form_name: str, location: str='main') -> bool:
        """
        Creates a update user details widget.

        Parameters
        ----------
        username: str
            The username of the user to update user details for.
        form_name: str
            The rendered name of the update user details form.
        location: str
            The location of the update user details form i.e. main or sidebar.
        Returns
        -------
        str
            The status of updating user details.
        """
#        if location not in ['main', 'sidebar']:
#            raise ValueError("Location must be one of 'main' or 'sidebar'")
#        if location == 'main':
#            update_user_details_form = st.form('Update user details')
#        elif location == 'sidebar':
#            update_user_details_form = st.sidebar.form('Update user details')
        
#        update_user_details_form.subheader(form_name)
#        self.username = username.lower()
#        field = update_user_details_form.selectbox('Field', ['Name', 'Email']).lower()
#        new_value = update_user_details_form.text_input('New value')

#        if update_user_details_form.form_submit_button('Update'):
#            if len(new_value) > 0:
#                if new_value != self.credentials['usernames'][self.username][field]:
#                    self._update_entry(self.username, field, new_value)
#                    if field == 'name':
#                            st.session_state['name'] = new_value
#                            self.exp_date = self._set_exp_date()
#                            self.token = self._token_encode()
#                            self.cookie_manager.set(self.cookie_name, self.token,
#                            expires_at=datetime.now() + timedelta(days=self.cookie_expiry_days))
#                    return True
#                else:
#                    raise UpdateError('New and current values are the same')
#            if len(new_value) == 0:
#                raise UpdateError('New value not provided')