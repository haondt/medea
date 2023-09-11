use digest::{Mac, OutputSizeUser};

pub trait DynHmacDigest {
    fn update(&mut self, data: &[u8]);
    fn finalize_into_bytes(&mut self) -> Vec<u8>;
}

impl<T: Mac + OutputSizeUser + Clone> DynHmacDigest for T {
    fn update(&mut self, data: &[u8]) {
        self.update(data)
    }

    fn finalize_into_bytes(&mut self) -> Vec<u8> {
        self.clone().finalize().into_bytes().to_vec().to_owned()
    }
}