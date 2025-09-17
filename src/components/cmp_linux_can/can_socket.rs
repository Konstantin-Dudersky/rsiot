use futures::StreamExt;
use socketcan::SocketOptions;

use super::{CanFilter, CanFrame, CanSettings, Error};

pub enum CanSocket {
    Classic(socketcan::tokio::CanSocket),
    Fd(socketcan::tokio::CanFdSocket),
}
impl CanSocket {
    pub fn open(ifname: &str, can_settings: CanSettings) -> Result<Self, Error> {
        if !can_settings.mode_fd {
            let socket = socketcan::tokio::CanSocket::open(ifname).map_err(Error::SocketOpen)?;
            Ok(Self::Classic(socket))
        } else {
            let socket = socketcan::tokio::CanFdSocket::open(ifname).map_err(Error::SocketOpen)?;
            Ok(Self::Fd(socket))
        }
    }

    pub fn set_filters(&mut self, filters: &[CanFilter]) -> Result<(), Error> {
        if filters.is_empty() {
            return Ok(());
        }

        let filters: Vec<socketcan::CanFilter> = filters.iter().map(|f| (*f).into()).collect();
        match self {
            CanSocket::Classic(socket) => socket.set_filters(&filters).map_err(Error::SetFilters),
            CanSocket::Fd(socket) => socket.set_filters(&filters).map_err(Error::SetFilters),
        }
    }

    pub async fn write_frame(&self, frame: CanFrame) -> Result<(), Error> {
        let res = match self {
            CanSocket::Classic(socket) => {
                let frame: socketcan::CanFrame = frame.try_into()?;
                socket.write_frame(frame).await
            }
            CanSocket::Fd(socket) => {
                let frame: socketcan::CanFrame = frame.try_into()?;
                socket.write_frame(&frame).await
            }
        };
        res.map_err(Error::WriteFrame)
    }

    pub async fn next(&mut self) -> Option<Result<CanFrame, Error>> {
        match self {
            CanSocket::Classic(socket) => {
                let frame = socket.next().await?;

                let frame = match frame {
                    Ok(v) => v,
                    Err(e) => return Some(Err(Error::ReadFrame(e))),
                };

                let frame: Result<CanFrame, Error> = frame.try_into();
                Some(frame)
            }
            CanSocket::Fd(_socket) => unimplemented!(),
        }
    }
}
