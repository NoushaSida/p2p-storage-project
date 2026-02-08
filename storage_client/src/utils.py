import string
import random
import constants as c

def generate_random_pw(length: int=16) -> str:
    """
    Generates a random password.

    Parameters
    ----------
    length: int
        The length of the returned password.
    Returns
    -------
    str
        The randomly generated password.
    """
    letters = string.ascii_letters + string.digits
    return ''.join(random.choice(letters) for i in range(length)).replace(' ','')

def get_formatted_size(size):
    size = int(size)
    unit = 'B'
    if size >= c.GB:
        size = int(size/c.GB)
        unit = c.GB_UNIT
    elif size >= c.MB:
        size = int(size/c.MB)
        unit = c.MB_UNIT
    elif size >= c.KB:
        size = int(size/c.KB)
        unit = c.KB_UNIT
    return size, unit

def get_device_name(device_id, devices):
    for device in devices:
        if device['peer_id'] == device_id:
            return device['device_name']
    return 'Unknown'