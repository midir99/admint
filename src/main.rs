// Author: Jorge Alarcon Alvarez
// Email:  jorge4larcon@gmail.com
// This module is where the program parses the commands, validate their values and starts the program.

extern crate clap;
extern crate regex;

use admint::ipparser;
use admint::run;
use admint::commands;
use clap::AppSettings;
use clap::SubCommand;
use clap::App;
use clap::Arg;
use regex::Regex;
use std::process;

fn ipv4_address_validator(addr: String) -> Result<(), String> {
    if ipparser::is_ipv4_addr(&addr) {
        return Ok(());
    }
    Err(format!("{} is not a valid IPv4 address ", addr))
}

fn sock_address_v4_validator(sockaddr: String) -> Result<(), String> {
    if ipparser::is_socket_addr_v4(&sockaddr) {
        return Ok(());
    }
    Err(format!("{} is not a valid IPv4 socket address ", sockaddr))
}

fn bool_validator(b: String) -> Result<(), String> {
    if let Ok(_b) = b.parse::<bool>() {
        return Ok(());
    }
    Err(format!("{} is not a valid boolean value", b))
}

fn list_size_validator(ll: String) -> Result<(), String> {
    if let Ok(_ll) = ll.parse::<u16>() {
        return Ok(());
    }
    Err(format!("{} is not a valid list size number, this value must be between [0,65535]", ll))
}

fn usize_validator(num: String) -> Result<(), String> {
    if let Ok(_n) = num.parse::<usize>() {
        return Ok(());
    }
    Err(format!("{} is not a valid unsigned number", num))
}

fn mac_validator(mac: String) -> Result<(), String> {
    if mac.is_ascii() {
        let mac_re = Regex::new(r"^((([a-fA-F0-9][a-fA-F0-9]+[-]){5}|([a-fA-F0-9][a-fA-F0-9]+[:]){5})([a-fA-F0-9][a-fA-F0-9])$)|(^([a-fA-F0-9][a-fA-F0-9][a-fA-F0-9][a-fA-F0-9]+[.]){2}([a-fA-F0-9][a-fA-F0-9][a-fA-F0-9][a-fA-F0-9]))$").unwrap();
        if mac_re.is_match(&mac) {
            return Ok(());
        }
    }
    Err(format!("{} is not a valid mac address", mac))
}

fn username_pattern_validator(pattern: String) -> Result<(), String> {
    if pattern.is_ascii() {
        return Ok(());
    }
    Err(format!("{} is not a valid pattern", pattern))
}

fn key_password_validator(key: String) -> Result<(), String> {
    if key.is_ascii() && key.len() < 33 {
        return Ok(());
    }
    Err(format!("This value must have less than 33 characters and all must be ascii"))
}

fn capacity_validator(c: String) -> Result<(), String> {
    if let Ok(v) = c.parse::<u16>() {
        if v < 2 {
            return Err(format!("This value must be between [2,65535]"));
        } else {
            return Ok(());
        }
    }
    return Err(format!("This value must be between [2,65535]"));
}

fn drop_votes_validator(c: String) -> Result<(), String> {
    if let Ok(v) = c.parse::<u8>() {
        if v < 1 {
            return Err(format!("This value must be between [1,255]"));
        } else {
            return Ok(());
        }
    }
    return Err(format!("This value must be between [1,255]"));
}

fn main() {
    let matches = App::new("ADMINT")
                          .version("1.0")
                          .author("Jorge A. <jorge4larcon@gmail.com>")
                          .about("ADministration tool for MINT server")
                          .setting(AppSettings::ArgRequiredElseHelp)
                          .subcommand(SubCommand::with_name("running-config")
                                       .about("Get the running config of the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")                                       
                                       .arg(Arg::with_name("admin-password")
                                            .index(1)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(2)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("set-dropvotes")
                                       .about("Set the drop votes of the server, this command can drop users that are logged in the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("drop-votes")
                                            .index(1)
                                            .short("d")
                                            .long("drop-votes")
                                            .value_name("DROP VOTES")
                                            .help("The new drop votes value for the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(drop_votes_validator))
                                        .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                        .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("set-dropverification")
                                       .about("Enable/Disable the drop verification in the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("state")
                                            .index(1)
                                            .short("s")
                                            .long("state")
                                            .value_name("STATE")
                                            .help("The new state of the drop verification in the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(bool_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("set-listsize")
                                       .about("Set the list size of the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("list-size")
                                            .index(1)
                                            .short("l")
                                            .long("list-size")
                                            .value_name("LIST SIZE")
                                            .help("The new list size of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(list_size_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("set-capacity")                          
                                       .about("Set the capacity of the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("capacity")
                                            .index(1)
                                            .short("c")
                                            .long("capacity")
                                            .value_name("CAPACITY")
                                            .help("The new capacity of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(capacity_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("set-password")
                                       .about("Set the password for the normal users")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("password")
                                            .index(1)
                                            .short("p")
                                            .long("password")
                                            .value_name("PASSWORD")
                                            .help("The new password for the clients")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("set-key")
                                       .about("Set the password for the admin user")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("key")
                                            .index(1)
                                            .short("k")
                                            .long("key")
                                            .value_name("KEY")
                                            .help("The new password for the admin")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("drop")
                                       .about("Drop a client from the server with an specific ip address")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("ip")
                                            .index(1)
                                            .short("i")
                                            .long("ip")
                                            .value_name("IP ADDRESS")
                                            .help("The IPv4 address of the client to drop (can be more than one)")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(ipv4_address_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("get-mac")
                                       .about("Get a client from the server with an specific mac address")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("mac")
                                            .index(1)
                                            .short("m")
                                            .long("mac")
                                            .value_name("MAC")
                                            .long_help("The mac to search, it could be aaaa.bbbb.cccc, aa:aa:bb:bb:cc:cc or aa-aa-bb-bb-cc-cc")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(mac_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(2)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(3)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("get-username")
                                       .about("Get a list of clients from the server with an specific pattern in their usernames")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("pattern")
                                            .index(1)
                                            .short("p")
                                            .long("pattern")
                                            .value_name("PATTERN")
                                            .help("The pattern to search")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(username_pattern_validator))
                                        .arg(Arg::with_name("start")
                                            .index(2)
                                            .short("s")
                                            .long("start")
                                            .value_name("START_INDEX")
                                            .help("The start index of the list")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(usize_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(3)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(4)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .subcommand(SubCommand::with_name("get-index")
                                       .about("Get a list of clients from the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("start")
                                            .index(1)
                                            .short("s")
                                            .long("start")
                                            .value_name("START_INDEX")
                                            .help("The start index of the list")                                            
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(usize_validator))
                                        .arg(Arg::with_name("end")
                                            .index(2)
                                            .short("e")
                                            .long("end")
                                            .value_name("END_INDEX")
                                            .help("The end index of the list")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(usize_validator))
                                       .arg(Arg::with_name("admin-password")
                                            .index(3)
                                            .short("P")
                                            .long("admin-password")
                                            .value_name("ADMIN PASSWORD")
                                            .help("The admin password of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(key_password_validator))
                                       .arg(Arg::with_name("server-address")
                                            .index(4)
                                            .short("a")
                                            .long("server-address")
                                            .value_name("SERVER IPV4 ADDRESS AND PORT")
                                            .help("The socket address of the server")
                                            .takes_value(true)
                                            .required(true)
                                            .number_of_values(1)
                                            .validator(sock_address_v4_validator)))
                          .get_matches();

    if let Some(command) = commands::BaseCommand::from_clap_matches(&matches) {
        run(command);
    } else {
        eprintln!("I didn't understand your command");
        process::exit(1);
    }

}
