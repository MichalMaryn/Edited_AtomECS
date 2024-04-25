extern crate csv;

use std::error::Error;
use csv::StringRecord;

#[derive(Debug)]
pub struct DataFrame {
   pos_x: Vec<f64>,
   pos_y: Vec<f64>,
   pos_z: Vec<f64>,
   vel_x: Vec<f64>,
   vel_y: Vec<f64>,
   vel_z: Vec<f64>,
 }


impl DataFrame {

    fn new() -> DataFrame {
        DataFrame {
            pos_x: Vec::new(),
            pos_y: Vec::new(),
            pos_z: Vec::new(),
            vel_x: Vec::new(),
            vel_y: Vec::new(),
            vel_z: Vec::new(),
        }
     }

     pub fn read_csv(filepath: &str, has_headers: bool) -> DataFrame {
         // Open file
         let file = std::fs::File::open(filepath).unwrap();
         let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);

         let mut data_frame = DataFrame::new();

         // push all the records
         for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
         }
         return data_frame;
      }

      fn push(&mut self, row: &csv::StringRecord) {
          // get pos_x
          self.pos_x.push(row[0].parse::<f64>().unwrap());
          // get pos_y
          self.pos_y.push(row[1].parse::<f64>().unwrap());
          // get pos_z
          self.pos_z.push(row[2].parse::<f64>().unwrap());
          // get vel_x
          self.vel_x.push(row[3].parse::<f64>().unwrap());
          // get vel_y
          self.vel_y.push(row[4].parse::<f64>().unwrap());
          // get vel_z
          self.vel_z.push(row[5].parse::<f64>().unwrap());
      }
}