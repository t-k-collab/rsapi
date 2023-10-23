use crate::{
    interfaces::{repositories::members::MemberModel, responses::CustomError},
    use_cases::members::create_member::CreateMemberInputData,
};
use chrono::Utc;
use sqlx::{Error as sqlxErr, Pool, Postgres};

pub struct CreateMemberRepository {
    pub pool: Pool<Postgres>,
}

// trait DataAccessInterface {
//     fn access_db(&self) -> Self;
// }

impl CreateMemberRepository {
    pub async fn create(
        &self,
        input: CreateMemberInputData,
    ) -> Result<Option<MemberModel>, CustomError> {
        println!("inserting a member data into db.");

        // TODO remove unwrap and handle Result
        // let mut tx = self.pool.begin().await;
        let tx = self.pool.begin().await.unwrap();

        let row = sqlx::query(
            "INSERT INTO members (
                first_name,
                middle_name,
                family_name,
                date_of_birth,
                email,
                password
                ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
                )",
        )
        .bind(&input.first_name)
        .bind(&input.middle_name)
        .bind(&input.family_name)
        .bind(input.date_of_birth)
        .bind(&input.email)
        .bind(&input.password)
        .execute(&self.pool)
        .await;
        println!("create a member result: {:#?}", row);

        // TODO get member call find_member repository
        let find_row = sqlx::query_as::<Postgres, MemberModel>(
            "SELECT * FROM members WHERE email = $1 AND first_name = $2 AND family_name = $3",
        )
        .bind(&input.email)
        .bind(&input.first_name)
        .bind(&input.family_name)
        .fetch_one(&self.pool)
        .await;
        println!("find a member after creating a member result: {:#?}", row);

        let _ = tx.commit().await;

        match find_row {
            Ok(res) => Ok(Some(res)),
            Err(sqlxErr::RowNotFound) => Ok(None),
            Err(err) => Err(CustomError::SqlxError {
                msg: err.to_string(), // REFACTOR Adjust error object.
            }),
            // Other errors related to SqlxError?
        }
    }
}
