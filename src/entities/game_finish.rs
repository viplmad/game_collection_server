use sea_query::Iden;

#[derive(Iden)]
#[iden = "GameFinish"]
pub enum GameFinishIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "date"]
    Date,
}
