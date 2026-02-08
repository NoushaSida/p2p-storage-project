const Mustache = require('mustache')
const fs = require('fs')

var max_element = 10 /* max num of documents/news/etc. accetped for each type */
var data = JSON.parse(fs.readFileSync('data.json'))
var page_home = fs.readFileSync("./templates/home_tpl.html", 'utf8')
fs.writeFileSync('./website/home.html', Mustache.render(page_home, data))

/*Sezione Scuola*/
var page_scuola = fs.readFileSync("./templates/organizzazione_tpl.html", 'utf8')
var page_presentazione = fs.readFileSync("./templates/presentazione_tpl.html", 'utf8')
//var page_documenti = fs.readFileSync("./templates/documenti_tpl.html", 'utf8')
//var page_scheda_organizzazione = fs.readFileSync("./templates/scheda-organizzazione_tpl.html", 'utf8')
//var page_persone = fs.readFileSync("./templates/sezione-persone_tpl.html", 'utf8')
var page_numeri_scuola = fs.readFileSync("./templates/numeri_tpl.html", 'utf8')
var page_storia_scuola = fs.readFileSync("./templates/storia_tpl.html", 'utf8')
fs.writeFileSync('./website/organizzazione.html', Mustache.render(page_scuola, data))
fs.writeFileSync('./website/presentazione.html', Mustache.render(page_presentazione, data))
/*fs.writeFileSync('./website/documenti.html', Mustache.render(page_documenti, data))
fs.writeFileSync('./website/scheda-organizzazione.html', Mustache.render(page_scheda_organizzazione, data))
fs.writeFileSync('./website/sezione-persone.html', Mustache.render(page_persone, data))*/
fs.writeFileSync('./website/numeri.html', Mustache.render(page_numeri_scuola, data))
fs.writeFileSync('./website/storia.html', Mustache.render(page_storia_scuola, data))

/*Pagine dei Luoghi*/
var page_luoghi = fs.readFileSync("./templates/luoghi_tpl.html", 'utf8')
fs.writeFileSync('./website/luoghi.html', Mustache.render(page_luoghi, data))

/*Pagina Sede Principale*/
/*data['luogo_tipologia'] = 'Sede principale'
data['luogo_indirizzo'] = data['indirizzo']
data['luogo_indirizzo_mappa'] = data['luogo_sede_principale_mappa']
data['luogo_descrizione_breve'] = data['descrizione_breve_sede_principale']
data['luogo_descrizione'] = data['descrizione_sede_principale']
data['luogo_email'] = data['email']
data['luogo_telefono'] = data['istituto_telefono']
data['luogo_sede_di'] = data['istituto_tipologia'] + " " + data['istituto_nome']
*/
var page_luoghi_sede_principale = fs.readFileSync("./templates/scheda-luogo-pagina_tpl.html", 'utf8')
fs.writeFileSync('./website/scheda-luogo-sede-principale.html', Mustache.render(page_luoghi_sede_principale, data))

/*Pagina Segreteria*/
/*data['luogo'] = 'Segreteria'
data['luogo_indirizzo'] = data['luogo_segreteria']
data['luogo_indirizzo_mappa'] = data['luogo_segreteria_mappa']
data['luogo_descrizione_breve'] = data['descrizione_breve_segreteria']
data['luogo_descrizione'] = data['descrizione_segreteria']
data['luogo_email'] = data['segreteria_email']
data['luogo_telefono'] = data['segreteria_telefono']
var page_luoghi_segreteria = fs.readFileSync("./templates/scheda-luogo-pagina_tpl.html", 'utf8')
fs.writeFileSync('./website/scheda-luogo-segreteria.html', Mustache.render(page_luoghi_segreteria, data))*/
for (let i = 1; i < max_element; i++) {
    if (data['luogo' + i + '_nome'] && data['luogo' + i + '_indirizzo']) {
        data['luogo_nome'] = data['luogo' + i + '_nome']
        data['luogo_indirizzo'] = data['luogo' + i + '_indirizzo']
        data['luogo_indirizzo_mappa'] = data['luogo' + i + '_indirizzo_mappa']
        data['luogo_descrizione_breve'] = data['luogo' + i + '_desc_breve']
        data['luogo_descrizione'] = data['luogo' + i + '_desc_estesa']
        data['luogo_mail'] = data['luogo' + i + '_mail']
        data['luogo_telefono'] = data['luogo' + i + '_telefono']
        data['luogo_tipologia'] = data['luogo' + i + '_tipologia']
        data['luogo_orario'] = data['luogo' + i + '_orario']
        data['luogo_mod_accesso'] = data['luogo' + i + '_mod_accesso']
        var percorsi_studio = fs.readFileSync("./templates/scheda-luogo-pagina_tpl.html", 'utf8')
        fs.writeFileSync('./website/scheda-luogo-pagina' + i + '.html', Mustache.render(percorsi_studio, data))
    } else {
        break;
    }
}

/*Sezione Servizi*/
/*var page_servizi = fs.readFileSync("./templates/sezione-servizi_tpl.html", 'utf8');
var page_servizio_personale_scolastico = fs.readFileSync("./templates/servizio-personale-scolastico_tpl.html", 'utf8')
var page_servizio_famiglia_studenti = fs.readFileSync("./templates/servizio-famiglie-studenti_tpl.html", 'utf8')
var page_servizio_percorsi_studio = fs.readFileSync("./templates/servizio-percorsi-studio_tpl.html", 'utf8')
fs.writeFileSync('./website/sezione-servizi.html', Mustache.render(page_servizi, data))
fs.writeFileSync('./website/servizio-personale-scolastico.html', Mustache.render(page_servizio_personale_scolastico, data))
fs.writeFileSync('./website/servizio-famiglie-studenti.html', Mustache.render(page_servizio_famiglia_studenti, data))
fs.writeFileSync('./website/servizio-percorsi-studio.html', Mustache.render(page_servizio_percorsi_studio, data))
*/

/*Pagine Personale Scolastico*/
/*if (data['personale_scolastico_libri_di_testo']) {
    var personale_scolastico_libri_di_testo = fs.readFileSync("./templates/servizio-personale-scolastico-libri-di-testo_tpl.html", 'utf8')
    fs.writeFileSync('./website/servizio-personale-scolastico-libri-di-testo.html', Mustache.render(personale_scolastico_libri_di_testo, data))
}*/

/*Pagine Famiglie Studenti*/
/*var famiglie_studenti_registro_eletronico = fs.readFileSync("./templates/servizio-famiglie-studenti-registro-elettronico_tpl.html", 'utf8')
fs.writeFileSync('./website/servizio-famiglie-studenti-registro-elettronico.html', Mustache.render(famiglie_studenti_registro_eletronico, data))
var famiglie_studenti_ricevimento_genitori = fs.readFileSync("./templates/servizio-famiglie-studenti-ricevimento-genitori_tpl.html", 'utf8')
fs.writeFileSync('./website/servizio-famiglie-studenti-ricevimento-genitori.html', Mustache.render(famiglie_studenti_ricevimento_genitori, data))
*/

/*Pagine Percorsi di Studio*/
/*for (let i = 1; i < max_element; i++) {
    if (data['percorso_studio' + i + '_titolo'] && data['percorso_studio' + i + '_grado'] && data['percorso_studio' + i + '_descrizione']) {
        data['percorso_studio_titolo'] = data['percorso_studio' + i + '_titolo']
        data['percorso_studio_grado'] = data['percorso_studio' + i + '_grado']
        data['percorso_studio_descrizione'] = data['percorso_studio' + i + '_descrizione']
        data['percorso_studio_come_accedere'] = data['percorso_studio' + i + '_come_accedere']
        var percorsi_studio = fs.readFileSync("./templates/servizio-percorsi-studio-pagina_tpl.html", 'utf8')
        fs.writeFileSync('./website/servizio-percorsi-studio-pagina' + i + '.html', Mustache.render(percorsi_studio, data))
    } else {
        break;
    }
}*/

/*Sezione Novità*/
var page_notizie = fs.readFileSync("./templates/sezione-notizie_tpl.html", 'utf8')
var page_le_notizie = fs.readFileSync("./templates/novita-notizie_tpl.html", 'utf8')
//var page_le_circolari = fs.readFileSync("./templates/novita-circolari_tpl.html", 'utf8')
var page_calendario_eventi = fs.readFileSync("./templates/archivio-eventi_tpl.html", 'utf8')
fs.writeFileSync('./website/sezione-notizie.html', Mustache.render(page_notizie, data))
fs.writeFileSync('./website/novita-notizie.html', Mustache.render(page_le_notizie, data))
//fs.writeFileSync('./website/novita-circolari.html', Mustache.render(page_le_circolari, data))
fs.writeFileSync('./website/archivio-eventi.html', Mustache.render(page_calendario_eventi, data))

/*Sezione Didattica*/
/*var page_didattica = fs.readFileSync("./templates/didattica_tpl.html", 'utf8')
var page_offerta_formativa = fs.readFileSync("./templates/didattica-offertaformativa_tpl.html", 'utf8')
var page_schede_didattiche = fs.readFileSync("./templates/schede-didattiche_tpl.html", 'utf8')
var page_progetti_classi = fs.readFileSync("./templates/progetti-classi_tpl.html", 'utf8')
fs.writeFileSync('./website/didattica.html', Mustache.render(page_didattica, data))
fs.writeFileSync('./website/didattica-offertaformativa.html', Mustache.render(page_offerta_formativa, data))
fs.writeFileSync('./website/schede-didattiche.html', Mustache.render(page_schede_didattiche, data))
fs.writeFileSync('./website/progetti-classi.html', Mustache.render(page_progetti_classi, data))
*/

/*Sezioni Extra*/
var page_privacy = fs.readFileSync("./templates/privacy_tpl.html", 'utf8')
var page_cookie = fs.readFileSync("./templates/cookie-policy_tpl.html", 'utf8')
var page_mappa_sito = fs.readFileSync("./templates/mappa-sito_tpl.html", 'utf8')
fs.writeFileSync('./website/privacy.html', Mustache.render(page_privacy, data))
fs.writeFileSync('./website/cookie-policy.html', Mustache.render(page_cookie, data))
fs.writeFileSync('./website/mappa-sito.html', Mustache.render(page_mappa_sito, data))

/*Pagine Notizie*/
for (let i = 1; i < max_element; i++) {
    if (data['notizie' + i + '_titolo'] && data['notizie' + i + '_desc_breve'] && data['notizie' + i + '_notizia']) {
        data['notizie_titolo'] = data['notizie' + i + '_titolo']
        data['notizie_desc_breve'] = data['notizie' + i + '_desc_breve']
        data['notizie_giorno'] = data['notizie' + i + '_giorno']
        data['notizie_mese'] = data['notizie' + i + '_mese']
        data['notizie_anno'] = data['notizie' + i + '_anno']
        data['notizie_notizia'] = data['notizie' + i + '_notizia']
        data['notizie_argomento1'] = data['notizie' + i + '_argomento1']
        data['notizie_argomento2'] = data['notizie' + i + '_argomento2']
        data['notizie_autore'] = data['notizie' + i + '_autore']
        var notizie_pagina = fs.readFileSync("./templates/novita-notizia-pagina_tpl.html", 'utf8')
        fs.writeFileSync('./website/novita-notizia-pagina' + i + '.html', Mustache.render(notizie_pagina, data))
    }
    else {
        break
    }
}

/*Pagine Eventi*/
for (let i = 1; i < max_element; i++) {
    if (data['evento' + i + '_titolo'] && data['evento' + i + '_testo']) {
        data['evento_titolo'] = data['evento' + i + '_titolo']
        data['evento_sottotitolo'] = data['evento' + i + '_sottotitolo']
        data['evento_giorno'] = data['evento' + i + '_giorno']
        data['evento_mese'] = data['evento' + i + '_mese']
        data['evento_anno'] = data['evento' + i + '_anno']
        data['evento_data_inizio'] = data['evento' + i + '_data_inizio']
        data['evento_giorno_fine'] = data['evento' + i + '_giorno_fine']
        data['evento_mese_fine'] = data['evento' + i + '_mese_fine']
        data['evento_anno_fine'] = data['evento' + i + '_anno_fine']
        data['evento_data_fine'] = data['evento' + i + '_data_fine']
        data['evento_inizio_ore'] = data['evento' + i + '_inizio_ore']
        data['evento_fine_ore'] = data['evento' + i + '_fine_ore']
        data['evento_testo'] = data['evento' + i + '_testo']
        data['evento_argomento1'] = data['evento' + i + '_argomento1']
        data['evento_argomento2'] = data['evento' + i + '_argomento2']
        data['evento_argomento3'] = data['evento' + i + '_argomento3']
        data['evento_argomento4'] = data['evento' + i + '_argomento4']
        data['evento_oggetto'] = data['evento' + i + '_oggetto']
        data['evento_luoogo'] = data['evento' + i + '_luogo']
        data['evento_tipo'] = data['evento' + i + '_tipo']
        data['evento_tipologia'] = data['evento' + i + '_tipologia']
        data['evento_destinatari'] = data['evento' + i + '_destinatari']
        var evento_pagina = fs.readFileSync("./templates/novita-evento-pagina_tpl.html", 'utf8')
        fs.writeFileSync('./website/novita-evento-pagina' + i + '.html', Mustache.render(evento_pagina, data))
    }
    else {
        break
    }
}

/*Pagine Circolari*/
/*for (let i = 1; i < max_element; i++) {
    if (data['circolare_titolo' + i] && data['circolare_testo' + i]) {
        data['circolare_titolo'] = data['circolare_titolo' + i]
        data['circolare_sottotitolo'] = data['circolare_sottotitolo' + i]
        data['circolare_testo'] = data['circolare_testo' + i]
        data['circolare_numero'] = data['circolare_numero' + i]
        var circolari_pagina = fs.readFileSync("./templates/novita-circolare-pagina_tpl.html", 'utf8')
        fs.writeFileSync('./website/novita-circolare-pagina.html', Mustache.render(circolari_pagina, data))
    }
    else {
        break
    }
}
*/
/*Pagine Progetti*/
/*
for (let i = 1; i < max_element; i++) {
    if (data['progetto' + i + '_titolo'] && data['progetto' + i + '_descrizione']) {
        data['progetto_titolo'] = data['progetto' + i + '_titolo']
        data['progetto_sottotitolo'] = data['progetto' + i + '_sottotitolo']
        data['progetto_testo'] = data['progetto' + i + '_descrizione']
        data['progetto_inizio'] = data['progetto' + i + '_inizio']
        data['progetto_fine'] = data['progetto' + i + '_fine']
        var progetto_pagina = fs.readFileSync("./templates/scheda-progetto-pagina_tpl.html", 'utf8')
        fs.writeFileSync('./website/scheda-progetto-pagina'+ i + '.html', Mustache.render(progetto_pagina, data))
    }
    else {
        break
    }
}*/

/*Pagine Schede Didattiche*/
/*for (let i = 1; i < max_element; i++) {
    if (data['scheda_didattica'+ i + '_titolo']) {
        data['scheda_didattica_titolo'] = data['scheda_didattica' + i + '_titolo']
        data['scheda_didattica_obiettivi'] = data['scheda_didattica' + i + '_obiettivi']
        for (let j = 1; j < max_element; j++) {
            if (data['scheda_didattica' + i + '_descrizione_attivita' + j]) {
                data['scheda_didattica_descrizione_attivita' + j] = data['scheda_didattica' + i + '_descrizione_attivita' + j]
            } else {
                break
            }
        }
        var scheda_didattica = fs.readFileSync("./templates/scheda-didattica-pagina_tpl.html", 'utf8')
        fs.writeFileSync('./website/scheda-didattica-pagina' + i + '.html', Mustache.render(scheda_didattica, data))
    }
    else {
        break
    }
}
*/
/*Pagine Schede Documenti*/

/*Modulistica*/
/*for (let i = 1; i < max_element; i++) {
    if (data['documento_modulistica' + i + '_titolo'] && data['documento_modulistica' + i + '_testo']) {
        data['documento_modulistica_titolo'] = data['documento_modulistica' + i + '_titolo']
        data['documento_modulistica_sottotitolo'] = data['documento_modulistica' + i + '_sottotitolo']
        data['documento_modulistica_testo'] = data['documento_modulistica' + i + '_testo']
        var scheda_documento_modulistica = fs.readFileSync("./templates/scheda-documento-modulistica_tpl.html", 'utf8')
        fs.writeFileSync('./website/scheda-documento-modulistica' + i + '.html', Mustache.render(scheda_documento_modulistica, data))
    }
    else {
        break
    }
}*/

/*Regolamenti*/
/*for (let i = 1; i < max_element; i++) {
    if (data['documento_regolamenti' + i + '_titolo'] && data['documento_regolamenti' + i + '_testo']) {
        data['documento_regolamenti_titolo'] = data['documento_regolamenti' + i + '_titolo']
        data['documento_regolamenti_sottotitolo'] = data['documento_regolamenti' + i + '_sottotitolo']
        data['documento_regolamenti_testo'] = data['documento_regolamenti' + i + '_testo']
        var scheda_documento_regolamento = fs.readFileSync("./templates/scheda-documento-regolamento_tpl.html", 'utf8')
        fs.writeFileSync('./website/scheda-documento-regolamento' + i + '.html', Mustache.render(scheda_documento_regolamento, data))
    }
    else {
        break
    }
}*/

/*Documenti Generici*/
/*for (let i = 1; i < max_element; i++) {
        if (data['documento_documenti_generici' + i + '_titolo'] && data['documento_documenti_generici' + i + '_testo']) {
            data['documento_documenti_generici_titolo'] = data['documento_documenti_generici' + i + '_titolo']
            data['documento_documenti_generici_sottotitolo'] = data['documento_documenti_generici' + i + '_sottotitolo']
            data['documento_documenti_generici_testo'] = data['documento_documenti_generici' + i + '_testo']
        var scheda_documento_generico = fs.readFileSync("./templates/scheda-documento-generico_tpl.html", 'utf8')
        fs.writeFileSync('./website/scheda-documento-generico' + i + '.html', Mustache.render(scheda_documento_generico, data))
    }
    else {
        break
    }
}*/

/*Pagine Schede Persone*/
/*var scheda_persona_dirigente = fs.readFileSync("./templates/scheda-persona-dirigente_tpl.html", 'utf8')
var scheda_persona_segreteria = fs.readFileSync("./templates/scheda-persona-segreteria_tpl.html", 'utf8')
fs.writeFileSync('./website/scheda-persona-dirigente.html', Mustache.render(scheda_persona_dirigente, data))
fs.writeFileSync('./website/scheda-persona-segreteria.html', Mustache.render(scheda_persona_segreteria, data))
*/