extern crate getopts;
extern crate protobuf;
extern crate riemann_client;
extern crate sys_info;

use sys_info::*;
use std::env;
use riemann_client::Client;
use riemann_client::proto::Event;
use riemann_client::proto::Attribute;
use std::{thread, time};
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
    std::process::exit(0);
}

fn main() {
    let ten_millis = time::Duration::from_millis(1000);
    loop {
        // delay events by one second
        thread::sleep(ten_millis);

        // using boottime() function from sys_info
        let boottime = boottime().unwrap();

        // att stands for attribute for boot time
        let mut att = Attribute::new();

        // att_vec is attribute vector to which all attributes are pushed.
        let mut att_vec: Vec<Attribute> = Vec::new();

        att.set_key("boottime".to_string());
        att.set_value(boottime.tv_sec.to_string());
        att_vec.push(att);

        // using function disk_info() from sys_info
        let disk_info = disk_info().unwrap();
        let mut disk_att = Attribute::new();

        disk_att.set_key("total disk".to_string());
        disk_att.set_value(disk_info.total.to_string());
        att_vec.push(disk_att);

        // using function mem_info() from sys_info
        let mem_info = mem_info().unwrap();

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

        // using function proc_total() from sys_info
        let proc_total = proc_total().unwrap();

        let mut proc_att = Attribute::new();
        proc_att.set_key("proc-total".to_string());
        proc_att.set_value(proc_total.to_string());
        att_vec.push(proc_att);

        // using function loadavg() from sys_info
        let load_avg = loadavg().unwrap();

        let mut load_att = Attribute::new();
        load_att.set_key("load-avg".to_string());
        load_att.set_value(load_avg.five.to_string());
        att_vec.push(load_att);

        let args: Vec<String> = env::args().collect();

        let program = args[0].clone();
        let mut opts = Options::new();
        opts.optopt(
            "c",
            "connection",
            "connection string to riemann server",
            "connection",
        );
        opts.optflag("h", "help", "print help menu");
        //opts.optopt("p", "port", "port of riemann server", "port");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(error) => {
                panic!("Unable to parse arguments {}", error.to_string());
            }
        };
        if matches.opt_present("h") {
            print_usage(&program, opts);
        }

        if !matches.opt_present("c") {
            println!("Kindly provide connection string with -c <hostname>:<port>");
            std::process::exit(2);
        }
        let hostname = matches.opt_str("c").unwrap();

        let mut client = Client::connect(&(&hostname.to_owned().to_string()[..])).unwrap();

        client
            .event({
                let mut event = Event::new();
                event.set_service("riemann-health".to_string());
                event.set_state("OK".to_string());
                event.set_attributes(protobuf::RepeatedField::from_vec(att_vec));
                event.set_metric_d(boottime.tv_sec as f64);
                event
            })
            .unwrap();
    }
}
