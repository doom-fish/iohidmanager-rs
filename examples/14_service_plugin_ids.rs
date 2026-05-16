use iohidmanager::service_plugin::ALL_SERVICE_PLUGIN_UUIDS;

fn main() {
    for uuid in ALL_SERVICE_PLUGIN_UUIDS {
        println!("{:?}: {}", uuid, uuid.hyphenated());
    }
}
