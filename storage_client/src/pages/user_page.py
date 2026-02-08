import streamlit as st
from authenticate import Authenticate
from streamlit_extras.switch_page_button import switch_page
import constants as c
import utils
import altair as alt
import pandas as pd
import parsing.parsing_pdf as parsing_pdf
import subprocess

def show_files(files_my, files_shared):
    if not files_my and not files_shared:
        tab1.write('No file found.')
    else:
        if files_my:
            tab1.write('Personal files')
            for file in files_my:
                if file:
                    col1, col2, col3, col4 = tab1.columns(4)
                    col1.metric('File name', file.file_name)
                    size, unit = utils.get_formatted_size(file.file_size)
                    col2.metric('Size', str(size) + ' ' + unit)
                    col3.metric('Upload date', file.upload_date)
                    col4.download_button(
                        key=file.file_id,
                        label="Download",
                        data=file.file_bytes,
                        on_click=authenticator.get_file,
                        kwargs=dict(file_id=file.file_id),
                        file_name=file.file_name,
                        mime=file.file_type
                    )
        if files_shared:
            tab1.write('Shared files')
            for file in files_shared:
                if file:
                    col1, col2, col3, col4, col5, col6 = tab1.columns(6)
                    col1.metric('File name', file.file_name)
                    size, unit = utils.get_formatted_size(file.file_size)
                    col2.metric('Size', str(size) + ' ' + unit)
                    col3.metric('Upload date', file.upload_date)
                    col4.metric('Owner', file.owner)
                    col5.metric('W', file.write)
                    col6.download_button(
                        label="Download",
                        data=b'', # file['file_bytes'],
                        #data=authenticator.get_file(file['file_name'])
                        file_name=file.file_name,
                        mime=file.file_type
                    )

def show_devices(devices):
    if devices:
        for device in devices:
            col1, col2, col3, col4 = tab2.columns(4)
            col1.metric('Device name', device['device_name'])
            size, unit = utils.get_formatted_size(device['disk_size'])
            col2.metric('Size', str(size) + ' ' + unit)
            col3.metric('Mount point', device['mount_point'])
            col4.metric('Registration date', device['registration_date'])
    else:
        tab2.write('No devices found.')

def get_chart(data, column_name, title, titleY):
    hover = alt.selection_single(
        fields=[column_name],
        nearest=True,
        on="mouseover",
        empty="none",
    )

    lines = (
        alt.Chart(data, title=title)
        .mark_line()
        .encode(
            x=alt.X("uptime_start:T", title="Date"),
            y=alt.Y(column_name+":Q", title=titleY),
            color="peer_name",
        )
    )

    # Draw points on the line, and highlight based on selection
    points = lines.transform_filter(hover).mark_circle(size=65)

    # Draw a rule at the location of the selection
    tooltips = (
        alt.Chart(data)
        .mark_rule()
        .encode(
            x="yearmonthdate(uptime_start)",
            y=column_name,
            opacity=alt.condition(hover, alt.value(0.3), alt.value(0)),
            tooltip=[
                alt.Tooltip("uptime_start", title="Date"),
                alt.Tooltip(column_name, title=title),
            ],
        )
        .add_selection(hover)
    )
    return (lines + points + tooltips).interactive()

def show_charts(metrics_df):
    tab3.write('No metrics found.')
    #if metrics_df.empty:
    #    tab3.write('No metrics found.')
    #else:
    #    tab3.altair_chart((get_chart(metrics_df, 'disk_read', 'Disk-Read performance', 'Disk-Read (MB/s)')).interactive(), use_container_width=True)
    #    tab3.altair_chart((get_chart(metrics_df, 'disk_write', 'Disk-Write performance', 'Disk-Write (MB/s)')).interactive(), use_container_width=True)
    #    tab3.altair_chart((get_chart(metrics_df, 'throughput', 'Network performance', 'Throughput (MB/s)')).interactive(), use_container_width=True)

def run_yarn_command(command, target_folder='website'):
    try:
        result = subprocess.run(['yarn'] + command.split(), capture_output=True, text=True, check=True, cwd=target_folder)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print("Error:", e.stderr)
        return None

def git_clone(repo_url, clone_dir):
    try:
        subprocess.run(['git', 'clone', repo_url, clone_dir])
        print("Repository successfully cloned.")
    except Exception as e:
        print("Error:", e)

def git_push(repo_path, commit_message):
    try:
        subprocess.run(['git', 'add', '.'], cwd=repo_path)
        subprocess.run(['git', 'commit', '-m', commit_message], cwd=repo_path)
        subprocess.run(['git', 'push'], cwd=repo_path)
        print("Changes successfully added, committed, and pushed to GitHub.")
    except Exception as e:
        print("Error:", e)

st.set_page_config(layout="wide")
st.session_state.clicked = False
st.sidebar.title("About")
st.sidebar.info("This application allow you to save your file with maximum privacy.")

st.markdown(
    """<style> [data-testid="stMetricValue"] {
        font-size: 30px;
    }</style>""",
    unsafe_allow_html=True,
)

if not st.session_state["authentication_status"]:
    switch_page("main page")
else:
    st.write(f'Welcome *{st.session_state["username"]}*')
    tab1, tab2, tab3, tab4 = st.tabs(["Files", "Devices", "Statistics", "Applications"])

    authenticator = st.session_state.authenticator
    authenticator.logout('Logout', 'sidebar')

    tab1.title('Your Files')
    uploaded_file = tab1.file_uploader("Upload file",  accept_multiple_files = False, )
    #to accept multiple files at once: uploaded_files = tab1.file_uploader("Choose a file", accept_multiple_files=True)

    tab1.divider()
    if uploaded_file:
       st.markdown('''
        <style>
            .uploadedFile {display: none}
        <style>''',
        unsafe_allow_html=True)
       authenticator.upload_file(uploaded_file, tab1) 

    my_files, shared_files = authenticator.get_file_list()
    show_files(my_files, shared_files)

    tab2.title('Your Devices')
    show_devices(authenticator.get_devices())
    tab2.divider()

    tab3.title('Statistics')
    _, metrics_df = authenticator.get_metrics()
    show_charts(metrics_df)

    if "Register Device" not in st.session_state:
        st.session_state["Register Device"] = False
    if tab2.button("Register Device"):
        st.session_state["Register Device"] = not st.session_state["Register Device"]
    if st.session_state["Register Device"]:
        try:
            if authenticator.register_device('Register device', preauthorization=False):
                tab2.success('Device registered successfully')
                show_devices(authenticator.devices)
                st.session_state["Register Device"] = not st.session_state["Register Device"]
        except Exception as e:
            tab2.error(e)