from db_manager import DbManager
import parameters as param

def get_token(db_manager, token):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    query = """
    SELECT auth_token
    FROM users
    WHERE auth_token='%s'
    """
    select_query = query % (token)
    return db_manager.select(select_query)

def search_user_token(db_manager, username, token):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    query = """
    SELECT auth_token
    FROM users
    WHERE username='%s' AND auth_token='%s'"""
    select_query=query%(username, token)
    return db_manager.select(select_query)

def search_user_password(db_manager, username, password):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    query = """
    SELECT count(*)
    FROM users
    WHERE username='%s' AND password='%s'"""
    select_query=query%(username,password)
    result = db_manager.select(select_query)
    return str(result[0]).strip('Row(count=').strip(')')

def insert_user(db_manager, username, password, email, name, surname):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    general_query = """
    INSERT INTO
    users (id, username, password, email, name, surname)
    VALUES (now(), '%s', '%s', '%s', '%s', '%s')
    """
    insert_query = general_query %(username, password, email, name, surname)
    result = db_manager.insert(insert_query)
    return result

def select_producer(db_manager, producer_id):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    general_query = """
    SELECT username, toTimestamp(ts)
    FROM providers
    WHERE id = %s
    """
    select_query = general_query % (producer_id)
    result = db_manager.select(select_query)
    return result

def select_peer_id(db_manager, peer_id):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    SELECT peer_id
    FROM peers
    WHERE peer_id='%s'
    """
    select_query = query % (peer_id)
    result = db_manager.select(select_query)
    return result

def select_peer(db_manager, peer_id, username):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    SELECT peer_id, toTimestamp(ts)
    FROM peers
    WHERE peer_id='%s' and username='%s'
    """
    select_query = query %(peer_id, username)
    result = db_manager.select(select_query)
    return result

def select_peer_from_cost(db_manager):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    SELECT peer_id
    FROM peers
    """
    return db_manager.select(query)

def update_peer(db_manager, peer_id, peer_token, username, bandwidth, space, db_location):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    UPDATE peers
    SET peer_token='%s', total_bandwidth=%s, total_space=%s, db_location='%s', ts=now()
    WHERE username='%s' AND peer_id='%s'
    """
    update_query = query %(peer_token, bandwidth, space, db_location, username, peer_id)
    result = db_manager.insert(update_query)
    return result

def update_token(db_manager, username, password, token):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    general_query = """
    UPDATE users
    SET auth_token='%s'
    WHERE username='%s' and password='%s'
    """
    update_query = general_query %(token, username, password)
    result = db_manager.update(update_query)
    return result

# configuration operations
def get_key(db_manager, key):
    db_manager.set_keyspace(param.KEYSPACE_CONFIGURATIONS)
    general_query = """
    SELECT *
    FROM configurations
    WHERE key='%s'
    """
    select_query = general_query %(key)
    return db_manager.select(select_query)

def set_key(db_manager, key, value):
    db_manager.set_keyspace(param.KEYSPACE_CONFIGURATIONS)
    general_query = """
    UPDATE configurations
    SET value='%s'
    WHERE key='%s'
    """
    update_query = general_query %(value, key)
    return db_manager.update(update_query)

# chunks distributions operations
def get_consumer_details(db_manager, username):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    general_query = """
    SELECT id, plan_type, usage, payment
    FROM consumers
    WHERE username='%s'
    """
    select_query = general_query %(username)
    consumers = db_manager.select(select_query)
    consumer = []
    for element in consumers:
        consumer.append(element[0])
    return consumer

def select_peers(db_manager, peers_available):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    peers_list = ''
    if peers_available:
        for peer in peers_available:
            peers_list += ('\'' +str(peer) + '\',')
    query = 'SELECT peer_id, peer_token FROM peers WHERE peer_id IN ('+peers_list[:-1]+')'
    results = db_manager.select(query)
    peers = []
    for row in results:
        p = []
        for element in row:
            p.append(element)
        peers.append(p)
    return peers

def select_metrics(db_manager, chunk_size, datetime_limit):
    db_manager.set_keyspace(param.KEYSPACE_MONITORING)
    query = """
    SELECT peer_id, space_available, bandwidth_available, local_uptime, speed_connection, speed_storage
    FROM metrics
    WHERE ts < maxTimeuuid('%s')
    AND space_available > %s
    ALLOW FILTERING
    """
    select_query = query % (datetime_limit, chunk_size)
    results = db_manager.select(select_query)
    metrics = []
    for row in results:
        m = []
        for element in row:
            m.append(element)
        metrics.append(m)
    return metrics

def select_metrics_from_peer(db_manager, peer_id):
    db_manager.set_keyspace(param.KEYSPACE_MONITORING)
    query = """
    SELECT space_available, bandwidth_available, used_space, used_bandwidth, local_uptime, speed_connection, speed_storage
    FROM metrics
    WHERE peer_id = '%s'
    """
    select_query = query % (peer_id)
    return db_manager.select(select_query)

def select_metrics_history(db_manager, peers):
    db_manager.set_keyspace(param.KEYSPACE_MONITORING)
    query = """
    SELECT peer_id, avg(space_available), avg(bandwidth_available), avg(local_uptime), avg(speed_connection), avg(speed_storage), avg(used_bandwidth), avg(used_space)
    FROM metrics_history
    WHERE peer_id IN (%s)
    """
    peer_list = ''
    for peer_id in peers:
        peer_list += '\'' + peer_id + '\','
    peer_list = peer_list[:-1]
    select_query = query % (peer_list)
    return db_manager.select(select_query)

def select_peer_info(db_manager, peer_id):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    SELECT peer_id, peer_token, username, total_bandwidth, total_space, db_location, ip, location, region, ts
    FROM peers
    WHERE peer_id = '%s'
    """
    select_query = query % (peer_id)
    return db_manager.select(select_query)

def search_chunk_peer(db_manager, chunk_id):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query =  """
    SELECT chunk_id, peer_id
    FROM chunks_peers
    WHERE chunk_id IN (%s)
    ALLOW FILTERING
    """
    select_query = query % (chunk_id)
    return db_manager.select(select_query)

def update_chunks_peers(db_manager, chunk_id, peer_id, upload_status):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query =  """
    UPDATE chunks_peers
    SET chunk_status = '%s'
    WHERE chunk_id = '%s' AND peer_id='%s'
    """
    update_query = query % (upload_status, chunk_id, peer_id)
    results = db_manager.update(update_query)

def insert_file(db_manager, filename, username, permission, size, upload_status):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query =  """
    INSERT INTO files (filename, username, permissions, size, upload_status)
    VALUES ('%s', '%s', '%s', %s, '%s')
    """
    insert_query = query % (filename, username, permission, size, upload_status)
    results = db_manager.insert(insert_query)

def insert_user(db_manager, username, password, email, name, surname):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    general_query = """
    INSERT INTO
    users (id, username, password, email, name, surname)
    VALUES (now(), '%s', '%s', '%s', '%s', '%s')
    """
    insert_query = general_query %(username, password, email, name, surname)
    result = db_manager.insert(insert_query)
    return result

def update_user(db_manager, username, password, token):
    db_manager.set_keyspace(param.KEYSPACE_USERS)
    general_query = """
    UPDATE users
    SET auth_token='%s'
    WHERE username='%s' and password='%s'
    """
    update_query = general_query %(token, username, password)
    result = db_manager.update(update_query)
    return result

#file_manager operations
def get_list_files(db_manager, username):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    general_query = """
    SELECT filename, username, permissions, size, upload_status
    FROM files
    WHERE username='%s'
    ALLOW FILTERING
    """
    select_query = general_query %(username)
    result = db_manager.select(select_query)
    return result

def get_list_chunks(db_manager, username, filename):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    SELECT chunk_id, filename, sequence_number, size, offset
    FROM chunks
    WHERE username='%s' AND filename='%s'
    ALLOW FILTERING
    """
    select_query = query %(username, filename)
    return db_manager.select(select_query)

def update_files(db_manager, username, filename, permissions, size, upload_status):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    general_query = """
    UPDATE files
    SET permissions='%s', size=%s, upload_status='%s'
    WHERE filename='%s'
    AND username='%s'
    """
    update_query = general_query % (permissions, size, upload_status, filename, username)
    return db_manager.update(update_query)

def search_chunk(db_manager, chunk_id):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    SELECT *
    FROM chunks
    WHERE chunk_id=%s
    """
    select_query = query %(chunk_id)
    return db_manager.select(select_query)

def insert_chunks(db_manager, chunk_id, filename, username, sequence_number, size, offset):
    db_manager.set_keyspace(param.KEYSPACE_FILES)
    query = """
    INSERT INTO chunks (chunk_id, username, filename, sequence_number, size, offset)
    VALUES (%s, '%s', '%s', '%s', %s, %s)
    """
    insert_query = query %(chunk_id, username, filename, sequence_number, size, offset)
    result = db_manager.select(insert_query)
    return result

def update_metrics(db_manager, peer_id, space_available, local_uptime, bandwidth_available, speed_connection, speed_storage, used_bandwidth, used_space):
    db_manager.set_keyspace(param.KEYSPACE_MONITORING)
    query = 'UPDATE metrics SET ts=now(), '
    if space_available:
        query += ' space_available=' + str(space_available) + ','
    if local_uptime:
        query += ' local_uptime=' + str(local_uptime) + ','
    if bandwidth_available:
        query += ' bandwidth_available=' + str(bandwidth_available) + ','
    if speed_connection:
        query += ' speed_connection=' + str(speed_connection) + ','
    if speed_storage:
        query += ' speed_storage=' + str(speed_storage) + ','
    if used_bandwidth:
        query += ' used_bandwidth=' + str(used_bandwidth) + ','
    if used_space:
        query += ' used_space=' + str(used_space) + ','
    query = query[:-1] + ' WHERE peer_id=\'' + str(peer_id) + '\''
    db_manager.update(query)
    #return results

def update_metrics_history(db_manager, peer_id, space_available, local_uptime, bandwidth_available, speed_connection, speed_storage, used_bandwidth, used_space):
    db_manager.set_keyspace(param.KEYSPACE_MONITORING)
    query = 'UPDATE metrics_history SET '
    if space_available:
        query += ' space_available=' + str(space_available) + ','
    if local_uptime:
        query += ' local_uptime=' + str(local_uptime) + ','
    if bandwidth_available:
        query += ' bandwidth_available=' + str(bandwidth_available) + ','
    if speed_connection:
        query += ' speed_connection=' + str(speed_connection) + ','
    if speed_storage:
        query += ' speed_storage=' + str(speed_storage) + ','
    if used_bandwidth:
        query += ' used_bandwidth=' + str(used_bandwidth) + ','
    if used_space:
        query += ' used_space=' + str(used_space) + ','
    query = query[:-1] + ' WHERE peer_id=\'' + str(peer_id) + '\' AND ts=now()'
    db_manager.update(query)
    #return results

def select_bill_information(db_manager, username):
    db_manager.set_keyspace(param.KEYSPACE_BILLING)
    query = """
    SELECT used_peer_id, time_consumed, storage_amount, ts_bill_start, ts_bill_end
    FROM billing
    WHERE username= '%s'
    ORDER BY used_peer_id
    """
    select_query = query % (username)
    result = db_manager.select(select_query)
    return result

def update_bill_information(db_manager, username, storage_amount, time_consumed, used_peer_id, ts_bill_end):
    db_manager.set_keyspace(param.KEYSPACE_BILLING)
    query = """
    UPDATE billing
    SET storage_amount = %s, time_consumed = %s
    WHERE used_peer_id = '%s'
    AND username= '%s'
    AND ts_bill_start = toTimeStamp(now())
    AND ts_bill_end = '%s'
    """
    select_query = query % (storage_amount,time_consumed,used_peer_id,username,ts_bill_end)
    result = db_manager.select(select_query)
    return result

def get_peer_unit_cost(db_manager, peer_id):
    db_manager.set_keyspace(param.KEYSPACE_BILLING)
    query = """
    SELECT unit_cost
    FROM peer_cost
    WHERE peer_id = '%s'
    """
    select_query = query % (peer_id)
    result = db_manager.select(select_query)
    return result

def update_peer_unit_cost(db_manager, peer_id, cost):
    db_manager.set_keyspace(param.KEYSPACE_BILLING)
    query = """
    UPDATE peer_cost
    SET unit_cost = %s
    WHERE peer_id= '%s'
    """
    update_query = query % (cost, peer_id)
    return db_manager.update(update_query)
