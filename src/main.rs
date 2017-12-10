extern crate riemann_client;
extern crate sys_info; 
extern crate libc;
extern crate protobuf;


use libc::timeval;
use sys_info::*;
use riemann_client::Client;
use riemann_client::proto::Event;
use riemann_client::proto::Attribute;
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

let mut att = Attribute::new();
let mut att_vec :Vec<Attribute> = Vec::new();

att.set_key("boottime".to_string());
att.set_value(boottime.tv_sec.to_string());
att_vec.push(att);


let mut client = Client::connect(&("localhost", 5555)).unwrap();

// client.event({
//     let mut event = Event::new();
//     event.set_service("boottime".to_string());
//     event.set_state("OK".to_string());
//     event.set_metric_d(boottime.tv_sec as f64);
//     event
// }).unwrap();



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

let mut disk_vec :Vec<Attribute> = Vec::new();
let mut disk_att = Attribute::new();
disk_att.set_key("total disk".to_string());
disk_att.set_value(disk_info.total.to_string());
att_vec.push(disk_att);


client.event({
    let mut event = Event::new();
    event.set_service("boottime".to_string());
    event.set_state("OK".to_string());
    event.set_attributes(protobuf::RepeatedField::from_vec(att_vec));
    event.set_metric_d(boottime.tv_sec as f64);
    event
}).unwrap();



}
