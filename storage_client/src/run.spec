# -*- mode: python ; coding: utf-8 -*-
from PyInstaller.utils.hooks import collect_data_files
from PyInstaller.utils.hooks import copy_metadata

import sys
sys.setrecursionlimit(10000)

block_cipher = None

datas = [("C:\\Users\\alessandro\\AppData\\Local\\Packages\\PythonSoftwareFoundation.Python.3.10_qbz5n2kfra8p0\\LocalCache\\local-packages\\Python310\\site-packages\\streamlit\\runtime", "./streamlit/runtime")]
datas += collect_data_files("streamlit")
datas += collect_data_files("streamlit_authenticator")
datas += collect_data_files("extra_streamlit_components")
datas += collect_data_files("streamlit_extras")
datas += collect_data_files("streamlit_extras.switch_page_button")
datas += copy_metadata("streamlit")

a = Analysis(
    ['run.py'],
    pathex=[],
    binaries=[],
    datas=datas,
    hiddenimports=[
        "streamlit",
        "streamlit_authenticator",
        "extra_streamlit_components",
        "streamlit_extras",
        "streamlit_extras.switch_page_button",
        "zenoh"
    ],
    hookspath=['./hooks'],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)
pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='run',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
