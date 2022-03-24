use clap::Parser;
use std::path::PathBuf;
use ostree_ext::ostree;
use hyper_staticfile::Static;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    repo: PathBuf,
    remote_path: String,
}

fn main() {
    let args = Args::parse();

    let static_ = Static::new(args.repo);

    let make_service = make_service_fn(|_| {
        let static_ = static_.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, static_.clone())))
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = hyper::Server::bind(&addr).serve(make_service);
    eprintln!("Doc server running on http://{}/", addr);
    server.await.expect("Server failed");

    // for _ in 0..args.count {
        // println!("Hello {}!", args.name)
    // }
}
