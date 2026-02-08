import chardet
import re

pattern_for_text = [ 
    r'^documento_.*testo.*$',
    r'^notizia_.*testo.*$',
    r'^circolare_.*testo.*$',
    r'^storia_.*testo.*$',
    r'^progetto_.*testo.*$'
]

pattern_for_maps = r'\b[A-Za-z]\.\s'

def parse_address_for_maps(address):
    if address:
        return re.sub(pattern_for_maps, '', address).strip()
    return address

def format_document_text(key, value):
    for pattern in pattern_for_text:
        if re.match(pattern, key):
            value = re.sub(r'\.', '.</p><p>', value)
            if value.endswith('.</p><p>'):
                value = value[:-7]
            return value
    return value

def decode_string(field, s):
    if not s:
        return ''
    try:
        encoding = chardet.detect(s)
    except TypeError:
        print('Type error during decoding the key {0} with value {1}'.format(field.get('T'), field.get('V')))
        return s.name

    try:
        return s.decode(encoding['encoding']).strip()
    except UnicodeDecodeError:
        print('Unicode error during decoding the key {0} with value {1}'.format(field.get('T'), field.get('V')))
        return ''
    except AttributeError:
        print('Attribute error during decoding the key {0} with value {1}'.format(field.get('T'), field.get('V')))
        return s.name

def associated_fields_checks(main_field, *args):
    if main_field:
        for arg in args:
            if arg is None or (isinstance(arg, str) and not arg.strip()) or (isinstance(arg, list) and not arg):
                raise ValueError(main_field + ' is compiled but argument ' + arg + ' is empty.')  