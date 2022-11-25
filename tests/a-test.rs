use tokio::sync::{mpsc, oneshot};
use steamworks::{AppIDs, Client, PublishedFileId, UGCType, UserList, UserListOrder};


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use tokio::sync::oneshot;
    use steamworks::{AppId, AppIDs, Client, PublishedFileId, QueryResults, SteamError, UGCType, UserList, UserListOrder};
    #[cfg(test)]
    use serial_test::serial;
    #[test]
    fn subscribed_items() {
        let (client, single) = Client::init().unwrap();
        let a = client.ugc().subscribed_items();
        println!("{:#?}", a)
    }

    #[test]
    #[serial]
    fn query_item() {
        let (client, single) = Client::init().unwrap();
        let a = client.ugc().subscribed_items();
        // let (tx, rx) = oneshot::channel();

        let test = client.ugc().query_item(PublishedFileId(2892196504));
        let mut a = test.unwrap();
        a = a.include_additional_previews(true);
        a= a.include_metadata(true);
        println!("0");
        a.fetch(|result| {
            let qs = match result {
                Ok(t) => t,
                Err(e) => panic!("pro"),
            };
            let str = qs.preview_url(0).unwrap();
            println!("{}",str);
        });
        for _ in 0..100 {
            single.run_callbacks();
            ::std::thread::sleep(::std::time::Duration::from_millis(100));
        }
    }
}

    #[tokio::test]
    async fn update_item(){
        let (client, single) = Client::init().unwrap();
        let (tx, rx) = oneshot::channel();

            let mut update = client.ugc().start_item_update(client.utils().app_id(), PublishedFileId(2892196504));
            update = update.description("test");

            update.submit(Some("aaaaa"), |result| {
                println!("{:#?}", result.unwrap().1);
                tx.send(result).unwrap();
                });

        let result = rx.await.unwrap();
        match result {
            Ok((item_id, needs_to_accept_agreement)) => println!("{:#?}",item_id),
            Err(e) => panic!("pro")
        }
        for _ in 0..100 {
            single.run_callbacks();
            ::std::thread::sleep(::std::time::Duration::from_millis(100));
        }
        println!("==========");
    }
    #[test]
    fn test_query_user() {
        let (client, single) = Client::init().unwrap();
        let accountId=client.user().steam_id().account_id();
        println!("{:#?}", accountId);
        let appId = client.utils().app_id();
        let res = client.ugc().query_user(accountId, UserList::Subscribed, UGCType::Items, UserListOrder::CreationOrderAsc, AppIDs::Both {creator:appId,consumer:appId}, 1);
        let f = match res {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        println!("aaaaaaaaa");
        f.fetch(|x| match x {
            Ok(_) => {println!("a")}
            Err(_) => {println!("b")}
        })
    }

    #[test]
    fn publish_workshop_item() {
        let (client, single) = Client::init().unwrap();
    }


async fn real_update_item() -> PublishedFileId {
    let (client, single) = Client::init().unwrap();
    let (tx, rx) = oneshot::channel();
    println!("{:#?}", client.utils().app_id());
    println!("{:#?}", PublishedFileId(2892196504));
    {
        let mut update = client.ugc().start_item_update(client.utils().app_id(), PublishedFileId(2892196504));
        update = update.description("test");

        update.submit(Some("aaaaa"), |result| {
            tx.send(result).unwrap();
        });
    }
    let result = rx.await.unwrap();
    match result {
        Ok((item_id, needs_to_accept_agreement)) => item_id,
        Err(e) => PublishedFileId(1),
    }
}

#[tokio::test]
async fn test_update_item(){
    let a = real_update_item().await;
    // let b = match a {
    //     Ok(id) => id,
    //     Err(e) => Err("a"),
    // };
    println!("{:#?}", a);

}
