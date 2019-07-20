use crate::Compat;
use core::{
    pin::Pin,
    task::{self, Poll},
};
use std::io;

impl<T> tokio_io::AsyncRead for Compat<T>
where
    T: futures_io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        futures_io::AsyncRead::poll_read(self.project().inner, cx, buf)
    }
}

impl<T> futures_io::AsyncRead for Compat<T>
where
    T: tokio_io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        tokio_io::AsyncRead::poll_read(self.project().inner, cx, buf)
    }
}

impl<T> tokio_io::AsyncBufRead for Compat<T>
where
    T: futures_io::AsyncBufRead,
{
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        cx: &mut task::Context,
    ) -> Poll<io::Result<&'a [u8]>> {
        futures_io::AsyncBufRead::poll_fill_buf(self.project().inner, cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        futures_io::AsyncBufRead::consume(self.project().inner, amt)
    }
}

impl<T> futures_io::AsyncBufRead for Compat<T>
where
    T: tokio_io::AsyncBufRead,
{
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        cx: &mut task::Context,
    ) -> Poll<io::Result<&'a [u8]>> {
        tokio_io::AsyncBufRead::poll_fill_buf(self.project().inner, cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        tokio_io::AsyncBufRead::consume(self.project().inner, amt)
    }
}

impl<T> tokio_io::AsyncWrite for Compat<T>
where
    T: futures_io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        futures_io::AsyncWrite::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        futures_io::AsyncWrite::poll_flush(self.project().inner, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        futures_io::AsyncWrite::poll_close(self.project().inner, cx)
    }
}

impl<T> futures_io::AsyncWrite for Compat<T>
where
    T: tokio_io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        tokio_io::AsyncWrite::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        tokio_io::AsyncWrite::poll_flush(self.project().inner, cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<io::Result<()>> {
        tokio_io::AsyncWrite::poll_shutdown(self.project().inner, cx)
    }
}
