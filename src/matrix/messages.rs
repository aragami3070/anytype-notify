use std::error::Error;

use crate::{
    anytype::{
        entities::notification::{AnytypeToMatrixIdMap, NotificationObject},
        parser::find_matrix_user_id,
    },
    matrix::{
        api::auth::DeviceId,
        client::{Client, RoomId},
    },
};

pub async fn send_message(
    notify: NotificationObject,
    matrix_id_map: &AnytypeToMatrixIdMap,
    matrix_client: &Client,
    room_id: &RoomId,
    device_id: &DeviceId,
) -> Result<(), Box<dyn Error>> {
    let name = &notify.name;
    let snippet = &notify.snippet;
    let creation_date = &notify.creation_date;
    let due_date = &notify.due_date;

    // Get matrix user ids using mapping
    let assignee = &notify
        .assignee
        .iter()
        .map(|a| find_matrix_user_id(matrix_id_map, a.as_str()))
        .collect::<Vec<String>>()
        .join(", ");

    let proposed_by = &notify
        .proposed_by
        .iter()
        .map(|p| find_matrix_user_id(&matrix_id_map, p.as_str()))
        .collect::<Vec<String>>()
        .join(", ");

    let message = format!(
        "От {proposed_by} поступила новая задача:\n{name}\n\n{snippet}\n\n{assignee}\n\nДата создания: {creation_date}\nДедлайн: {due_date}"
    );

    matrix_client
        .room()
        .send_message(&room_id, &device_id, message.to_string())
        .await?;
    println!("Notification text:");
    println!("{message}");
    println!();
    Ok(())
}
