use clap::{App, Arg};
use shiplift::Docker;
use subprocess::Redirection;
use users::{get_current_uid, get_user_by_uid};

use colored::*;

#[tokio::main]
async fn main() {
    let docker = Docker::new();
    let matches = App::new("Aboard")
        .version("0.1.0")
        .author("Antoine Morin-Paulhus")
        .about("Get inside a container as current user to avoid wrecking file permissions.")
        .arg(
            Arg::with_name("container")
                .help("Container image name or container ID")
                .takes_value(true),
        )
        .arg(Arg::with_name("root")
                 .short("r")
                 .long("root")
                 .required(false)
                 .takes_value(false)
                 .help("Launch shell with superuser."))
        .get_matches();

    let container_name_or_id = matches.value_of("container").unwrap();

    // Get container list
    let options = Default::default();
    let containers = docker.containers();
    let containers_list_result = containers.list(&options).await;
    let containers_list = match containers_list_result {
        Ok(containers_list) => containers_list,
        Err(_) => return,
    };

    // Verify if there is a match in the container ids first
    for container in containers_list.iter() {
        if container.id.contains(container_name_or_id) {
            shell_in_container(container.id.clone(), matches.is_present("root"));
            return ();
        }
    }

    // If no container was found using the ID, look for image names
    for container in containers_list.iter() {
        if container.image.contains(container_name_or_id) {
            shell_in_container(container.id.clone(), matches.is_present("root"));
            return ();
        }
    }
}

fn shell_in_container(id: String, as_root: bool) {
    let uid = get_current_uid();
    let user = get_user_by_uid(uid).unwrap();
    let mut username = user.name().to_str().unwrap();

    println!(
        "{} {} {}",
        "Adding user".blue(),
        username.red(),
        "to container...".blue()
    );

    if as_root {
        username = "root";
    } else {
        // Create user in container
        let out = subprocess::Exec::cmd("docker")
            .arg("exec")
            .arg("-it")
            .arg(id.clone())
            .arg("/bin/sh")
            .arg("-c")
            .arg(format!(
                "adduser --disabled-password --gecos GECOS \"{}\"",
                username
            ))
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture();

        match out {
            Ok(_) => (),
            Err(_) => panic!("Could not create user!"),
        };
    }

    println!("{} {}", "Launching /bin/sh in container".blue(), id);

    // We could use adduser with option-u {uid}, however this causes problems if the
    // container already has another user with this uid e.g.: the default new linux
    // user uid which is frequently 1000.

    // Run the shell as user
    let out = subprocess::Exec::cmd("docker")
        .arg("exec")
        .arg("-it")
        .arg(id)
        .arg("/bin/sh")
        .arg("-c")
        .arg(format!("su {}", username))
        .arg("-i")
        .join();

    match out {
        Ok(_) => (),
        Err(_) => panic!("Could not launch shell!"),
    };

    // Prevent next line of host shell from appearing next to other text
    println!("\n");
}
