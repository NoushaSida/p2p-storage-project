from pdfminer.pdfparser import PDFParser
from pdfminer.pdfdocument import PDFDocument
from pdfminer.pdftypes import resolve1
import json
import datetime
import os
import argparse
import parsing.parsing_utils as utils
import parsing.parsing_texts as texts

output_json_file_path = 'website/data.json'

italian_month_names = [
    "Gennaio", "Febbraio", "Marzo", "Aprile", "Maggio", "Giugno", 
    "Luglio", "Agosto", "Settembre", "Ottobre", "Novembre", "Dicembre"
]

def additional_data(pdf_file):
    current_date = datetime.datetime.now()
    if pdf_file["storia_evento_mese1"]:
        pdf_file["storia_evento_mese1"] = italian_month_names[int(pdf_file["storia_evento_mese1"]) - 1]
    if pdf_file["storia_evento_mese2"]:
        pdf_file["storia_evento_mese2"] = italian_month_names[int(pdf_file["storia_evento_mese2"]) - 1]
    if pdf_file["storia_evento_mese3"]:
        pdf_file["storia_evento_mese3"] = italian_month_names[int(pdf_file["storia_evento_mese3"]) - 1]
    if pdf_file["storia_evento_mese4"]:
        pdf_file["storia_evento_mese4"] = italian_month_names[int(pdf_file["storia_evento_mese4"]) - 1]
    if pdf_file["storia_evento_mese5"]:
        pdf_file["storia_evento_mese5"] = italian_month_names[int(pdf_file["storia_evento_mese5"]) - 1]
    if pdf_file["storia_evento_mese6"]:
        pdf_file["storia_evento_mese6"] = italian_month_names[int(pdf_file["storia_evento_mese6"]) - 1]

    if pdf_file["storia_evento_anno1"] and int(pdf_file['storia_evento_anno1']) < 1000:
        pdf_file["storia_evento_anno1"] = '20' + pdf_file["storia_evento_anno1"] 
    if pdf_file["storia_evento_anno2"] and int(pdf_file['storia_evento_anno2']) < 1000:
        pdf_file["storia_evento_anno2"] = '20' + pdf_file["storia_evento_anno2"]
    if pdf_file["storia_evento_anno3"] and int(pdf_file['storia_evento_anno3']) < 1000:
        pdf_file["storia_evento_anno3"] = '20' + pdf_file["storia_evento_anno3"] 
    if pdf_file["storia_evento_anno4"] and int(pdf_file['storia_evento_anno4']) < 1000:
        pdf_file["storia_evento_anno4"] = '20' + pdf_file["storia_evento_anno4"]
    if pdf_file["storia_evento_anno5"] and int(pdf_file['storia_evento_anno5']) < 1000:
        pdf_file["storia_evento_anno5"] = '20' + pdf_file["storia_evento_anno5"] 
    if pdf_file["storia_evento_anno6"] and int(pdf_file['storia_evento_anno6']) < 1000:
        pdf_file["storia_evento_anno6"] = '20' + pdf_file["storia_evento_anno6"] 

    if pdf_file['storia_evento_descrizione1']:
        pdf_file['storia_evento_titolo1'] = 'Evento1'
    if pdf_file['storia_evento_descrizione2']:
        pdf_file['storia_evento_titolo2'] = 'Evento2'
    if pdf_file['storia_evento_descrizione3']:
        pdf_file['storia_evento_titolo3'] = 'Evento3'
    if pdf_file['storia_evento_descrizione4']:
        pdf_file['storia_evento_titolo4'] = 'Evento4'
    if pdf_file['storia_evento_descrizione5']:
        pdf_file['storia_evento_titolo5'] = 'Evento5'
    if pdf_file['storia_evento_descrizione6']:
        pdf_file['storia_evento_titolo6'] = 'Evento6'
    
    if pdf_file['notizie1_data']:
        pdf_file['notizie1_giorno'] = pdf_file['notizie1_data'].split('-')[0]
        pdf_file['notizie1_mese'] = pdf_file['notizie1_data'].split('-')[1]
        pdf_file['notizie1_anno'] = pdf_file['notizie1_data'].split('-')[2]
    if pdf_file['notizie2_data']:
        pdf_file['notizie2_giorno'] = pdf_file['notizie2_data'].split('-')[0]
        pdf_file['notizie2_mese'] = pdf_file['notizie2_data'].split('-')[1]
        pdf_file['notizie2_anno'] = pdf_file['notizie2_data'].split('-')[2]

    if pdf_file['evento1_data_inizio']:
        pdf_file['evento1_giorno'] = pdf_file['evento1_data_inizio'].split('-')[0]
        pdf_file['evento1_mese'] = pdf_file['evento1_data_inizio'].split('-')[1]
        pdf_file['evento1_anno'] = pdf_file['evento1_data_inizio'].split('-')[2]
    if pdf_file['evento2_data_inizio']:
        pdf_file['evento2_giorno'] = pdf_file['evento2_data_inizio'].split('-')[0]
        pdf_file['evento2_mese'] = pdf_file['evento2_data_inizio'].split('-')[1]
        pdf_file['evento2_anno'] = pdf_file['evento2_data_inizio'].split('-')[2]
    if pdf_file['evento3_data_inizio']:
        pdf_file['evento3_giorno'] = pdf_file['evento3_data_inizio'].split('-')[0]
        pdf_file['evento3_mese'] = pdf_file['evento3_data_inizio'].split('-')[1]
        pdf_file['evento3_anno'] = pdf_file['evento3_data_inizio'].split('-')[2]
    if pdf_file['evento1_data_fine']:
        pdf_file['evento1_giorno_fine'] = pdf_file['evento1_data_fine'].split('-')[0]
        pdf_file['evento1_mese_fine'] = pdf_file['evento1_data_fine'].split('-')[1]
        pdf_file['evento1_anno_fine'] = pdf_file['evento1_data_fine'].split('-')[2]
    if pdf_file['evento2_data_fine']:
        pdf_file['evento2_giorno_fine'] = pdf_file['evento2_data_fine'].split('-')[0]
        pdf_file['evento2_mese_fine'] = pdf_file['evento2_data_fine'].split('-')[1]
        pdf_file['evento2_anno_fine'] = pdf_file['evento2_data_fine'].split('-')[2]
    if pdf_file['evento3_data_fine']:
        pdf_file['evento3_giorno_fine'] = pdf_file['evento3_data_fine'].split('-')[0]
        pdf_file['evento3_mese_fine'] = pdf_file['evento3_data_fine'].split('-')[1]
        pdf_file['evento3_anno_fine'] = pdf_file['evento3_data_fine'].split('-')[2]

    if pdf_file['luogo1_indirizzo']:
        pdf_file['luogo1_indirizzo_mappa'] = pdf_file['luogo1_indirizzo'] + ' ' + pdf_file['istituto_cap'] + ' '  + pdf_file['istituto_citta']
    if pdf_file['luogo2_indirizzo']:
        pdf_file['luogo2_indirizzo_mappa'] = pdf_file['luogo2_indirizzo'] + ' ' + pdf_file['istituto_cap']  + ' ' + pdf_file['istituto_citta']

    #pdf_file["storia_anno_sito"] = str(current_date.year)
    #pdf_file["storia_mese_sito"] = italian_month_names[current_date.month - 1]
    pdf_file["contenuto_pubblicato"] = str(current_date.day) + '.' + str(current_date.month) + '.' + str(current_date.year)  
    
    return pdf_file

def modify_custom_fields(pdf_file):
    pdf_file['istituto_statale'] = texts.texts_fixed['logo2'] if pdf_file['istituto_statale'] == '2' else texts.texts_fixed['logo1']
    pdf_file['email_pec'] = pdf_file['istituto_pec']
    pdf_file['luogo_sede_principale_mappa'] = pdf_file['istituto_indirizzo'] + ', ' + pdf_file['istituto_cap'] + ' ' + pdf_file['istituto_citta']
    pdf_file['numero_membri_area'] = str(int(pdf_file['numero_membri'])/int(pdf_file['numero_aree']))
    return pdf_file

def check_required_fields(pdf_file):
    mandatory_fields = ['istituto_citta', 'istituto_nome', 'istituto_statale', 'istituto_cap', 'istituto_indirizzo', 'email', 'istituto_pec',
     'istituto_desc_breve', 'istituto_cosa_fa', 'istituto_telefono', 'azienda_slogan', 'luogo1_nome', 'luogo1_indirizzo']

    empty_fields = []
    for field in mandatory_fields:
        print(pdf_file)
        if field in pdf_file:
            value = pdf_file[field]
            if value is None or (isinstance(value, str) and not value.strip()) or (isinstance(value, list) and not value):
                empty_fields.append(field)
    
    if empty_fields:
        print('The following fields are empty: ')
        for field in empty_fields:
            print('Empty field found ', field)
        raise ValueError('Empty fields found: ' + ', '.join(map(str, empty_fields)))

    # Luoghi
    utils.associated_fields_checks('luogo1_indirizzo', 'luogo1_nome', 'luogo1_orario', 'luogo1_mail', 'luogo1_telefono', 'luogo1_tipologia')
    utils.associated_fields_checks('luogo2_indirizzo', 'luogo1_nome', 'luogo2_orario', 'luogo2_mail', 'luogo2_telefono', 'luogo2_tipologia')
    utils.associated_fields_checks('luogo3_indirizzo', 'luogo3_nome', 'luogo3_orario', 'luogo3_mail', 'luogo3_telefono', 'luogo3_tipologia')
    utils.associated_fields_checks('luogo4_indirizzo', 'luogo4_nome', 'luogo4_orario', 'luogo4_mail', 'luogo4_telefono', 'luogo4_tipologia')
    
    # News
    utils.associated_fields_checks('notizie1_titolo', 'notizie1_autore', 'notizie1_desc_breve', 'notizie1_notizia', 'notizie1_argomento1')
    utils.associated_fields_checks('notizie2_titolo', 'notizie2_autore', 'notizie2_desc_breve', 'notizie2_notizia', 'notizie2_argomento1')
    utils.associated_fields_checks('notizie3_titolo', 'notizie3_autore', 'notizie3_desc_breve', 'notizie3_notizia', 'notizie3_argomento1')
    
    return pdf_file

def pdf_to_json(pdf_file):
    pdf_content = {}
    doc = PDFDocument(PDFParser(open(pdf_file, 'rb')))
    for i in resolve1(doc.catalog['AcroForm'])['Fields']:
        field = resolve1(i)
        key_b, value_b = field.get('T'), field.get('V')
        key = utils.decode_string(field, key_b)
        value = utils.decode_string(field, value_b)
        value = utils.format_document_text(key, value)
        #print('{0}: {1}'.format(key, value))
        pdf_content[key] = value
    return pdf_content

def main(pdf_file):
    json_data = check_required_fields(pdf_to_json(pdf_file))
    #json_data.update(texts.texts_fixed)
    json_data = modify_custom_fields(json_data)
    json_data = additional_data(json_data)

 #   for pdf_filename in os.listdir(folder_path):
 #       if pdf_filename.startswith("pdf-") and pdf_filename.endswith(".pdf"):
 #           json_data_add = pdf_to_json(os.path.join(folder_path, pdf_filename))
 #           json_data_add['prova'] = 'prova'
 #           json_data_add['istituto_tipologia'] = 'test'
 #           json_data_add.update(json_data)
 #           json_data = json_data_add
    
    with open(output_json_file_path, 'w') as output_file:
        json.dump(json_data, output_file, indent=4)
    print(f"JSON data saved to {output_json_file_path}")
