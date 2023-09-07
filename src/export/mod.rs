use std::fs;

use crate::DBManage;

pub fn csv_export(dbmanage: &DBManage, exportfilename: &String) {
    let hfile = fs::File::create(exportfilename).unwrap();

    let mut wtr = csv::Writer::from_writer(hfile);
    for (i, record) in dbmanage.db.iter().enumerate(){
        wtr.write_record(vec![&i.to_string(), &record.title, &record.username, &record.password, &record.notes]).expect("Ops... (*ﾟ∀ﾟ)っ［.+:｡☆Good Night☆.+:｡］");
    }
}