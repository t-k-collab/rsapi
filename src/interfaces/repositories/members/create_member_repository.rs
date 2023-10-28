use crate::{
    interfaces::{
        repositories::members::{FamilyModel, MemberModel},
        responses::CustomError,
    },
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
    pub async fn create(&self, input: CreateMemberInputData) -> MemberModel {
        // ) -> Result<Option<MemberModel>, CustomError> {
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
        println!("create a member record: {:#?}", row);

        // TODO get member call find_member repository
        let find_row = sqlx::query_as::<Postgres, MemberModel>(
            "SELECT * FROM members WHERE email = $1 AND first_name = $2 AND family_name = $3",
        )
        .bind(&input.email)
        .bind(&input.first_name)
        .bind(&input.family_name)
        .fetch_one(&self.pool)
        .await;
        println!(
            "find a member after creating a member result: {:#?}",
            &find_row
        );

        // TODO create family record using use_case in controller
        let family_row = sqlx::query(
            "INSERT INTO families (
                name,
                pass_code
                ) VALUES (
                $1,
                $2
                )",
        )
        .bind(&input.family_unit_name)
        .bind(&input.family_unit_pass_code)
        .execute(&self.pool)
        .await;
        println!("create a family record: {:#?}", family_row);

        // TODO fetch family info or family_id
        let created_family_rows = sqlx::query_as::<Postgres, FamilyModel>(
            "
        select family_id, name
        from families
        where name = $1 and pass_code = $2
        ",
        )
        .bind(input.family_unit_name)
        .bind(input.family_unit_pass_code)
        .fetch_one(&self.pool)
        .await;
        println!("select from family result: {:#?}", &created_family_rows);

        // let family =
        // TODO create family_member record with member_id and family_id
        // if created_family_rows() {}

        let _ = tx.commit().await;

        // TODO return Result
        match find_row {
            Ok(res) => Ok(Some(res)),
            Err(sqlxErr::RowNotFound) => Ok(None),
            Err(err) => Err(CustomError::SqlxError {
                msg: err.to_string(), // REFACTOR Adjust error object.
            }),
            // Other errors related to SqlxError?
        };

        // TODO implement Later
        let utc = Utc::now();
        MemberModel {
            member_id: 1,
            family_name: "input.family_name".to_string(),
            middle_name: "input.middle_name".to_string(),
            first_name: "input.first_name".to_string(),
            date_of_birth: Some(utc.date_naive()),
            email: "member@email.com".to_string(),
            password: "password".to_string(),
            created_at: utc.naive_utc(),
            updated_at: utc.naive_utc(),
        }
    }
}
