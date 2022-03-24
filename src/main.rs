use clap::Parser;
use std::path::PathBuf;
use ostree_ext::ostree;
use hyper_staticfile::Static;
use hyper::service::{make_service_fn, service_fn};
use futures_util::future;
use std::net::SocketAddr;
use hyper::{Server};
use anyhow::{Context, Result, bail};
use std::net::{TcpStream};
use ssh2::Session;
use std::io::{self, Write};
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    repo: PathBuf,
    #[clap(short, long, parse(from_os_str))]
    remote_path: PathBuf,
    #[clap(short, long)]
    host: String,
    #[clap(short, long, parse(try_from_str), default_value_t = 22)]
    port: usize,
    #[clap(short, long)]
    username: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !args.repo.exists() {
        bail!("Path does not exist: {}", args.repo.display());
    };

    let static_ = Static::new(args.repo);
    
    // TODO: So many clones
    let make_service = make_service_fn(|_| {
        let static_ = static_.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| static_.clone().serve(req)))
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let server = Server::bind(&addr).serve(make_service);

    let remote_tcp = TcpStream::connect(args.host + &args.port.to_string()).context("Could not connect to remote server")?;
    let mut sess = Session::new().context("Could not create SSH session")?;
    sess.set_tcp_stream(remote_tcp);
    sess.handshake().context("Could not handshake with remote server")?;
    sess.userauth_agent(&args.username).context("Unable to authenticate with remote server")?;

    let mut channel = sess.channel_session().context("Unable to get channel with remote server")?;
    channel.exec(format!("ostree remote add --if-not-exists --repo {} --no-gpg-verify --no-sign-verify _remote https://uwu.neko", args.remote_path.display())).unwrap();
    channel.exec(format!("ostree pull --repo {} --mirror --url {} _remote", args.remote_path.display(), "")).unwrap();

    let (listener, _) = sess.channel_forward_listen(addr.port(), Some("127.0.0.1"), None).unwrap();

    loop {
        let channel = listener.accept()?;
        server::builder();
    }

    // io::stdout().write_all_vectored(channel);
    // channel.read_to_end();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());


    server.await.context("Unable to start local http file server")
}
