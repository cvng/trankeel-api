use crate::File;
use crate::FileId;
use crate::Rent;

pub type Notice = File; // alias for a File

pub type NoticeId = FileId; // alias for a FileId

pub type NoticeWithRent = (Notice, Rent);
