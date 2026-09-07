// Enables f16/f128 automatically whenever building on nightly.
#![cfg_attr(nightly, feature(f16, f128))]

use std::io::{Read, Write, Result as IoResult};

/// A trait for reading endian dependant bytes and transforming it into a
/// concrete type.
/// 
/// Inspired by Nightly PR [#156984](https://github.com/rust-lang/rust/issues/156984)
pub trait FromEndianBytes : Sized {
    /// Reads `Self` from a `&mut impl Read` in **LITTLE ENDIAN** order.
    fn read_le_from(r: &mut impl Read) -> IoResult<Self>;
    /// Reads `Self` from a `&mut impl Read` in **BIG ENDIAN** order.
    fn read_be_from(r: &mut impl Read) -> IoResult<Self>;
    /// Reads `Self` from a `&mut impl Read` in **NATIVE ENDIAN** order.
    fn read_ne_from(r: &mut impl Read) -> IoResult<Self>;
}

impl FromEndianBytes for bool {
    #[inline]
    fn read_be_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
    #[inline]
    fn read_le_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
    #[inline]
    fn read_ne_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}

impl FromEndianBytes for i8 {
    #[inline]
    fn read_be_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
    #[inline]
    fn read_le_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
    #[inline]
    fn read_ne_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }
}

impl FromEndianBytes for u8 {
    #[inline]
    fn read_be_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    #[inline]
    fn read_le_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    #[inline]
    fn read_ne_from(r: &mut impl Read) -> IoResult<Self> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

macro_rules! impl_from_endian_bytes {
    ($($t:ty),*$(,)?) => {$(
        impl FromEndianBytes for $t {
            #[inline]
            fn read_le_from(r: &mut impl Read) -> IoResult<Self> {
                let mut buf = [0u8; size_of::<Self>()];
                r.read_exact(&mut buf)?;
                Ok(<$t>::from_le_bytes(buf))
            }
            #[inline]
            fn read_be_from(r: &mut impl Read) -> IoResult<Self> {
                let mut buf = [0u8; size_of::<Self>()];
                r.read_exact(&mut buf)?;
                Ok(<$t>::from_be_bytes(buf))
            }
            #[inline]
            fn read_ne_from(r: &mut impl Read) -> IoResult<Self> {
                let mut buf = [0u8; size_of::<Self>()];
                r.read_exact(&mut buf)?;
                Ok(<$t>::from_ne_bytes(buf))
            }
        }
    )*};
}

impl_from_endian_bytes!(u16, i16, u32, i32, u64, i64, u128, i128, f32, f64);

#[cfg(nightly)]
impl_from_endian_bytes!(f16, f128);

/// A trait for writing endian dependant bytes of a Sized type to a [Write] object.
/// 
/// Inspired by Nightly PR [#156984](https://github.com/rust-lang/rust/issues/156984)
pub trait ToEndianBytes : Sized {
    /// Writes `Self` to a `&mut impl Write` in **LITTLE ENDIAN**.
    fn write_le_to(&self, w: &mut impl Write) -> IoResult<()>;
    /// Writes `Self` to a `&mut impl Write` in **BIG ENDIAN**.
    fn write_be_to(&self, w: &mut impl Write) -> IoResult<()>;
    /// Writes `Self` to a `&mut impl Write` in **NATIVE ENDIAN**.
    fn write_ne_to(&self, w: &mut impl Write) -> IoResult<()>;
}

impl ToEndianBytes for bool {
    #[inline]
    fn write_be_to(&self, w: &mut impl Write) -> IoResult<()> {
        let buf = [match self {true => 1, false => 0}];
        w.write_all(&buf)
    }
    #[inline]
    fn write_le_to(&self, w: &mut impl Write) -> IoResult<()> {
        let buf = [match self {true => 1, false => 0}];
        w.write_all(&buf)
    }
    #[inline]
    fn write_ne_to(&self, w: &mut impl Write) -> IoResult<()> {
        let buf = [match self {true => 1, false => 0}];
        w.write_all(&buf)
    }
}

impl ToEndianBytes for u8 {
    #[inline]
    fn write_be_to(&self, w: &mut impl Write) -> IoResult<()> {
        w.write_all(&[*self])
    }
    #[inline]
    fn write_le_to(&self, w: &mut impl Write) -> IoResult<()> {
        w.write_all(&[*self])
    }
    #[inline]
    fn write_ne_to(&self, w: &mut impl Write) -> IoResult<()> {
        w.write_all(&[*self])
    }
}

impl ToEndianBytes for i8 {
    #[inline]
    fn write_be_to(&self, w: &mut impl Write) -> IoResult<()> {
        let buf = [*self as u8];
        w.write_all(&buf)
    }
    #[inline]
    fn write_le_to(&self, w: &mut impl Write) -> IoResult<()> {
        let buf = [*self as u8];
        w.write_all(&buf)
    }
    #[inline]
    fn write_ne_to(&self, w: &mut impl Write) -> IoResult<()> {
        let buf = [*self as u8];
        w.write_all(&buf)
    }
}

macro_rules! impl_to_endian_bytes {
    ($($t:ty),*$(,)?) => {$(
        impl ToEndianBytes for $t {
            #[inline]
            fn write_le_to(&self, w: &mut impl Write) -> IoResult<()> {
                let buf = self.to_le_bytes();
                w.write_all(&buf)
            }
            #[inline]
            fn write_be_to(&self, w: &mut impl Write) -> IoResult<()> {
                let buf = self.to_be_bytes();
                w.write_all(&buf)
            }
            #[inline]
            fn write_ne_to(&self, w: &mut impl Write) -> IoResult<()> {
                let buf = self.to_ne_bytes();
                w.write_all(&buf)
            }
        }
    )*};
}

impl_to_endian_bytes!(u16, i16, u32, i32, u64, i64, u128, i128, f32, f64);

#[cfg(nightly)]
impl_to_endian_bytes!(f16, f128);

/// This trait is used to automatically implement read_be, read_le and read_ne
/// to any type that is `impl Read + Sized`.
pub trait ReadExt : Read + Sized {
    /// Read and return a type (e.g. an integer) in big-endian order.
    ///
    /// You can specify the type with turbofish (`reader.read_le::<u64>()`), or let type inference
    /// determine the type based on how the return value gets used.
    ///
    /// Like [`read_exact`], if this function encounters an "end of file" before reading the desired
    /// number of bytes, it returns an error of the kind [`ErrorKind::UnexpectedEof`].
    ///
    /// [`ErrorKind::UnexpectedEof`]: std::io::ErrorKind::UnexpectedEof
    /// [`read_exact`]: std::io::Read::read_exact
    fn read_be<T: FromEndianBytes>(&mut self) -> IoResult<T> {
        T::read_be_from(self)
    }
    /// Read and return a type (e.g. an integer) in little-endian order.
    ///
    /// You can specify the type with turbofish (`reader.read_be::<u64>()`), or let type inference
    /// determine the type based on how the return value gets used.
    ///
    /// Like [`read_exact`], if this function encounters an "end of file" before reading the desired
    /// number of bytes, it returns an error of the kind [`ErrorKind::UnexpectedEof`].
    ///
    /// [`ErrorKind::UnexpectedEof`]: std::io::ErrorKind::UnexpectedEof
    /// [`read_exact`]: std::io::Read::read_exact
    fn read_le<T: FromEndianBytes>(&mut self) -> IoResult<T> {
        T::read_le_from(self)
    }
    /// Read and return a type (e.g. an integer) in native-endian order.
    ///
    /// You can specify the type with turbofish (`reader.read_ne::<u64>()`), or let type inference
    /// determine the type based on how the return value gets used.
    ///
    /// Like [`read_exact`], if this function encounters an "end of file" before reading the desired
    /// number of bytes, it returns an error of the kind [`ErrorKind::UnexpectedEof`].
    ///
    /// [`ErrorKind::UnexpectedEof`]: std::io::ErrorKind::UnexpectedEof
    /// [`read_exact`]: std::io::Read::read_exact
    fn read_ne<T: FromEndianBytes>(&mut self) -> IoResult<T> {
        T::read_ne_from(self)
    }
}

impl<R : Read + Sized> ReadExt for R {}

/// This trait is used to automatically implement write_be, write_le and write_ne
/// to any type that is `impl Write + Sized`.
pub trait WriteExt : Write + Sized {
    /// Write a type (e.g. a integer) in little-endian order.
    /// 
    /// # Errors
    ///
    /// This function will return the first error of
    /// non-[`ErrorKind::Interrupted`] kind that [`write`] returns.
    ///
    /// [`write`]: std::io::Write::write
    fn write_le<T: ToEndianBytes>(&mut self, item: &T) -> IoResult<()> {
        item.write_le_to(self)
    }
    /// Write a type (e.g. a integer) in big-endian order.
    /// 
    /// # Errors
    ///
    /// This function will return the first error of
    /// non-[`ErrorKind::Interrupted`] kind that [`write`] returns.
    ///
    /// [`write`]: std::io::Write::write
    fn write_be<T: ToEndianBytes>(&mut self, item: &T) -> IoResult<()> {
        item.write_be_to(self)
    }
    /// Write a type (e.g. a integer) in native-endian order.
    /// 
    /// # Errors
    ///
    /// This function will return the first error of
    /// non-[`ErrorKind::Interrupted`] kind that [`write`] returns.
    ///
    /// [`write`]: std::io::Write::write
    fn write_ne<T: ToEndianBytes>(&mut self, item: &T) -> IoResult<()> {
        item.write_ne_to(self)
    }
}

impl<W: Write + Sized> WriteExt for W {}