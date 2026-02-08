import zlib
import	zenoh
from zenoh import QueryTarget, Query, Sample
import constants as c
from datetime import datetime
import argparse
import json
import re
from protos.data_pb2 import MsgDevice, MsgFileId, MsgLiveness, MsgMetrics, MsgPeerId, MsgChunk, MsgUser, MsgUsername, MsgPermissions, MsgFileInfo
import time
import base64

target = {
    'ALL': QueryTarget.ALL(),
    'BEST_MATCHING': QueryTarget.BEST_MATCHING(),
    'ALL_COMPLETE': QueryTarget.ALL_COMPLETE(),
}

def queryable_callback(query: Query):
    print(f">> [Queryable ] Received Query '{query.selector}'")
    if query.value is None:
        query.reply(Sample(str(query.selector), 'error'))
    else:
        print(query.value.payload)
        msg_chunk = MsgChunk()
        msg_chunk.ParseFromString(query.value.payload)
        print(f"File_id: {msg_chunk.file_id}")
        print(f"Piece_id: {msg_chunk.piece_id}")
        print(f"Chunk_id: {msg_chunk.chunk_id}")
        print(f"Bytes: {msg_chunk.chunk_bytes}")
        print(f">> [Queryable ] Received Query '{query.selector}' with chunk_id: '{msg_chunk.chunk_id}' and '{len(msg_chunk.chunk_bytes)}' bytes")
        with open(c.FILES_FOLDER + msg_chunk.file_id + '_' + msg_chunk.piece_id + '_' + msg_chunk.msg_chunk, 'wb') as file:
            file.write(msg_chunk.chunk_bytes)
        query.reply(Sample(str(query.selector), 'ok'))

class Network:
    """
    This class will conect to the remote server.
    """
    def __init__(self):
        """
        Create a new instance of "Network".
        """
        parser = argparse.ArgumentParser(prog='storage-client', description='Storage client')
        parser.add_argument('--mode', '-m', dest='mode',
                        choices=['peer', 'client'],
                        type=str,
                        default='client',
                        help='The zenoh session mode.')
        parser.add_argument('--connect', '-e', dest='connect',
                        metavar='ENDPOINT',
                        action='append',
                        type=str,
                        default=c.ROUTER,
                        help='zenoh endpoints to connect to.')
        #parser.add_argument('--listen', '-l', dest='listen',
        #                metavar='ENDPOINT',
        #                action='append',
        #                type=str,
        #                default='tcp/172.22.209.161:8000',
        #                help='zenoh endpoints to listen on.')
        args, _ = parser.parse_known_args()
        args = vars(args)
        z_mode = args['mode']
        z_connect = args['connect']
        conf = zenoh.Config()
        conf.insert_json5(zenoh.config.MODE_KEY, json.dumps(z_mode))
        conf.insert_json5(zenoh.config.CONNECT_KEY, json.dumps([z_connect]))
      
        zenoh.init_logger()
        self.session = zenoh.open(conf)
        info = self.session.info()
        print(f"zid: {info.zid()}")
        print(f"routers: {info.routers_zid()}")
        print(f"peers: {info.peers_zid()}")

    def get_chunk(self, devices: dict):
        """
        Retrieves a chunk of data from the specified devices.

        Args:
            devices (dict): A dictionary containing information about the devices.

        Returns:
            list: A list of queriable pointers for the retrieved data.
        """
        queriables = []
        for device in devices:
            key_expr = c.KEY_EXPR_FILE_DISTRIBUTION + device['peer_id']
            print("Declaring Queryable on '{}'".format(key_expr))
            queriables.append(self.session.declare_queryable(key_expr, queryable_callback, complete=True))
        # Need to conserve the queriable pointers to avoid RAII deletion
        return queriables

    def send_credentials(self, username: str, password: str, name: str, surname: str, email: str, salt: str) -> bool: 
        """
        Sends user credentials to the server for registration.

        Args:
            username (str): The username of the user.
            password (str): The password of the user.
            name (str): The name of the user.
            surname (str): The surname of the user.
            email (str): The email of the user.
            salt (str): The salt used for password hashing.

        Returns:
            bool: True if the credentials were successfully sent, False otherwise.
        """
        selector = c.KEY_EXPR_USER_SIGNUP + username
        print('selector:'+selector+',username:'+username+',password:'+password+',email:'+email+',name:'+name+',surname:'+surname+',salt:'+str(salt))

        msg_user = MsgUser(
            username=username,
            name=name,
            surname=surname,
            password=password,
            email=email,
            salt=salt,
            registration_date=str(datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
        )

        replies = self.session.get(
            selector, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=msg_user.SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, reply.ok.payload.decode("utf-8")))
                return True
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
                return False

    def check_credentials(self, user) -> str: 
        """
        Check the credentials of a user.

        Args:
            user (str): The username to check.

        Returns:
            Tuple[str, str]: A tuple containing the password and salt of the user, or empty strings if the credentials are not found.
        """
        print('username:' + user)
        replies = self.session.get(
            c.KEY_EXPR_USER_LOGIN + user, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=MsgUsername(username=user).SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        pattern = re.compile(r'(\w+):([^,]+)')
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                if value:
                    matches = pattern.findall(reply.ok.payload.decode("utf-8"))
                    print(">> Received ('{}': '{}')".format(reply.ok.key_expr, dict(matches)))
                    return dict(matches)['password'], dict(matches)['salt']
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return '', ''

    def create_device(self, device_name: str, username: str, disk_size: str, mount_point: str, country: str) -> bool: 
        """
        Creates a new device for a user.

        Args:
            device_name (str): The name of the device.
            username (str): The username of the user.
            disk_size (str): The disk size of the device.
            mount_point (str): The mount point of the device.
            country (str): The country of the user.

        Returns:
            bool: True if the device is created successfully, False otherwise.
        """
        selector = c.KEY_EXPR_PEER_SIGNUP + username
        disk_size = str(int(disk_size)*1000000)
        print('selector:'+selector+',username:'+username+',device_name:'+device_name+',disk_size:'+disk_size)
        
        new_device = MsgDevice(
            username=username,
            device_name=device_name,
            disk_size=disk_size,
            mount_point=mount_point,
            country=country,
            registration_date=str(datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
        )

        replies = self.session.get(
            selector, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=new_device.SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, value))
                if value != c.FALSE:
                    return value
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return False

    def get_devices(self, user) -> dict: 
        """
        Retrieves the devices associated with the specified user.

        Args:
            user (str): The username of the user.

        Returns:
            dict: A dictionary containing the devices associated with the user.

        """
        print('username:' + user)
        replies = self.session.get(
            c.KEY_EXPR_PEER_GET + user, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=MsgUsername(username=user).SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
                if value:
                    return json.loads(value)
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return {}
    
    def get_file_list(self, user) -> dict: 
        """
        Retrieves the file list for a given user.

        Args:
            user (str): The username of the user.

        Returns:
            dict: A dictionary containing the file list.

        """
        print('username:' + user)
        replies = self.session.get(
            c.KEY_EXPR_FILE_LIST + user, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=MsgUsername(username=user).SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
                if value:
                    files = json.loads(value)
                    my_files = []
                    shared_files = []
                    for elem in files[0]:
                        msg = MsgFileInfo()
                        msg.ParseFromString(base64.b64decode(elem))
                        my_files.append(msg)
                    for elem in files[1]:
                        msg = MsgFileInfo()
                        msg.ParseFromString(base64.b64decode(elem))
                        shared_files.append(msg)
                    return my_files, shared_files
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return {}
    
    def get_file(self, file_id: str, user: str) -> dict: 
        """
        Retrieves a file from the server.

        Args:
            file_id (str): The name of the file to retrieve.
            user (str): The username of the user requesting the file.

        Returns:
            dict: A dictionary containing the file data, or an empty dictionary if the file was not found.
        """
        print('file_id:' + file_id + ',username:' + user)

        replies = self.session.get(
            c.KEY_EXPR_FILE_GET + user, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=MsgFileId(username=user, file_id=file_id).SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
                if value:
                    msg = MsgFileInfo()
                    msg.ParseFromString(base64.b64decode(value))
                    if msg.file_size_compressed <= msg.file_size:
                        msg.file_bytes = zlib.decompress(msg.file_bytes)
                    return msg
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return {}

    def upload_file(self, username: str, file_name: str, file_size: str, file_type: str, file_bytes: bytes) -> MsgFileInfo: 
        """
        Uploads a file to the server.

        Args:
            username (str): The username of the user uploading the file.
            file_name (str): The name of the file.
            file_size (str): The size of the file.
            file_type (str): The type of the file.
            file_bytes (bytes): The bytes of the file.

        Returns:
            bool: True if the file was successfully uploaded, False otherwise.
        """
        selector = c.KEY_EXPR_FILE_UPLOAD + username
        print('selector:'+selector+',username:'+username+',file_name:'+file_name+',file_size:'+str(file_size),',file_type:'+file_type)

        file_byte_compressed = zlib.compress(file_bytes)
        file_size_compressed = len(file_byte_compressed)
        if file_size_compressed <= len(file_bytes):
            file_bytes = file_byte_compressed

        file_data = MsgFileInfo(
            username=username,
            file_id="",
            file_name=file_name,
            file_size=file_size,
            file_size_compressed=str(file_size_compressed),
            file_type=file_type,
            upload_date=str(datetime.now().strftime('%Y-%m-%d %H:%M:%S')),
            file_bytes=file_bytes,
            owner="",
            write="",
        )

        replies = self.session.get(
            selector,
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=file_data.SerializeToString()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, value))
                if value != c.FALSE:
                    file_data.file_id = value
                    if file_data.file_size_compressed <= file_data.file_size:
                        file_data.file_bytes = zlib.decompress(file_data.file_bytes)
                    return file_data
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
            return MsgFileInfo()

    def delete_file(self, file_id: str, user: str) -> dict: 
        """
        Deletes a file from the storage.

        Args:
            file_id (str): The ID of the file to be deleted.
            user (str): The username of the user performing the deletion.

        Returns:
            dict: A dictionary containing information about the deleted file, if successful. Otherwise, an empty dictionary.
        """
        print('file_id:' + file_id + ',username:' + user)

        replies = self.session.get(
            c.KEY_EXPR_FILE_DELETE + user, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=MsgFileId(username=user, file_id=file_id).SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
                return value
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return {}
        
    #TODO (to trigger every 1h):
    def send_metrics(self, peer_id: str, uptime_start: str, uptime_end: str, disk_read: str, disk_write: str, throughput:str):
        """
        Sends metrics data to the server.

        Args:
            peer_id (str): The ID of the peer sending the metrics.
            uptime_start (str): The start time of the uptime period.
            uptime_end (str): The end time of the uptime period.
            disk_read (str): The amount of disk read during the uptime period.
            disk_write (str): The amount of disk write during the uptime period.
            throughput (str): The throughput during the uptime period.

        Returns:
            dict: The response from the server, if any.
        """
        print('peer_id:' + peer_id + ', update_start: ' + uptime_start + ', update_end: ' + uptime_end + ', disk_read: ' + 
                disk_read + ', disk_write: ' + disk_write + ', throughput: ' + throughput)
        
        metrics = MsgMetrics(
            peer_id = peer_id,
            uptime_start = uptime_start,
            uptime_end = uptime_end,
            disk_read = disk_read,
            disk_write = disk_write,
            throughput = throughput
        )

        replies = self.session.get(
            c.KEY_EXPR_METRICS_PUT + peer_id, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=metrics.SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
                if value:
                    return json.loads(value)
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return

    def get_metrics(self, devices):
        """
        Retrieves metrics for a given peer ID.

        Args:
            peer_id (str): The ID of the peer.

        Returns:
            list: A list of deserialized metrics messages.

        Raises:
            Exception: If an error occurs during the retrieval process.
        """
        peer_id = ','.join([device['peer_id'] for device in devices])
        print('peer_id: ' + peer_id)
        replies = self.session.get(
            c.KEY_EXPR_METRICS_GET + peer_id, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=MsgPeerId(peer_id=peer_id).SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                deserialized_messages = []
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
                if value:
                    metrics = json.loads(value)
                    for elem in metrics:
                        msg = MsgMetrics()
                        msg.ParseFromString(base64.b64decode(elem))
                        deserialized_messages.append(msg)
                    return deserialized_messages
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return

    #TODO (to trigger every 5min):
    def send_liveness(self, peer_id: str):
        """
        Sends a liveness message to a peer.

        Args:
            peer_id (str): The ID of the peer.

        Returns:
            None
        """
        print('liveness for peer_id ' + peer_id)

        replies = self.session.get(
            c.KEY_EXPR_LIVENESS_PUT + peer_id, 
            zenoh.Queue(), 
            target=target['BEST_MATCHING'], 
            value=MsgLiveness(peer_id=peer_id).SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))

    def set_permission(self, username: str, file_id: str, owner: str, write: str):
        print('username:' + username + ', file_id: ' + file_id + ', owner: ' + owner + ', write: ' + write)

        permission = MsgPermissions(
            username = username,
            file_id = file_id,
            owner = owner,
            write = write,
        )

        replies = self.session.get(
            c.KEY_EXPR_PERMISSION_PUT + username,
            zenoh.Queue(),
            target=target['BEST_MATCHING'],
            value=permission.SerializeToString(),
            consolidation=zenoh.QueryConsolidation.NONE()
        )
        for reply in replies.receiver:
            try:
                value = reply.ok.payload.decode("utf-8")
                print(">> Received ('{}': '{}')".format(reply.ok.key_expr, str(value)))
                if value:
                    return json.loads(value)
            except:
                print(">> Received (ERROR: '{}')".format(reply.err.payload.decode("utf-8")))
        return

    def close(self):
        """
        Close the instance of "Network".
        """
        self.session.close()

def get_arguments():
    parser = argparse.ArgumentParser(description='zenoh client')
    parser.add_argument('--mode', '-m', dest='mode',
                        choices=['peer', 'client'],
                        type=str,
                        default='peer',
                        help='The zenoh session mode.')
    parser.add_argument('--connect', '-e', dest='connect',
                        metavar='ENDPOINT',
                        action='append',
                        type=str,
                        default=['tcp/127.0.0.1:7447'],
                        help='Endpoints to connect to.')
    parser.add_argument('--listen', '-l', dest='listen',
                        metavar='ENDPOINT',
                        action='append',
                        type=str,
                        help='Endpoints to listen on.')
    parser.add_argument("--iter", dest="iter", type=int,
                        help="How many puts to perform")
    parser.add_argument('--config', '-c', dest='config',
                        metavar='FILE',
                        type=str,
                        help='A configuration file.')
    return parser.parse_args()


#while True:
	#time.sleep(1.0)
	#session.put(KEY_EXPR_USER_SIGNUP, "username:user1, name:prova, surname:prova2, password:pass, role:consumer, registration_date:01/01/2024")
	#time.sleep(1.0) 
	#session.put(KEY_EXPR_USER_LOGIN, "username:user1, password:pass")
#	time.sleep(1.0)
#	session.put(KEY_EXPR_PEER_SIGNUP, "peer_id:peer_id1, owner:user1, name:peer_pc, ranking:1, country:Italy, disk_type:hhd, disk_size:1000000000, disk_used:0, disk_available:1000000000, list_piece:[]")
	#time.sleep(1.0)
	#session.put(KEY_EXPR_FILE_UPLOAD, "file_id: file_id1, owner:user1, file_title:file_name, file_size:1000000, upload_date:10/01/2024, piece_num:0, piece_list:{}")
	#time.sleep(1.0)
	#session.put(KEY_EXPR_FILE_GET, "Hello5!")
