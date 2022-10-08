use bytes::{BytesMut, BufMut};

fn main() {
    let mut buf = BytesMut::with_capacity(128);
    println!("buf capacity: {}, len: {}", buf.capacity(), buf.len());

    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);

    let a = buf.split();
    println!("buf: {:?}", a);

    buf.put(&b"goodbye world"[..]);

    let b = buf.split();
    println!("split: {:?}", b);

    println!("split: {:?}", buf);
    println!("buf capacity: {}, len: {}", buf.capacity(), buf.len());
    buf.clear();
    println!("after clear, buf capacity: {}, len: {}", buf.capacity(), buf.len());
    println!("split: {:?}", buf);
}
