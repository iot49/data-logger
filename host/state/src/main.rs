
use heapless::Vec as HeaplessVec;

fn x<const N: usize>(buf: &mut HeaplessVec::<u8, N>) {
    let _ = buf.push(5u8);
    let _ = buf.extend_from_slice(&[0u8, 3, 7, 8, 9, 10, 11]);
}

fn main() {
    const SZ1: usize = 10;

    let mut v1 = HeaplessVec::<u8,SZ1>::new();
    x::<SZ1>(&mut v1);
    println!("v1 = {:?}", v1);

    println!("x = {:?}", &v1[2..]);

}