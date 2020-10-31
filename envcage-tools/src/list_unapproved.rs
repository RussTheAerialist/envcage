use envcage_domain::unapproved_devices;
use envcage_tools::cmdline;

cmdline! { db {
    let result = unapproved_devices(&db)?;
    println!("{:#?}", result);
} }
