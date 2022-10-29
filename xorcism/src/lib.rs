use std::borrow::Borrow;
#[cfg(feature = "io")]
use std::io::{Read, Write};
/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<Key> {
    key: KeyStream<Key>,
}
impl<Key> Xorcism<Key>
where
    Key: AsRef<[u8]>,
{
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new(key: Key) -> Xorcism<Key> {
        Self {
            key: KeyStream::new(key),
        }
    }
    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for item in data.iter_mut() {
            *item ^= self.key.next().unwrap();
        }
    }
    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data, T>(&'_ mut self, data: Data) -> XorStream<'_, Key, Data::IntoIter>
    where
        Data: IntoIterator<Item = T>,
        T: Borrow<u8>,
    {
        XorStream::new(&mut self.key, data.into_iter())
    }
    #[cfg(feature = "io")]
    pub fn reader<T: Read>(self, read: T) -> XorAdapter<Key, T> {
        XorAdapter::new(self, read)
    }
    #[cfg(feature = "io")]
    pub fn writer<T: Write>(self, write: T) -> XorAdapter<Key, T> {
        XorAdapter::new(self, write)
    }
}
/// Adds xoring capabilities to any `Read` or `Write`
#[cfg(feature = "io")]
pub struct XorAdapter<Key, Inner>
where
    Key: AsRef<[u8]>,
{
    xorcism: Xorcism<Key>,
    inner: Inner,
    // Since empty vecs don't allocate, this adds a few more bytes in `XorAdapter` instead of writing two separate structs
    buf: Vec<u8>,
}
#[cfg(feature = "io")]
impl<Key, Inner> XorAdapter<Key, Inner>
where
    Key: AsRef<[u8]>,
{
    fn new(xorcism: Xorcism<Key>, inner: Inner) -> Self {
        Self {
            xorcism,
            inner,
            buf: Vec::new(),
        }
    }
}
#[cfg(feature = "io")]
impl<Key, Reader> Read for XorAdapter<Key, Reader>
where
    Key: AsRef<[u8]>,
    Reader: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.inner.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[0..res]);
        Ok(res)
    }
}
#[cfg(feature = "io")]
impl<Key, Reader> Write for XorAdapter<Key, Reader>
where
    Key: AsRef<[u8]>,
    Reader: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend(self.xorcism.munge(buf));
        let written = self.inner.write(&self.buf)?;
        self.buf.clear();
        Ok(written)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
pub struct XorStream<'a, Key, It> {
    key_iterator: &'a mut KeyStream<Key>,
    data_iterator: It,
}
impl<'a, Key, It, T> XorStream<'a, Key, It>
where
    Key: AsRef<[u8]>,
    It: Iterator<Item = T>,
    T: Borrow<u8>,
{
    fn new(key_iterator: &'a mut KeyStream<Key>, data_iterator: It) -> Self {
        Self {
            key_iterator,
            data_iterator,
        }
    }
}
impl<'a, Key, It, T> Iterator for XorStream<'a, Key, It>
where
    Key: AsRef<[u8]>,
    It: Iterator<Item = T>,
    T: Borrow<u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.data_iterator.next()?;
        let next = next.borrow();
        let next = *next ^ self.key_iterator.next().unwrap();
        Some(next)
    }
}
/// Wraps an `AsRef<[u8]>` and provide a repeating iterator over its contents
///
#[derive(Clone)]
struct KeyStream<Key> {
    inner: Key,
    offset: usize,
}
impl<Key> KeyStream<Key> {
    fn new(inner: Key) -> Self {
        Self { inner, offset: 0 }
    }
}
impl<Key> Iterator for KeyStream<Key>
where
    Key: AsRef<[u8]>,
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.as_ref();
        let val = key[self.offset];
        self.offset = (self.offset + 1) % key.len();
        Some(val)
    }
}
