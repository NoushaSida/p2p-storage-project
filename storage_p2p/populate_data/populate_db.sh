#!/bin/bash
cd $HOME/cqlsh-6.8.41/bin

echo "Populating database..."

# Create keyspaces
./cqlsh -e "CREATE KEYSPACE IF NOT EXISTS user WITH replication = {'class': 'SimpleStrategy', 'replication_factor': '1'};"
./cqlsh -e "CREATE KEYSPACE IF NOT EXISTS file WITH replication = {'class': 'SimpleStrategy', 'replication_factor': '1'};"
./cqlsh -e "CREATE KEYSPACE IF NOT EXISTS peer WITH replication = {'class': 'SimpleStrategy', 'replication_factor': '1'};"
echo "Created keyspaces."

./cqlsh -e "CREATE TABLE IF NOT EXISTS user.user (username text, name text, surname text, password text, email text, registration_date text, salt text, PRIMARY KEY (username));"
./cqlsh -e "CREATE TABLE IF NOT EXISTS file.permission (username text, file_id text, owner text, write text, PRIMARY KEY (username, file_id));"
./cqlsh -e "CREATE TABLE IF NOT EXISTS file.file (file_id text, username text, file_name text, file_size text, file_size_compressed text, file_type text, upload_date text, PRIMARY KEY (username, file_id));"
./cqlsh -e "CREATE TABLE IF NOT EXISTS file.piece (file_id text, piece_order text, piece_size text, chunk_num text, replication_num text, chunk_peer text, chunk_hash text, transfer_length text, symbol_size text, source_blocks text, sub_blocks text, symbol_alignment text, PRIMARY KEY (file_id, piece_order));"
./cqlsh -e "CREATE TABLE IF NOT EXISTS peer.peer (peer_id text, username text, device_name text, country text, disk_size text, mount_point text, registration_date text, PRIMARY KEY (username, device_name));"
./cqlsh -e "CREATE TABLE IF NOT EXISTS peer.selection (peer_id text, disk_available text, last_liveness text, penalties text, ranking text, country text, mount_point text, pieces text, PRIMARY KEY (ranking, peer_id));"
./cqlsh -e "CREATE TABLE IF NOT EXISTS peer.performance (peer_id text, uptime_start text, uptime_end text, disk_read text, disk_write text, throughput text, PRIMARY KEY (peer_id, uptime_start));"
./cqlsh -e "CREATE TABLE IF NOT EXISTS peer.liveness (peer_id text, timestamp text, available text, PRIMARY KEY (peer_id));"
echo "Created tables."

#./cqlsh -e "CREATE INDEX ON user.user(password);"
#./cqlsh -e "CREATE INDEX ON file.permission(username);"
#./cqlsh -e "CREATE INDEX ON file.file(username);"
#./cqlsh -e "CREATE INDEX ON file.file(file_id);"
#./cqlsh -e "CREATE INDEX ON peer.peer(username);"
./cqlsh -e "CREATE INDEX ON peer.selection(last_liveness);"
#./cqlsh -e "CREATE INDEX ON peer.selection(ranking);"
echo "Created indexes."

# Insert sample data into the table
./cqlsh -e "INSERT INTO user.user (username, name, surname, password, email, salt, registration_date) 
    VALUES ('user', 'mario', 'rossi', '\$2b\$12\$RFowKwDoCSQLhqvOecQK5O3nX0hAyVIy2.VTYgglEM0WxDxm4Xfau', 'mario.rossi@email.com', '\$2b\$12\$RFowKwDoCSQLhqvOecQK5O', '2024-02-26 15:36:50');"
./cqlsh -e "INSERT INTO  user.user (username, password, email, salt, registration_date, name, surname) 
     VALUES ('user2', '\$2b\$12\$UWQ/mnAX5adJtoow05GLee72mdPtu0RWT4BhPm1LGY1DAi4pyTdda', 'giuseppe.verdi@email.com', '\$2b\$12\$UWQ/mnAX5adJtoow05GLee', '2024-02-26 17:51:49', 'giuseppe', 'verdi');"

./cqlsh -e "INSERT INTO peer.peer (disk_size, country, device_name, username, mount_point, registration_date, peer_id) 
     VALUES ('100000000', 'Italy', 'pc', 'user', '/home/user/saved_files', '2024-02-26 17:44:33', 'ed8273cf-af1e-4cc4-9e11-cd96e3a2f514') IF NOT EXISTS;"
./cqlsh -e "INSERT INTO peer.peer (peer_id, device_name, country, username, mount_point, disk_size, registration_date) 
     VALUES ('64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e', 'mobile', 'Italy', 'user', '/home/user/mobile', '20000000', '2024-02-26 17:46:52') IF NOT EXISTS;"
./cqlsh -e "INSERT INTO peer.peer (mount_point, disk_size, country, registration_date, peer_id, device_name, username) 
     VALUES ('/home/user/folder', '50000000', 'Italy', '2024-02-26 17:54:19', 'f654f087-0a1e-4c2b-9a86-cfdd1a836000', 'pc', 'user2') IF NOT EXISTS;"

./cqlsh -e "INSERT INTO peer.selection (last_liveness, penalties, ranking, peer_id, disk_available, mount_point, pieces, country) 
     VALUES ('1708965873', '0', '1', 'ed8273cf-af1e-4cc4-9e11-cd96e3a2f514', '100000000', '/home/user/saved_files', '[]', 'Italy');"
./cqlsh -e "INSERT INTO peer.selection (mount_point, pieces, last_liveness, penalties, ranking, disk_available, peer_id, country) 
     VALUES ('/home/user/mobile', '[]', '1708966012', '0', '1', '20000000', '64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e', 'Italy');"
./cqlsh -e "INSERT INTO peer.selection (last_liveness, penalties, country, mount_point, peer_id, disk_available, ranking, pieces) 
     VALUES ('1708966459', '0', 'Italy', '/home/user/folder', 'f654f087-0a1e-4c2b-9a86-cfdd1a836000', '50000000', '1', '[]');"

./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('ed8273cf-af1e-4cc4-9e11-cd96e3a2f514', '1708972142', '10', '10', '20', '1808972142');"
./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('ed8273cf-af1e-4cc4-9e11-cd96e3a2f514', '1808972143', '12', '15', '25', '1908972143');"
./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('ed8273cf-af1e-4cc4-9e11-cd96e3a2f514', '2008972144', '9', '11', '30', '2108972144');"

./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e', '1708972142', '30', '30', '50', '1808972142');"
./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e', '1808972143', '32', '31', '55', '1908972143');"
./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e', '2008972144', '26', '28', '45', '2108972144');"

./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('f654f087-0a1e-4c2b-9a86-cfdd1a836000', '1708972142', '50', '30', '20', '1808972142');"
./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('f654f087-0a1e-4c2b-9a86-cfdd1a836000', '1808972143', '48', '26', '18', '1908972143');"
./cqlsh -e "INSERT INTO peer.performance (peer_id, uptime_start, disk_read, disk_write, throughput, uptime_end) 
     VALUES ('f654f087-0a1e-4c2b-9a86-cfdd1a836000', '2008972144', '52', '32', '22', '2108972144');"

./cqlsh -e "INSERT INTO file.file (username, file_id, file_name, file_size, file_size_compressed, file_type, upload_date) 
     VALUES ('user', '15d82ac3-a6f6-4ca7-83e4-6cc3fb6f8715', 'debian-icon.png','4691', '4702','image/png', '2024-02-26 18:50:55') IF NOT EXISTS;"
./cqlsh -e "INSERT INTO file.piece (file_id, piece_order, chunk_num, chunk_peer, piece_size, replication_num, source_blocks, sub_blocks, symbol_alignment, symbol_size, transfer_length) 
     VALUES ('15d82ac3-a6f6-4ca7-83e4-6cc3fb6f8715', '0', '4', '{"chunk2": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk7": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk6": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk0": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk1": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk5": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk4": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk3": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e"}', '4691', '4', '1', '1', '8', '1488', '4691');"

./cqlsh -e "INSERT INTO file.file (username, file_id, file_name, file_size, file_size_compressed, file_type, upload_date)
     VALUES ('user', '53caa039-7862-4803-80ed-e6d4e08e680c', 'ubuntu-icon.png', '7008', '', '7019', 'image/png', '2024-02-26 19:08:14') IF NOT EXISTS;"
./cqlsh -e "INSERT INTO file.piece (file_id, piece_order, chunk_num, chunk_peer, piece_size, replication_num, source_blocks, sub_blocks, symbol_alignment, symbol_size, transfer_length)
     VALUES ('53caa039-7862-4803-80ed-e6d4e08e680c', '0', '5', '{"chunk0": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk7": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk8": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk2": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk6": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk1": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk4": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk5": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk3": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e", "chunk9": "64faa647-4af4-45d3-bbb2-0dd9c8bb3d4e"}', '7008', '5', '1', '1', '8', '1488', '7008');"

echo "Database populated successfully."
