use bevy::prelude::*;

use crate::grpc::GrpcPlugin;

mod grpc;

#[tokio::main]
async fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(GrpcPlugin)
        .add_resource(Counter { count: 0 })
        .add_system(increase_counter_system.system())
        .run();
}

struct Counter {
    count: usize,
}

fn increase_counter_system(mut counter: ResMut<Counter>) {
    counter.count += 1;
}
