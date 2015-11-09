
extern crate libc;
use std::env;
use std::ptr;
use std::ffi;
use std::io;

fn main() {
  let mut arg = env::args();
  let progname = arg.next().unwrap();
  match arg.next() {
    None => println!("Usage: {} <space in byte unit, 10m>", progname),
    Some(size) => lock_pages(parse_size(size))
  }
}

fn parse_size(size: String) -> usize {
  let mut n: u64 = 0;
  for c in size.chars() {
    match c.to_digit(10) {
      Some(x) => n = (n * 10) + x as u64,
      None => match c {
        'k' => n = n * 1024,
        'm' => n = n * 1024 * 1024,
        'g' => n = n * 1024 * 1024 * 1024,
        't' => n = n * 1024 * 1024 * 1024 * 1024,
        _ => println!("Unknown unit '{}', assuming bytes.", c)
      }
    }
  };
  n as usize
}

fn lock_pages(size: usize) {
  let page_size = get_page_size();
  let pages = size / page_size;
  println!("Locking {} pages ({} bytes)…", pages, size);
  let pointer = mmap_anonymous(size);
  mlock(pointer, size);
  for p in 0..pages {
    let offset = (p * page_size) as isize;
    unsafe { ptr::read(pointer.offset(offset)) };
  }
  println!("Done. Press return to stop…");
  io::stdin().read_line(&mut String::new()).unwrap();
  munmap(pointer, size);
}

fn get_page_size() -> usize {
  unsafe {
    libc::sysconf(libc::_SC_PAGESIZE) as usize
  }
}

fn mmap_anonymous(size: usize) -> *mut libc::c_void {
  let addr: *mut libc::c_void = ptr::null_mut();
  let len: libc::size_t = size;
  let prot: libc::c_int = libc::PROT_READ | libc::PROT_WRITE;
  let flags: libc::c_int = libc::MAP_PRIVATE | libc::MAP_ANON;
  let fd: libc::c_int = -1;
  let offset: libc::off_t = 0;
  let pointer = unsafe { libc::mmap(addr, len, prot, flags, fd, offset) };
  if pointer == libc::MAP_FAILED {
    err(1, "mmap");
  }
  pointer
}

fn mlock(pointer: *const libc::c_void, size: usize) {
  let ret = unsafe { libc::mlock(pointer, size) };
  if ret != 0 {
    err(2, "mlock");
  }
}

fn munmap(pointer: *mut libc::c_void, size: usize) {
  unsafe {
    libc::munmap(pointer, size);
  }
}

fn err(code: i32, msg: &str) -> ! {
  unsafe {
    libc::perror(ffi::CString::new(msg).unwrap().as_ptr());
  }
  std::process::exit(code);
}
