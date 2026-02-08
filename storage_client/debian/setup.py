import os
from setuptools import setup
from nvpy import nvpy

setup(
    name='ctrlz_client',
    version='1.0',
    author='Alessandro Zanni',
    author_email='go@ctrlzeta.me',
    description='Client interface to use ctrl+Z storage',
    license = "BSD",
    scripts=['main_page.py'],
   # data_files=[('/etc/systemd/system', ['ctrlz_client.service']), ('/etc/ctrlz_client', ['ctrlz_client.conf'])],
   # install_requires=['pyftplib','configparser', 'eclipse-zenoh', 'streamlit', 'streamlit-authenticator', 'streamlit-extras', 'protobuf', 'pandas', 'tornado'],
    packages=['pages', 'protos', 'website'],
    entry_points = {
        'console_scripts' : ['myscript = myscript.myscript:main']
    },
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3",
        "Operating System :: POSIX :: Linux",
    ],
)