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
 
 // boottime()

 let boottime = boottime();

 let _time :timeval;

 let boottime = match boottime {
     Ok(time) => time,
     Err(error) => {
         panic!("Problem detecting boottime : {:?} ", error)
     },
 };

let mut att = Attribute::new();
let mut att_vec :Vec<Attribute> = Vec::new();

att.set_key("boottime".to_string());
att.set_value(boottime.tv_sec.to_string());
att_vec.push(att);



// disk_info()
let disk_info = disk_info();

let _diskinfo :DiskInfo ;

let disk_info = match disk_info{
     Ok(diskinfo) => diskinfo,
     Err(error) => {
         panic!("Problem detecting disk info : {:?} ", error)
     },
 };

let mut disk_att = Attribute::new();
disk_att.set_key("total disk".to_string());
disk_att.set_value(disk_info.total.to_string());
att_vec.push(disk_att);

// memory 
let mem_info = mem_info();
let _meminfo :MemInfo;
let mem_info = match mem_info {
    Ok(meminfo) => meminfo,
    Err(error) => {
        panic!("Problem detecting mem info : {:?}", error)
    },
};

let mut mem_t_att = Attribute::new();
mem_t_att.set_key("memory-total".to_string());
mem_t_att.set_value(mem_info.total.to_string());

let mut mem_f_att = Attribute::new();
mem_f_att.set_key("memory-free".to_string());
mem_f_att.set_value(mem_info.free.to_string());

let mut mem_a_att = Attribute::new();
mem_a_att.set_key("memory-available".to_string());
mem_a_att.set_value(mem_info.avail.to_string());

let mut mem_b_att = Attribute::new();
mem_b_att.set_key("memory-buffer".to_string());
mem_b_att.set_value(mem_info.buffers.to_string());

let mut mem_st_att = Attribute::new();
mem_st_att.set_key("swap-total".to_string());
mem_st_att.set_value(mem_info.swap_total.to_string());

let mut mem_sf_att = Attribute::new();
mem_sf_att.set_key("swap-free".to_string());
mem_sf_att.set_value(mem_info.swap_free.to_string());

let mut mem_c_att = Attribute::new();
mem_c_att.set_key("cached".to_string());
mem_c_att.set_value(mem_info.cached.to_string());

att_vec.push(mem_t_att);
att_vec.push(mem_f_att);
att_vec.push(mem_a_att);
att_vec.push(mem_b_att);
att_vec.push(mem_st_att);
att_vec.push(mem_sf_att);



// proc total

let proc_total = proc_total().unwrap();

let mut proc_att = Attribute::new();
proc_att.set_key("proc-total".to_string());
proc_att.set_value(proc_total.to_string());
att_vec.push(proc_att);

// load average

let load_avg = loadavg().unwrap();
let mut load_att = Attribute::new();
load_att.set_key("load-avg".to_string());
load_att.set_value(load_avg.five.to_string());
att_vec.push(load_att);



let mut client = Client::connect(&("localhost", 5555)).unwrap();

client.event({
    let mut event = Event::new();
    event.set_service("riemann-health".to_string());
    event.set_state("OK".to_string());
    event.set_attributes(protobuf::RepeatedField::from_vec(att_vec));
    event.set_metric_d(boottime.tv_sec as f64);
    event
}).unwrap();



}
