use diesel::pg::PgConnection;
use envcage_domain::create_log_entry;
use envcage_tools::cmdline;
use rumqttc::{Client, ConnectionError, Event, MqttOptions, Packet, QoS};
use structopt::StructOpt;

mod error;
mod payload;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    daemon: bool,
}

cmdline! { db Opt opt {
    if opt.daemon {
    } else {
        run(&db)?
    }
}}

fn run(db: &PgConnection) -> Result<(), Box<dyn Error>> {
    let host = env::var("MQTT_ADDRESS")?;
    let port = env::var("MQTT_PORT")?.parse()?;
    let keep_alive = env::var("MQTT_KEEP_ALIVE")
        .or_else::<String, _>(|_| Ok("6".to_owned()))?
        .parse()?;
    let topic = env::var("MQTT_TOPIC")?;

    debug!("Connecting to {}:{}/{}", host, port, topic);
    let mut mqttoptions = MqttOptions::new("envcage-recorder", host, port);
    mqttoptions.set_keep_alive(keep_alive);
    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe(topic, QoS::ExactlyOnce)?;

    for notification in connection.iter() {
        handle_notification(db, notification);
    }

    Ok(())
}

fn handle_notification(db: &PgConnection, msg: Result<Event, ConnectionError>) {
    process_notification(db, msg).unwrap_or_else(|e| {
        error!("{:?}", e);
    });
}

fn process_published(db: &PgConnection, item: Packet) -> Result<(), Box<dyn Error>> {
    if let Packet::Publish(p) = item {
        let mac_addr = p
            .topic
            .split('/')
            .nth(1)
            .ok_or_else(|| error::Error::Split(p.topic.to_owned()))?;
        let payload = p.payload.to_vec();
        let payload = std::str::from_utf8(&payload)?;
        let payload = payload.parse::<payload::Payload>()?;

        debug!(
            "Addr={} T={}F H={}",
            mac_addr, payload.temperature, payload.humidity
        );
        create_log_entry(
            db,
            mac_addr,
            payload.created,
            &payload.temperature,
            &payload.humidity,
        )?;
    }

    Ok(())
}

fn process_notification(
    db: &PgConnection,
    msg: Result<Event, ConnectionError>,
) -> Result<(), Box<dyn Error>> {
    let msg = msg?;

    if let Event::Incoming(item) = msg {
        process_published(db, item)?;
    }

    Ok(())
}
