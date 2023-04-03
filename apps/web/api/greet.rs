use vercel_runtime::{run, Body, Error, Request, RequestExt, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let query_string = req.query_string_parameters();
    let user = query_string.first("user").unwrap_or("Guest").to_owned();

    let body = format!("<h1 class=\"btn btn-primary\">Hello, {}</h1>", { user }).into();

    let response = Response::builder().status(StatusCode::OK).body(body)?;

    Ok(response)
}
