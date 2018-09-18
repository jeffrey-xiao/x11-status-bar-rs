use super::{Error, Result};
use bincode::serialize;
use server;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::net::Shutdown;

fn send_request(request: server::Request) -> Result<()> {
    let mut socket = UnixStream::connect(server::SOCKET_PATH)
        .map_err(|err| Error::new("connecting to server", err))?;
    let serialized_request = serialize(&request)
        .map_err(|err| Error::new("serializing request", err))?;
    socket.write_all(&serialized_request)
        .map_err(|err| Error::new("sending request", err))?;
    socket.shutdown(Shutdown::Write)
        .map_err(|err| Error::new("sending request", err))?;

    let mut buffer = Vec::with_capacity(server::MAX_REQUEST_SIZE);
    socket.read_to_end(&mut buffer)
        .map_err(|err| Error::new("reading reply", err))?;
    Ok(())
}

pub fn show(profile: String, value: f64) -> Result<()> {
    send_request(server::Request::Show { profile, value })
}

pub fn hide() -> Result<()> {
    send_request(server::Request::Hide)
}

pub fn stop() -> Result<()> {
    send_request(server::Request::Stop)
}
