use futures::StreamExt;
use socketcan::{BlockingCan, Socket, SocketOptions};

use super::{CanFilter, CanFrame, CanSettings, Error};

pub enum CanSocketAsync {
    Classic(socketcan::tokio::CanSocket),
    Fd(socketcan::tokio::CanFdSocket),
}
impl CanSocketAsync {
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
        // Получение фреймов ошибок
        match self {
            Self::Classic(socket) => {
                socket
                    .set_error_filter_accept_all()
                    .map_err(Error::SetFilters)?;
            }
            Self::Fd(_) => todo!(),
        }

        if filters.is_empty() {
            return Ok(());
        }

        let filters: Vec<socketcan::CanFilter> = filters.iter().map(|f| (*f).into()).collect();
        match self {
            Self::Classic(socket) => socket.set_filters(&filters).map_err(Error::SetFilters),
            Self::Fd(socket) => socket.set_filters(&filters).map_err(Error::SetFilters),
        }
    }

    pub async fn write_frame(&self, frame: CanFrame) -> Result<(), Error> {
        let res = match self {
            Self::Classic(socket) => {
                let frame: socketcan::CanFrame = frame.try_into()?;
                socket.write_frame(frame).await
            }
            Self::Fd(socket) => {
                let frame: socketcan::CanFrame = frame.try_into()?;
                socket.write_frame(&frame).await
            }
        };
        res.map_err(Error::WriteFrame)
    }

    pub async fn next(&mut self) -> Option<Result<CanFrame, Error>> {
        match self {
            Self::Classic(socket) => {
                let frame = socket.next().await?;

                let frame = match frame {
                    Ok(v) => v,
                    Err(e) => return Some(Err(Error::ReadFrame(e))),
                };

                let frame: CanFrame = frame.into();
                Some(Ok(frame))
            }
            Self::Fd(_socket) => unimplemented!(),
        }
    }
}

pub enum CanSocketSync {
    Classic(socketcan::CanSocket),
    Fd(socketcan::CanFdSocket),
}
impl CanSocketSync {
    pub fn open(ifname: &str, can_settings: CanSettings) -> Result<Self, Error> {
        if !can_settings.mode_fd {
            let socket = socketcan::CanSocket::open(ifname).map_err(Error::SocketOpen)?;
            Ok(Self::Classic(socket))
        } else {
            let socket = socketcan::CanFdSocket::open(ifname).map_err(Error::SocketOpen)?;
            Ok(Self::Fd(socket))
        }
    }

    pub fn set_filters(&mut self, filters: &[CanFilter]) -> Result<(), Error> {
        // Получение фреймов ошибок
        match self {
            Self::Classic(socket) => {
                socket
                    .set_error_filter_accept_all()
                    .map_err(Error::SetFilters)?;
            }
            Self::Fd(_) => todo!(),
        }

        if filters.is_empty() {
            return Ok(());
        }

        let filters: Vec<socketcan::CanFilter> = filters.iter().map(|f| (*f).into()).collect();
        match self {
            Self::Classic(socket) => socket.set_filters(&filters).map_err(Error::SetFilters),
            Self::Fd(socket) => socket.set_filters(&filters).map_err(Error::SetFilters),
        }
    }

    pub fn transmit(&mut self, frame: CanFrame) -> Result<(), Error> {
        let res = match self {
            Self::Classic(socket) => {
                let frame: socketcan::CanFrame = frame.try_into()?;
                socket.transmit(&frame)
            }
            Self::Fd(_socket) => {
                // let frame: socketcan::CanAnyFrame = frame.try_into()?;
                // socket.write_frame(&frame).await
                todo!()
            }
        };
        res.map_err(Error::WriteFrame2)
    }

    pub fn receive(&mut self) -> Result<CanFrame, Error> {
        match self {
            Self::Classic(socket) => {
                let frame = socket.read_frame();

                let frame: socketcan::CanFrame = match frame {
                    Ok(v) => v,
                    Err(e) => return Err(Error::ReadFrame2(e)),
                };

                let frame = frame.into();
                Ok(frame)
            }
            Self::Fd(_socket) => unimplemented!(),
        }
    }
}
