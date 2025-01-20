use std::fs::File;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddr};

use clap::Parser;
use hickory_resolver::{
	config::{NameServerConfig, ResolverConfig, ResolverOpts},
	Resolver,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	r#loop: bool,
}

fn check_engine(err: &str) {
	if atty::is(atty::Stream::Stdout) {
		// We are running in a terminal
		println!("\x1b[1;5;31;107m Check Engine! \x1b[0m");
		print!("{}", err);
	}
	let mut file = File::create("/tmp/check_engine").unwrap();
	file.write_all(err.as_bytes()).unwrap();
}

fn main() {
	let args = Args::parse();

	if args.r#loop {
		std::thread::sleep(std::time::Duration::from_secs(5));
		loop {
			run_checks();

			// Sleep for 5 seconds
			std::thread::sleep(std::time::Duration::from_secs(5));
		}
	} else {
		run_checks();
	}
}

fn run_checks() {
	dns_check();
}

fn dns_check() {
	// Try to resolve "google.com" using the local 127.0.0.1 DNS Server (this might not be the system's DNS Server)
	let mut resolverconfig = ResolverConfig::new();
	resolverconfig.add_name_server(NameServerConfig::new(
		SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53),
		hickory_resolver::config::Protocol::Udp,
	));
	let mut resolveropts = ResolverOpts::default();
	resolveropts.attempts = 1;
	resolveropts.timeout = std::time::Duration::from_secs(3);
	let resolver = Resolver::new(resolverconfig, resolveropts).unwrap();
	let response = resolver.lookup_ip("www.google.com.");
	match response {
		Ok(_) => {
			println!("Success");
		}
		Err(err) => {
			println!("Error: {}", err);
			check_engine(&format!("DNS Server: {}\n", err));
		}
	}
}
