
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let names = vec!["config-service", "key-service", "python3.12"];
    for name in names {
        let resp = docker.start_container(name);
        println!("{:?}", resp);
    }

    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => {
            panic!("{}", e);
        }
    };

    println!("{:?}", containers);
}
