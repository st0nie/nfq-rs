use nfq::{Queue, Verdict};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut queue = Queue::open().await?;
    queue.bind(0).await?;
    queue.set_recv_conntrack(0, true).await?;
    queue.set_recv_security_context(0, true).await?;
    queue.set_recv_uid_gid(0, true).await?;
    loop {
        let mut msg = queue.recv().await?;
        println!("{:#?}", msg);
        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg).await?;
    }
}
