use rumqttc::{MqttOptions, Client, QoS};
use envcage_tools::cmdline;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {

}
cmdline! { db Opt opt {
    let host = env::var("MQTT_ADDRESS")?;
    let port = env::var("MQTT_PORT")?.parse()?;
    let keep_alive = env::var("MQTT_KEEP_ALIVE").or_else::<String, _>(|_| Ok("6".to_owned()))?.parse()?;
    let topic = env::var("MQTT_TOPIC")?;

    info!("Connecting to {}:{}/{}", host, port, topic);
    let mut mqttoptions = MqttOptions::new("envcage-recorder", host, port);
    mqttoptions.set_keep_alive(keep_alive);
    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe(topic, QoS::ExactlyOnce)?;

    for notification in connection.iter() {
        println!("Received: {:?}", notification);
    }
}}