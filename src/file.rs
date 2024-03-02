pub mod FileManager {
    #[derive(Debug)]   
    pub struct file<T>{
        pub fileName : String,
        pub fileSize : T,
        pub timestamp : String,
    }
}