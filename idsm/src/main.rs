
// afunction Open tcp and udp socket to listen for packets
// Parse packets and extract information and keep in list
// Collecting & buffering possible security events (SEVs) from Smart Sensors (Can be SW-C, a CDD or a BSW Module).
// Analyzing the collected SEVs and identifying possible security incidents (SIs).
// Reporting the identified SIs to the Security Manager (SM).

// Importing the required libraries
use std::net::{TcpListener, UdpSocket};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
// include YAML
use serde_yaml::Value;
use serde_yaml::Mapping;

// How to use serde_yaml?
// https://docs.rs/serde_yaml/0.8.17/serde_yaml/


// Function to open tcp and udp socket to listen for packets
fn open_socket() {
    // Read ip and port from YAML file

    // Open a tcp socket to listen for packets
    let listener = TcpListener::bind(


fn main() {
    println!("Hello, world!");
}
