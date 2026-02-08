# Publisher
KEY_EXPR_USER_SIGNUP = 'storage/user/signup/'
KEY_EXPR_USER_LOGIN = 'storage/user/login/'
KEY_EXPR_PEER_SIGNUP = 'storage/peer/signup/'
KEY_EXPR_PEER_GET = 'storage/peer/get/'
KEY_EXPR_FILE_LIST = 'storage/file/list/'
KEY_EXPR_FILE_UPLOAD = 'storage/file/upload/'
KEY_EXPR_FILE_UPLOAD_CLOUD = 'storage/file/upload_cloud/'
KEY_EXPR_FILE_GET = 'storage/file/get/'
KEY_EXPR_FILE_DELETE = 'storage/file/delete/'
KEY_EXPR_METRICS_PUT = "storage/metrics/put/"
KEY_EXPR_METRICS_GET = "storage/metrics/get/"
KEY_EXPR_LIVENESS_PUT = "storage/liveness/"
KEY_EXPR_PERMISSION_PUT = "storage/permission/put/"

# Subscriber
KEY_EXPR_FILE_DISTRIBUTION = "storage/file/distribution/"

FILES_FOLDER = "./files/"
ROUTER = 'tcp/172.22.209.161:7447'

OK = 'OK'
KO = 'KO'
TRUE = 'true'
FALSE = 'false'

DISK_SIZE_MB_MIN = 1     #1MB
DISK_SIZE_MB_MAX = 1000  #1GB

KB = 1000
MB = 1000000
GB = 1000000000

KB_UNIT = 'KB'
MB_UNIT = 'MB'
GB_UNIT = 'GB'

STORAGE_TYPE = ['ssd', 'hdd']
COUNTRIES = [
    'Afghanistan', 'Albania', 'Algeria', 'Andorra', 'Angola', 'Antigua and Barbuda',
    'Argentina', 'Armenia', 'Australia', 'Austria', 'Azerbaijan', 'Bahamas', 'Bahrain',
    'Bangladesh', 'Barbados', 'Belarus', 'Belgium', 'Belize', 'Benin', 'Bhutan', 'Bolivia',
    'Bosnia and Herzegovina', 'Botswana', 'Brazil', 'Brunei', 'Bulgaria', 'Burkina Faso',
    'Burundi', 'Cabo Verde', 'Cambodia', 'Cameroon', 'Canada', 'Central African Republic',
    'Chad', 'Chile', 'China', 'Colombia', 'Comoros', 'Congo', 'Costa Rica', 'Cote d\'Ivoire',
    'Croatia', 'Cuba', 'Cyprus', 'Czechia', 'Denmark', 'Djibouti', 'Dominica', 'Dominican Republic',
    'Ecuador', 'Egypt', 'El Salvador', 'Equatorial Guinea', 'Eritrea', 'Estonia', 'Eswatini',
    'Ethiopia', 'Fiji', 'Finland', 'France', 'Gabon', 'Gambia', 'Georgia', 'Germany', 'Ghana',
    'Greece', 'Grenada', 'Guatemala', 'Guinea', 'Guinea-Bissau', 'Guyana', 'Haiti', 'Honduras',
    'Hungary', 'Iceland', 'India', 'Indonesia', 'Iran', 'Iraq', 'Ireland', 'Israel', 'Italy',
    'Jamaica', 'Japan', 'Jordan', 'Kazakhstan', 'Kenya', 'Kiribati', 'Korea (North)', 'Korea (South)',
    'Kosovo', 'Kuwait', 'Kyrgyzstan', 'Laos', 'Latvia', 'Lebanon', 'Lesotho', 'Liberia', 'Libya',
    'Liechtenstein', 'Lithuania', 'Luxembourg', 'Madagascar', 'Malawi', 'Malaysia', 'Maldives', 'Mali',
    'Malta', 'Marshall Islands', 'Mauritania', 'Mauritius', 'Mexico', 'Micronesia', 'Moldova', 'Monaco',
    'Mongolia', 'Montenegro', 'Morocco', 'Mozambique', 'Myanmar', 'Namibia', 'Nauru', 'Nepal',
    'Netherlands', 'New Zealand', 'Nicaragua', 'Niger', 'Nigeria', 'North Macedonia', 'Norway', 'Oman',
    'Pakistan', 'Palau', 'Panama', 'Papua New Guinea', 'Paraguay', 'Peru', 'Philippines', 'Poland',
    'Portugal', 'Qatar', 'Romania', 'Russia', 'Rwanda', 'Saint Kitts and Nevis', 'Saint Lucia',
    'Saint Vincent and the Grenadines', 'Samoa', 'San Marino', 'Sao Tome and Principe', 'Saudi Arabia',
    'Senegal', 'Serbia', 'Seychelles', 'Sierra Leone', 'Singapore', 'Slovakia', 'Slovenia', 'Solomon Islands',
    'Somalia', 'South Africa', 'South Sudan', 'Spain', 'Sri Lanka', 'Sudan', 'Suriname', 'Sweden',
    'Switzerland', 'Syria', 'Taiwan', 'Tajikistan', 'Tanzania', 'Thailand', 'Timor-Leste', 'Togo',
    'Tonga', 'Trinidad and Tobago', 'Tunisia', 'Turkey', 'Turkmenistan', 'Tuvalu', 'Uganda', 'Ukraine',
    'United Arab Emirates', 'United Kingdom', 'United States', 'Uruguay', 'Uzbekistan', 'Vanuatu',
    'Vatican City', 'Venezuela', 'Vietnam', 'Yemen', 'Zambia', 'Zimbabwe'
]
