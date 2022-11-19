use std::future::Future;
use std::pin::Pin;

use bson::Document;
use futures::StreamExt;
use mongodb::Cursor;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub trait CursorIntoVec {
    fn into_vec<T>(self) -> Pin<Box<dyn Future<Output=Vec<T>> + Unpin>>
        where
            T: 'static + DeserializeOwned;
}

pub fn struct_into_document<'a, T: Sized + Serialize + Deserialize<'a>>(t: &T) -> Option<Document> {
    let mid: Option<Document> = bson::to_bson(t)
        .ok()
        .map(|x| x.as_document().unwrap().to_owned());

    mid.map(|mut doc| {
        let keys = doc.keys();
        let rm: Vec<String> = keys
            .filter(|k| doc.is_null(k))
            .map(|x| x.to_owned())
            .collect();
        // remove null value fields
        for x in rm {
            doc.remove(&x);
        }
        doc
    })
}
