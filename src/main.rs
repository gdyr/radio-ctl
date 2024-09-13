mod cli;
mod radio;

use clap::Parser;

fn main() {

    let args = cli::Args::parse();
    println!("{:?}", args);

    let radios = radio::get_radios().expect("Failed to enumerate radios");

    for radio in radios {
        
        if let Some(name) = &args.name {
            if radio.Name().expect("Failed to get radio name") != name {
                continue;
            }
        }

        if let Some(kind) = &args.kind {
            if radio.Kind().expect("Failed to get radio kind") != (*kind).clone().into() {
                continue;
            }
        }

        match args.command() {
            cli::Command::List => {
                println!("{}", radio::describe_radio(&radio));
            },
            cli::Command::On => {
                radio::turn_on_radio(&radio).expect("Failed to turn on radio");
            },
            cli::Command::Off => {
                radio::turn_off_radio(&radio).expect("Failed to turn off radio");
            },
        }

    }

    // let radios = get_radios().expect("Failed to enumerate radios");

    // for radio in radios {
    //     println!("{}", describe_radio(&radio));

    //     // if radio.Kind().expect("Failed to get radio kind") == RadioKind::Bluetooth {
    //     //     radio.SetStateAsync(RadioState::On).expect("Failed to turn off Bluetooth radio").get().expect("Failed to turn off Bluetooth radio");
    //     // }

    // }

}
