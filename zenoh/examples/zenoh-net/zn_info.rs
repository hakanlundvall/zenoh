//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use clap::{App, Arg, Values};
use zenoh::net::*;

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    let mut config: Properties = parse_args();
    config.push((config::ZN_USER_KEY, b"user".to_vec()));
    config.push((config::ZN_PASSWORD_KEY, b"password".to_vec()));

    println!("Opening session...");
    let session = open(config).await.unwrap();

    let info = session.info().await;
    for (key, value) in info {
        println!(
            "{} : {}",
            info::key_to_string(key),
            hex::encode_upper(value)
        );
    }
}

fn parse_args() -> Properties {
    let args = App::new("zenoh-net info example")
        .arg(
            Arg::from_usage("-m, --mode=[MODE] 'The zenoh session mode.")
                .possible_values(&["peer", "client"])
                .default_value("peer"),
        )
        .arg(Arg::from_usage(
            "-e, --peer=[LOCATOR]...  'Peer locators used to initiate the zenoh session.'",
        ))
        .arg(Arg::from_usage(
            "-l, --listener=[LOCATOR]...   'Locators to listen on.'",
        ))
        .get_matches();

    let mut config = config::empty();
    config.push((
        config::ZN_MODE_KEY,
        args.value_of("mode").unwrap().as_bytes().to_vec(),
    ));
    for peer in args
        .values_of("peer")
        .or_else(|| Some(Values::default()))
        .unwrap()
    {
        config.push((config::ZN_PEER_KEY, peer.as_bytes().to_vec()));
    }
    for listener in args
        .values_of("listener")
        .or_else(|| Some(Values::default()))
        .unwrap()
    {
        config.push((config::ZN_LISTENER_KEY, listener.as_bytes().to_vec()));
    }
    config
}
