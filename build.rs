use prost_build::Config;

fn main() {
    let mut config = Config::new();
    config.bytes(&["."]);

    tonic_build::configure()
        .out_dir("src/pb")
        .compile_with_config(
            config,
            &[
                "IDL/acceptor.proto",
                "IDL/publisher.proto",
                "IDL/router.proto",
                "IDL/subscriber.proto",
            ],
            &["IDL"],
        )
        .unwrap();
}
