

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, MutexGuard};

use rusqlite::{params, Connection, Error, Result, Row, Rows, Statement};

use tabled::tables::*;


mod server;
mod fields;
mod templates;
mod senders;
mod utils;
mod cli;
mod sql;

use server::NetflowServer;
use crate::cli::*;
// use crate::utils::*;
// use crate::senders::*;
use crate::sql::*;




fn main() {
    
    //secure the db access for multi-thread use
    let mut db_conn_cli: std::sync::Arc<Mutex<Connection>>  = Arc::new(Mutex::new(setup_db(ConnType::InFile)));
    let db_conn_srv: std::sync::Arc<Mutex<Connection>>  = Arc::clone(&db_conn_cli);


    let server_thread = thread::spawn(move || {
        let mut netflow_server = NetflowServer::new("10.0.0.40:2055", db_conn_srv);
        netflow_server.run();
    });


    loop {

       std::thread::sleep(Duration::from_secs(5));
        // {
        //     let mut conn: MutexGuard<Connection> = db_conn_cli.lock().unwrap();
        //     let mut stmt: rusqlite::Statement = conn.prepare("SELECT * FROM senders")
        //         .expect("Unable to prepare query");

        //     let mut rows = stmt.query([])
        //         .expect("Unable to query rows");

        //    while let Some(row) = rows.next().expect("no more rows") {
        //       let ip_from_db: String = row.get(0).expect("Unable to open column 0");
        //       //println!("ip_from_db is {ip_from_db}");
        //     }
        // }
        clear_console();
        let flow_table = get_all_flows_from_sender(&mut db_conn_cli);
        println!("{flow_table}");
  
    }
    
}



            //this works but i will rty to use query instead
            // let mut rows = stmt.query_map([], |row| {
            //     let ip_str: String = row.get(0)?;
            //     Ok(ip_str)
            // }).expect("Unable to query rows");
            
            // for row in rows {
            //     match row {
            //         Ok(ip_str) => { 
            //             println!("IP in row is {ip_str}");
            //         },
            //         Err(err) => eprintln!("Error in row - {err}")

            //     }
            // }