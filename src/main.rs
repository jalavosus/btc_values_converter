use clap::{Arg, App};

const BTC_TO_SATOSHI: f64 = 100000000.0;

fn btc_to_sats(btc: f64) -> u64 {
    let res = btc * BTC_TO_SATOSHI;
    let res = res as u64;

    return res;
}

fn sats_to_btc(sats: u64) -> f64 {
    let sats = sats as f64;
    let res = sats / BTC_TO_SATOSHI;

    return res;
}

fn main() {
    let flags = App::new("BTC Values converter")
        .version("0.1.0")
        .author("James Alavosus <alavosus.james@gmail.com>")
        .about("Converts btc to satoshis or vice-versa")
        .arg(Arg::with_name("to-btc")
            .short("b")
            .long("to-btc")
            .takes_value(true)
            .help("Value in satoshis to convert to btc"))
        .arg(Arg::with_name("to-sats")
            .short("s")
            .long("to-sats")
            .takes_value(true)
            .help("Value in btc to convert to satoshis"))
        .get_matches();

    let sats_to_convert = flags.value_of("to-btc").unwrap_or("0");
    let btc_to_convert = flags.value_of("to-sats").unwrap_or("0.0");

    if sats_to_convert != "0" {
        let sats: u64 = sats_to_convert.parse::<u64>().unwrap();
        let converted: f64 = sats_to_btc(sats);
        println!("{}", converted);
    } else if btc_to_convert != "0.0" {
        let btc: f64 = btc_to_convert.parse::<f64>().unwrap();
        let converted: u64 = btc_to_sats(btc);
        println!("{}", converted);
    }
}
