use crate::{http::GQLResponse, QueryResponse, Result};
use bytes::{Buf, Bytes};
use futures::{Stream, StreamExt};

/// Create a multipart response data stream.
pub fn multipart_stream(s: impl Stream<Item = Result<QueryResponse>>) -> impl Stream<Item = Bytes> {
    s.map(|res| serde_json::to_vec(&GQLResponse(res)).unwrap())
        .map(|data| {
            let mut b = Bytes::from(format!(
                "\r\n---\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
                data.len()
            ))
            .chain(Bytes::from(data));
            b.copy_to_bytes(b.remaining())
        })
        .chain(futures::stream::once(async move {
            Bytes::from_static(b"\r\n-----\r\n")
        }))
}
