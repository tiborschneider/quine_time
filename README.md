# It's Quine Time

A rust program that prints its own source code, but also the current time. This project is heavily inspired by this [video](https://www.youtube.com/watch?v=plFwBqBYpcY), but implemented in rust.

## How to run it

The program `quine.rs` contains the nicely formatted source code, without the quine blob. The program `compile.rs` reads `quine.rs`, re-formats it, and embeds the quine-blob (escaped source code) within the source itself. The provided `Makefile` does everything for you:

```sh
make
./time # or ./matrix
```

Then, you can copy the source code, and compile it with `rustc`:

```sh
rustc quine_copy.rs && ./quine_copy
```

## The compiled source code

Here is the compiled source code for the time:

```rust
use std::time::*;const F:[u32;11]=[31599,5961,25255,29647,23497,31118,14831,29332,31727,31694,1040,];const E:&str="\x1b[";fn p(d:
[u32;8],x:&mut usize,y:&mut usize,c:char){let mut col=false;if*y>1&&*x>1{let dx=(*x-2)/4;let dy=(*y-2)/2;let di=dx/4;let ddx=dx%4;
col=di<8&&ddx<3&&dy<5&&(d[di]>>(2-ddx)+(4-dy)*3)&1==1;}print!("{E}{}m{c}",if col{47}else{49});*x+=1;if c=='\n'{*x=0;*y+=1;}}fn main(
){let mut d=[0u32;8];d[2]=F[10];d[5]=F[10];loop{print!("{E}2J{E}1;1H{E}90m");let t=SystemTime::now().duration_since(UNIX_EPOCH).
unwrap().as_secs()as usize;d[0]=F[t/3600%60/10];d[1]=F[t/3600%10];d[3]=F[t/10%60/10];d[4]=F[t/60%10];d[6]=F[t%60/10];d[7]=F[t%10];
let mut x=0;let mut y=0;for c in S.chars(){if Some(c)==char::from_u32(126){for c in S.chars(){if c=='"'||c=='\\'{p(d,&mut x,&mut y,
'\\')}p(d,&mut x,&mut y,c)}}else{p(d,&mut x,&mut y,c)}}println!("\x1b[0m");std::thread::sleep(Duration::from_secs(1))}}const S:&str=
"use std::time::*;const F:[u32;11]=[31599,5961,25255,29647,23497,31118,14831,29332,31727,31694,1040,];const E:&str=\"\\x1b[\";fn p(d:
[u32;8],x:&mut usize,y:&mut usize,c:char){let mut col=false;if*y>1&&*x>1{let dx=(*x-2)/4;let dy=(*y-2)/2;let di=dx/4;let ddx=dx%4;
col=di<8&&ddx<3&&dy<5&&(d[di]>>(2-ddx)+(4-dy)*3)&1==1;}print!(\"{E}{}m{c}\",if col{47}else{49});*x+=1;if c=='\\n'{*x=0;*y+=1;}}fn main(
){let mut d=[0u32;8];d[2]=F[10];d[5]=F[10];loop{print!(\"{E}2J{E}1;1H{E}90m\");let t=SystemTime::now().duration_since(UNIX_EPOCH).
unwrap().as_secs()as usize;d[0]=F[t/3600%60/10];d[1]=F[t/3600%10];d[3]=F[t/10%60/10];d[4]=F[t/60%10];d[6]=F[t%60/10];d[7]=F[t%10];
let mut x=0;let mut y=0;for c in S.chars(){if Some(c)==char::from_u32(126){for c in S.chars(){if c=='\"'||c=='\\\\'{p(d,&mut x,&mut y,
'\\\\')}p(d,&mut x,&mut y,c)}}else{p(d,&mut x,&mut y,c)}}println!(\"\\x1b[0m\");std::thread::sleep(Duration::from_secs(1))}}const S:&str=
\"~\";";
```

here is the compiled source code for the matrix version:

```rust
use std::{fs::*,io::{self,*},thread::*,time::*,};fn p(d:&Vec<Vec<u8>>,x:&mut usize,y:&mut usize,c:char){let z="\x1b[";let(
a,b)=match d[*y][*x]{2=>(1,92),1=>(1,32),_=>(21,90),};print!("{}{}m{}{}m{}",z,a,z,b,c);*x+=1;if c=='\n'{*x=0;*y+=1;}}fn
main()->io::Result<()>{let mut rng=File::open("/dev/urandom")?;let d=vec![0u8;256];let mut d:Vec<_>=std::iter::repeat(&d).
take(20).cloned().collect();let mut b=[0];loop{let mut row=d[0].clone();for x in row.iter_mut(){if*x>0{*x-=1;}}for _ in 0
..10{rng.read_exact(&mut b)?;row[b[0]as usize]=2u8;}d.pop();d.insert(0,row);print!("\x1b[2J\x1b[1;1H");let mut x=0;let mut
y=0;for c in S.chars(){if Some(c)==char::from_u32(126){for c in S.chars(){if c=='"'||c=='\\'{p(&d,&mut x,&mut y,'\\')}p(&d
,&mut x,&mut y,c)}}else{p(&d,&mut x,&mut y,c)}}println!("\x1b[0m");sleep(Duration::from_millis(99))}Ok(())}const S:&str=
"use std::{fs::*,io::{self,*},thread::*,time::*,};fn p(d:&Vec<Vec<u8>>,x:&mut usize,y:&mut usize,c:char){let z=\"\\x1b[\";let(
a,b)=match d[*y][*x]{2=>(1,92),1=>(1,32),_=>(21,90),};print!(\"{}{}m{}{}m{}\",z,a,z,b,c);*x+=1;if c=='\\n'{*x=0;*y+=1;}}fn
main()->io::Result<()>{let mut rng=File::open(\"/dev/urandom\")?;let d=vec![0u8;256];let mut d:Vec<_>=std::iter::repeat(&d).
take(20).cloned().collect();let mut b=[0];loop{let mut row=d[0].clone();for x in row.iter_mut(){if*x>0{*x-=1;}}for _ in 0
..10{rng.read_exact(&mut b)?;row[b[0]as usize]=2u8;}d.pop();d.insert(0,row);print!(\"\\x1b[2J\\x1b[1;1H\");let mut x=0;let mut
y=0;for c in S.chars(){if Some(c)==char::from_u32(126){for c in S.chars(){if c=='\"'||c=='\\\\'{p(&d,&mut x,&mut y,'\\\\')}p(&d
,&mut x,&mut y,c)}}else{p(&d,&mut x,&mut y,c)}}println!(\"\\x1b[0m\");sleep(Duration::from_millis(99))}Ok(())}const S:&str=
\"~\";";
```
