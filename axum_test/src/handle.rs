use std::collections::HashMap;

use tokio_stream::StreamExt;

pub(super) async fn handle(
    mut cache_rx: tokio_stream::wrappers::UnboundedReceiverStream<crate::SenderBody>,
) {
    let mut user_map = HashMap::<String, crate::User>::new();

    while let Some(event) = cache_rx.next().await {
        match event.method {
            crate::Method::Create => {
                user_map.insert(event.user.name.clone(), event.user);
                event.sender.send(crate::Reponse::Ok).unwrap();
            }
            crate::Method::Query => match user_map.get(&event.user.name) {
                Some(v) => {
                    event
                        .sender
                        .send(crate::Reponse::User(v.to_owned()))
                        .unwrap();
                }
                None => {
                    event
                        .sender
                        .send(crate::Reponse::Err("没有该用户".to_string()))
                        .unwrap();
                }
            },
            crate::Method::Del => match user_map.remove(&event.user.name) {
                Some(_) => {
                    event.sender.send(crate::Reponse::Ok).unwrap();
                }
                None => {
                    event
                        .sender
                        .send(crate::Reponse::Err("没有该用户".to_string()))
                        .unwrap();
                }
            },
        }
    }
}
