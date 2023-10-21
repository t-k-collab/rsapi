use crate::{
    interfaces::repositories::members::MemberModel,
    use_cases::members::create_member::CreateMemberInputData,
};
use chrono::Utc;
use sqlx::{Pool, Postgres};

pub struct CreateMemberRepository {
    pub pool: Pool<Postgres>,
}

// trait DataAccessInterface {
//     fn access_db(&self) -> Self;
// }

impl CreateMemberRepository {
    pub async fn create(&self, input: CreateMemberInputData) -> MemberModel {
        println!("inserting a member data into db.");

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

        // TODO implement Later
        let utc = Utc::now();
        MemberModel {
            member_id: 1,
            family_name: "input.family_name".to_string(),
            middle_name: "input.middle_name".to_string(),
            first_name: "input.first_name".to_string(),
            date_of_birth: utc.date_naive(),
            email: "member@email.com".to_string(),
            password: "password".to_string(),
            created_at: utc.naive_utc(),
            updated_at: utc.naive_utc(),
        }
    }
}
