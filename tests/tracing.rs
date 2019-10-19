#![deny(warnings)]
extern crate tracing;
#[cfg(feature = "tracing-futures")]
extern crate tracing_futures;
extern crate tracing_subscriber;
extern crate warp;

#[cfg(feature = "tracing-futures")]
use tracing_futures::Instrument;
#[cfg(feature = "tracing-futures")]
use warp::Filter;

#[cfg(feature = "tracing-futures")]
#[test]
fn uses_tracing() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter("debug")
        .without_time()
        .finish();
    async fn inner() {
        let span = tracing::debug_span!("outer", field = "a");
        let _guard = span.enter();

        let ok = warp::any()
            .map(|| {
                tracing::error!("this isn't printed but should be");
                tracing::trace!("this shouldn't be printed");
                tracing::debug!(other_field = "b", "inner debug");
                println!("hello");
            })
            .untuple_one()
            .map(warp::reply)
            .in_current_span();

        tracing::info!("this is printed");

        let req = warp::test::request();
        let resp = req.reply(&ok).await;
        assert_eq!(resp.status(), 200);
    }
    tracing::subscriber::with_default(subscriber, || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(inner());
    });
}
