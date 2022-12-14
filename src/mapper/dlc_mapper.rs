use crate::entities::{DLCWithDate, DLC};
use crate::models::{DLCAvailableDTO, DLCDTO};

impl From<DLC> for DLCDTO {
    fn from(dlc: DLC) -> Self {
        Self {
            id: dlc.id,
            name: dlc.name,
            base_game_id: dlc.base_game_id,
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            cover_url: None,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
        }
    }
}

impl From<DLCDTO> for DLC {
    fn from(dlc: DLCDTO) -> Self {
        Self {
            id: dlc.id,
            user_id: -1,
            name: dlc.name,
            base_game_id: dlc.base_game_id,
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
        }
    }
}

impl From<DLCWithDate> for DLCAvailableDTO {
    fn from(dlc: DLCWithDate) -> Self {
        Self {
            id: dlc.id,
            name: dlc.name,
            base_game_id: dlc.base_game_id,
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            cover_url: None,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
            available_date: dlc.query_date,
        }
    }
}
