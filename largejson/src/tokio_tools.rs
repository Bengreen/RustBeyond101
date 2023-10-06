//! Module to handle easy sending functions to tokio
//!
//! The provides two functions one function run_in_tokio creates and sends the function to tokio.
//! The second function run_in_tokio_with_cancel allows the creation of a CancellationToken which can be used to shut down the tokio async.

use crate::error::MyError;
use futures::Future;
use log::{info, error};

use tokio_util::sync::CancellationToken;

/// run async function inside tokio instance on current thread
pub fn run_in_tokio<F, T>( my_function: F)  -> F::Output
where F: Future<Output = Result<T, MyError>>
 {
    info!("starting Tokio");

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Runtime created in current thread")
        .block_on(my_function)
}

/// Run async with cancellability via CancellationToken
pub fn run_in_tokio_with_cancel<F, T>(cancel: CancellationToken, my_function: F) -> F::Output
    where F: Future<Output = Result<T,MyError>>
{
    run_in_tokio(async {
        tokio::select!{
            _ = cancel.cancelled() => {
                error!("Token cancelled");
                Err(MyError::Cancelled)
            },
            z = my_function => {
                eprintln!("Completed function");
                z
            },
        }
    })
}
