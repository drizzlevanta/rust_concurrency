fn main() {
    threads::demo();
    channels::demo();
    mutex::demo();
}

mod channels;
mod mutex;
mod threads;
