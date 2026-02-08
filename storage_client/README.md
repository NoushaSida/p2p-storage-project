## Peer to peer distributed file storage - client side ##

### What is this repository for? ###

* Python client frontend, based on streamlit, to interact with the files storage application.
* v1.0.0

![badge](https://img.shields.io/badge/lang-Python-green)
![badge](https://img.shields.io/badge/storage-in_progess...-blue)

At-a-glance:
- 💻 Client - frontend
- 💡 Python + Streamlit framework
- 🐧 OS agnostic

### Setup the python requirements

To install the python dependencies required, it's reommended to use `virtualenv`. So, in addition to python, install `pip` and `virtualenv` through package management. 
Once they are installed, use the following commands to setup the virtual python environment.

```bash
$ python3 -m venv venv
$ source venv/bin/activate
```

### How to setup

To install the packages required use the `requirements.txt` file in the repository.

```bash
pip install -r /path/to/requirements.txt
```

### Create data for serialization/deserialization

Install the protocol buffer compiler `protoc` [(here the instructions)](https://protobuf.dev/installation)
```bash
# on Ubuntu/Debian
apt install -y protobuf-compiler
protoc --version
```

and compile the `.proto` file

```bash
protoc --python_out . src/protos/data.proto
```

### How to run

```bash
streamlit run src/main_page.py
```

## Create the installer for windows
### Create the spec file
```bash
cd src
pyinstaller --onefile --additional-hooks-dir=./hooks run.py --clean
```

### Create the executable file
```bash
pyinstaller myapp.spec --clean
```

## Create deb file for Debian/Ubuntu
### install dependencies
```bash
pip3 install stdeb
apt-get install python3-all debhelper
apt-get install dh-python
```
