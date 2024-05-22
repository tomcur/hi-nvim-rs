use axum::{
    headers::{HeaderMapExt, IfModifiedSince, LastModified},
    http::{header, Request, Response, StatusCode},
};
use pin_project::pin_project;
use std::{
    future::Future,
    task::{Context, Poll},
    time::SystemTime,
};
use tower::{Layer, Service};

/// Layer that applies the [`CacheControl`] middleware which applies and enforces caching headers.
#[derive(Debug, Clone, Copy)]
pub struct CacheControlLayer {
    last_modified: SystemTime,
}

impl CacheControlLayer {
    /// Creates a new [`CacheControlLayer`].
    pub fn new() -> Self {
        CacheControlLayer {
            last_modified: SystemTime::now(),
        }
    }
}

impl<S> Layer<S> for CacheControlLayer {
    type Service = CacheControl<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CacheControl::new(inner, self.last_modified)
    }
}

enum ResponseKind {
    NotModified,
    Forward,
}

/// Cache control service. Set `last-modified` response header to the given `last_modified` time.
/// Requests with an `if-modified-since` no later than the `last_modified` time are responded to
/// with NOT MODIFIED.
#[derive(Clone, Debug)]
pub struct CacheControl<S> {
    inner: S,
    last_modified: SystemTime,
    last_modified_header: LastModified,
}

impl<S> CacheControl<S> {
    pub fn new(inner: S, last_modified: SystemTime) -> Self {
        Self {
            inner,
            last_modified,
            last_modified_header: last_modified.into(),
        }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for CacheControl<S>
where
    ResBody: Default,
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let if_modified_since: Option<IfModifiedSince> = axum::headers::Header::decode(
            &mut req.headers().get(header::IF_MODIFIED_SINCE).iter().copied(),
        )
        .ok();

        let response_kind = match if_modified_since {
            Some(if_modified_since) => {
                let if_modified_since: SystemTime = if_modified_since.into();
                if if_modified_since > self.last_modified {
                    ResponseKind::Forward
                } else {
                    ResponseKind::NotModified
                }
            }
            None => ResponseKind::Forward,
        };

        ResponseFuture {
            inner: self.inner.call(req),
            response_kind,
            last_modified_header: self.last_modified_header,
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    inner: F,
    response_kind: ResponseKind,
    last_modified_header: LastModified,
}

impl<F, B, E> Future for ResponseFuture<F>
where
    B: Default,
    F: Future<Output = Result<Response<B>, E>>,
{
    type Output = F::Output;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if matches!(self.response_kind, ResponseKind::NotModified) {
            let mut res = Response::new(B::default());
            res.headers_mut().typed_insert(self.last_modified_header);
            *res.status_mut() = StatusCode::NOT_MODIFIED;
            return Poll::Ready(Ok(res));
        }

        let this = self.project();

        let mut res = match this.inner.poll(cx)? {
            Poll::Ready(t) => t,
            Poll::Pending => return Poll::Pending,
        };

        res.headers_mut().typed_insert(*this.last_modified_header);
        Poll::Ready(Ok(res))
    }
}
