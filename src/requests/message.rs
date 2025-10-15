use std::sync::Arc;

use crate::{server::Server, types, utils::client::Client};

crate::logger!(LOGGER "Message Manager");

pub fn send(
    server: &Arc<Server>,
    client: &Client,
    channel_id: &str,
    contents: &str,
) -> crate::Result<()> {
    LOGGER.info(format!("SendMessage to {channel_id}: {contents}"));

    if contents.is_empty() {
        client.send(types::ResponseError::InvalidRequest(format!(
            "Invalid message: empty message"
        )))?;

        return Ok(());
    }

    let msg = server.db.insert_message(
        channel_id,
        &client.get_uuid()?,
        &contents,
        chrono::Utc::now().timestamp(),
    )?;

    if msg.from != msg.channel_id {
        let server = server.clone();

        for c in server.clients.lock().unwrap().iter() {
            let c = c.clone();
            let server = server.clone();
            let msg = msg.clone();

            std::thread::spawn(move || {
                if let Some(uuid) =
                    LOGGER.extract(server.wrap_err(&c, c.get_uuid()), "Unable to get uuid")
                {
                    if uuid != msg.channel_id && uuid != msg.from {
                        return;
                    }

                    LOGGER.extract(
                        server.wrap_err(&c, c.send(types::ServerMessage::MessageCreate(msg))),
                        "Failed to send message",
                    );
                }
            });
        }
    }

    LOGGER.extract(
        server.wrap_err(
            &client,
            client.send(types::ServerMessage::MessageCreate(msg)),
        ),
        "Failed to send message",
    );

    Ok(())
}

pub fn edit(
    _server: &Arc<Server>,
    _client: &Client,
    message_id: usize,
    new_contents: &str,
) -> crate::Result<()> {
    LOGGER.info(format!("EditMessage {message_id}: {new_contents}"));
    Ok(())
}

pub fn delete(_server: &Arc<Server>, _client: &Client, message_id: usize) -> crate::Result<()> {
    LOGGER.info(format!("DeleteMessage {message_id}"));
    Ok(())
}
