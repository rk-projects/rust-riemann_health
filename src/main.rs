extern crate riemann_client;
extern crate sys_info; 
extern crate libc;
extern crate protobuf;


use libc::timeval;
use sys_info::*;
use riemann_client::Client;
use riemann_client::proto::Event;
//use riemann_client::proto::Attribute;
//use protobuf::repeated::RepeatedField;


fn main() {
 
 let boottime = boottime();

 let _time :timeval;

 let boottime = match boottime {
     Ok(time) => time,
     Err(error) => {
         panic!("Problem detecting boottime : {:?} ", error)
     },
 };
println!("{}", boottime.tv_sec);

// let mut att = RepeatedField::<Attribute>::new();

// att.key = "boottime";
// att.value = boottime.tv_sec;

let mut client = Client::connect(&("localhost", 5555)).unwrap();

// client.event({
//     let mut event = Event::new();
//     event.set_service("boottime".to_string());
//     event.set_state("OK".to_string());
//     event.set_metric_d(boottime.tv_sec as f64);
//     event
// }).unwrap();

client.event({
    let mut event = Event::new();
    event.set_service("boottime".to_string());
    event.set_state("OK".to_string());
    //event.set_attributes(att);
    event.set_metric_d(boottime.tv_sec as f64);
    event
}).unwrap();



let disk_info = disk_info();

let _diskinfo :DiskInfo ;

let disk_info = match disk_info{
     Ok(diskinfo) => diskinfo,
     Err(error) => {
         panic!("Problem detecting boottime : {:?} ", error)
     },
 };
println!("{}", disk_info.total);
println!("{}", disk_info.free);


}
