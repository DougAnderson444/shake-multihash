// #[cfg(any(feature = "shake"))]
#[macro_export]
macro_rules! derive_rustcrypto_shaker {
    ($module:ty, $name:ident, $size:expr) => {
        /// Multihash hasher.

        #[derive(Debug)]
        pub struct $name {
            state: $module,
            digest: [u8; $size],
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    state: Default::default(),
                    digest: [0; $size],
                }
            }
        }

        impl ::multihash::Hasher for $name {
            fn update(&mut self, input: &[u8]) {
                use sha3::digest::Update;
                self.state.update(input)
            }

            fn finalize(&mut self) -> &[u8] {
                use sha3::digest::ExtendableOutput;
                use sha3::digest::XofReader;
                let mut reader = self.state.clone().finalize_xof();
                let mut res = vec![0u8; $size];
                reader.read(&mut res);
                let digest_out = &mut self.digest[..$size];
                digest_out.copy_from_slice(&res);
                digest_out
            }

            fn reset(&mut self) {
                let Self { state, .. } = Self::default();
                self.state = state;
            }
        }

        impl core2::io::Write for $name {
            fn write(&mut self, buf: &[u8]) -> core2::io::Result<usize> {
                use multihash::Hasher as _;

                self.update(buf);
                Ok(buf.len())
            }

            fn flush(&mut self) -> core2::io::Result<()> {
                Ok(())
            }
        }
    };
}
