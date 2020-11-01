use envcage_domain::log_entries;
use envcage_tools::cmdline;

cmdline! { db {
    let result = log_entries(&db)?;
    println!("{:#?}", result);
} }
