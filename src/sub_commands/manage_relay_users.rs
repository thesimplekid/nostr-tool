use serde::{Deserialize, Serialize};

use clap::Args;
use nostr_sdk::prelude::*;

use crate::utils::{create_client, handle_keys};

#[derive(Args)]
pub struct ManageRelayUsers {
    #[arg(short, long)]
    user_file: String,
    // Print keys as hex
    #[arg(long, default_value = "false")]
    hex: bool,
}

#[derive(Serialize, Deserialize)]
struct Users {
    allow: Vec<String>,
    deny: Vec<String>,
}

/*
Example users JSON
```json
{
  "allow": ["d81eb632d2385c3e6bdc8da5a32b57275348819aebd39ff74613793f29694203"],
  "deny": ["7c27a04b7c27299f16dc07d3eb8f28544f188bc7a34982328b7d581edc405dc2", "7995c67e4b40fcc88f7603fcedb5f2133a74b89b2678a332b21faee725f039f9"]
}
```

 */

pub fn manage_relay_users(
    priv_key: Option<String>,
    relays: Vec<String>,
    sub_command_args: &ManageRelayUsers,
) -> Result<()> {
    if relays.is_empty() {
        panic!("No relays specified, at least one relay is required!")
    }

    let keys = handle_keys(priv_key, sub_command_args.hex)?;
    let client = create_client(&keys, relays.clone(), 0)?;

    let file = std::fs::File::open(&sub_command_args.user_file)?;

    let users: Users = serde_json::from_reader(file)?;

    let allow = Tag::Generic(TagKind::Custom("allow".to_string()), users.allow);
    let deny = Tag::Generic(TagKind::Custom("deny".to_string()), users.deny);

    let event = EventBuilder::new(Kind::Custom(4242), "", &vec![allow, deny]).to_event(&keys)?;

    client.send_event(event)?;

    Ok(())
}
