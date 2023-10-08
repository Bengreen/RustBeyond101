
use std::{time::Duration, fmt::Display};

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use futures::future::join_all;
use largejson::{schema::Person, webservice::{http_service_cancellable, WebServicePrefixConfig, WebServiceConfig}, error::MyError};
use reqwest::{Client, Response};
use serde::Serialize;
use tokio::runtime::{Runtime, self};
use tokio_util::sync::CancellationToken;


#[derive(Debug, PartialEq, Clone)]
enum ParamVariant {
    ServerThreads,
    ClientThreads,
    DataLength,
    ConcurrentUsers,
}

#[derive(Debug)]
struct PostParams {
    num_threads_server: usize,
    num_threads_client: usize,
    data_length: usize,
    num_concurrent: usize,
    variable: ParamVariant,
}

impl Display for PostParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.variable)?;

        if self.variable != ParamVariant::ServerThreads {
            write!(f, "-server:{}", self.num_threads_server)?;
        }
        if self.variable != ParamVariant::ClientThreads {
            write!(f, "-client:{}", self.num_threads_client)?;
        }
        if self.variable != ParamVariant::DataLength {
            write!(f, "-length:{:07}", self.data_length)?;
        }
        if self.variable != ParamVariant::ConcurrentUsers {
            write!(f, "-concurrent:{:03}", self.num_concurrent)?;
        }
        Ok(())
    }
}



fn rt_multithreaded(threads: usize) -> Runtime {
    if threads == 0 {
        runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build().unwrap()
    } else {
        runtime::Builder::new_multi_thread()
            .worker_threads(threads)
            .thread_name(format!("threads-{threads}"))
            .thread_stack_size(3 * 1024 * 1024)
            .enable_io()
            .enable_time()
            .build().unwrap()
    }
}

fn gen_data(size: usize) -> Result<Vec<Person>, MyError> {
    Ok((0..size).map(|index| Person{
        name: format!("name-{:08}", index),
        age: u32::try_from(index).unwrap(),
    }).collect())
}

async fn http_post_fetch<MyObj: Serialize >(client: &Client, my_content: &Vec<MyObj>)  -> Result<Response, reqwest::Error> {
    client.post("http://localhost:8080/test/v0/review")
        .json(my_content)
        .send()
        .await
}


async fn http_post_parallel<MyObj: Serialize>(client: &Client, my_content: &Vec<MyObj>, concurrent_count: usize) {

    let list_of = (0..concurrent_count).map(|_index| {
        async move {
            let resp = http_post_fetch(client, my_content);
            match resp.await {
                Ok(_msg) => {
                },
                Err(e) => println!("ERRORED {}", e),
            };

        }
    });
    let _mye = join_all(list_of).await;
}




fn post_benchmark_paramed(c: &mut Criterion,
    group_name: &str,  variable: &ParamVariant, bench_name: &str,
    server_threads: Vec<usize>, client_threads: Vec<usize>, message_lengths: Vec<usize>, message_concurrents: Vec<usize>,
) {
    let config = WebServiceConfig{
        prefix: WebServicePrefixConfig {
            name: "test".to_owned(),
            version: "v0".to_owned(),
        },
    };

    let client = reqwest::Client::new();

    let mut group = c.benchmark_group(group_name);

    for num_threads_server in &server_threads {
        let server_rt = rt_multithreaded(*num_threads_server);
        let ct = CancellationToken::new();

        let server_jh = server_rt.spawn(
            http_service_cancellable(ct.clone(), config.clone())
        );

        for data_length in &message_lengths {
            let my_data = gen_data(*data_length).unwrap();


            for num_threads_client in &client_threads {
                let client_rt = rt_multithreaded(*num_threads_client);

                for num_concurrent in &message_concurrents {

                    group.throughput(Throughput::Elements((data_length*num_concurrent) as u64 ));

                    let my_var = match *variable {
                        ParamVariant::ServerThreads => num_threads_server,
                        ParamVariant::ClientThreads => num_threads_client,
                        ParamVariant::DataLength => data_length,
                        ParamVariant::ConcurrentUsers => num_concurrent,
                    };

                    group.bench_with_input(
                        BenchmarkId::new(format!("{}-{}", bench_name,PostParams{ num_threads_server: *num_threads_server, num_threads_client: *num_threads_client, data_length: *data_length, num_concurrent: *num_concurrent, variable: (*variable).clone() }), my_var),
                        // BenchmarkId::new(bench_name, PostParams{ num_threads_server: *num_threads_server, num_threads_client: *num_threads_client, data_length: *data_length, num_concurrent: *num_concurrent, variable: (*variable).clone() }),
                        my_var,
                        |b, &s| {
                            // Insert a call to `to_async` to convert the bencher to async mode.
                            // The timing loops are the same as with the normal bencher.
                            b.to_async(&client_rt).iter(|| http_post_parallel(&client, &my_data, *num_concurrent));
                        }
                    );
                }

                client_rt.shutdown_timeout(Duration::from_secs(3));
            }
        }


        ct.cancel();
        server_rt.shutdown_timeout(Duration::from_secs(3));
    }

    group.finish();
}


pub fn bench_concurrent(c: &mut Criterion) {
    let server_threads = vec!(1);
    let client_threads = vec!(5);
    let message_lengths = vec!(1000);
    let message_concurrents = vec!(1,2,10,20,200);

    post_benchmark_paramed(c, "Users", &ParamVariant::ConcurrentUsers, "post", server_threads, client_threads, message_lengths, message_concurrents);
}

pub fn bench_server(c: &mut Criterion) {
    let server_threads = vec!(1,2,5,10);
    let client_threads = vec!(5);
    let message_lengths = vec!(1000);
    let message_concurrents = vec!(200);

    post_benchmark_paramed(c, "Server Threads", &ParamVariant::ServerThreads, "post", server_threads, client_threads, message_lengths, message_concurrents);
}


criterion_group!(benches, bench_concurrent, bench_server);
criterion_main!(benches);
