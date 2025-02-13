#![allow(unused)]

struct Configuration {
    retry: u32,
    timeout: u32,
}

trait RequestClient {
    fn send(&self);
}

struct GrpcRequestClient {
    config: Configuration,
}

impl RequestClient for GrpcRequestClient {
    fn send(&self) {
        println!("Sending request by gRPC")
    }
}

struct HttpRequestClient {
    config: Configuration,
}

impl RequestClient for HttpRequestClient {
    fn send(&self) {
        println!("Sending request by HTTP")
    }
}

struct Service<T: RequestClient> {
    client: T,
}

impl<T: RequestClient> Service<T> {
    fn call(&self) {
        self.client.send();
    }
}

fn main() {
    let config = Configuration {
        retry: 3,
        timeout: 10,
    };
    let grpc_client = GrpcRequestClient { config };
    let grpc_service = Service {
        client: grpc_client,
    };
    grpc_service.call();

    let config = Configuration {
        retry: 3,
        timeout: 60,
    };
    let http_client = HttpRequestClient { config };
    let http_service = Service {
        client: http_client,
    };
    http_service.call();
}
