use tokio::sync::oneshot;
use steamworks::{AppIDs, Client, PublishedFileId, UGCType, UserList, UserListOrder};

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use tokio::sync::oneshot;
    use steamworks::{AppId, AppIDs, Client, PublishedFileId, QueryResults, SteamError, UGCType, UserList, UserListOrder};

    #[test]
    fn subscribed_items() {
        let (client, single) = Client::init().unwrap();
        let a = client.ugc().subscribed_items();
        println!("{:#?}", a)
    }

    #[tokio::test]
    async fn query_item() {
        // let (client, single) = Client::init().unwrap();
        // let a = client.ugc().subscribed_items();
        // println!("{:#?}", a[0]);
        //
        // let (tx, rx) = oneshot::channel();
        // {
        //     let test = client.ugc().query_item(PublishedFileId(1));
        //     let f = match test {
        //         Ok(x) => x,
        //         Err(error) => panic!("Problem opening the file: {:?}", error),
        //     };
        //     f.fetch(|r| {
        //         tx.send(r).unwrap()
        //     });
        //
        //     let result = rx.await.unwrap();
        //     println!("{:#?}", result);
        //     match result {
        //         Ok(_) => (),
        //         Err(_) => ()
        //     };
        //     println!("======");
        // }
    }
}

    #[tokio::test]
    async fn update_item(){
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
            Ok((item_id, needs_to_accept_agreement)) => println!("{:#?}",item_id),
            Err(e) => panic!("pro")
        }
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



