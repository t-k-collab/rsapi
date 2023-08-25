use crate::use_cases::members::find_member::FindMemberInputData;
use sqlx::{Pool, Postgres};

use super::MemberModel;

pub struct FindMemberRepository {
    pub pool: Pool<Postgres>,
}

impl FindMemberRepository {
    pub async fn find(&self, input: FindMemberInputData) -> MemberModel {
        println!("find a member data from db.");
        let row =
            sqlx::query_as::<Postgres, MemberModel>("SELECT * FROM members WHERE member_id = $1")
                .bind(input.member_id)
                .fetch_one(&self.pool)
                .await;
        println!("result: {:#?}", row);
        // TODO Error handling
        let member = row.unwrap();

        MemberModel {
            member_id: member.member_id,
            first_name: member.first_name,
            middle_name: member.middle_name,
            family_name: member.family_name,
            date_of_birth: member.date_of_birth,
            email: member.email,
            password: member.password,
            created_at: member.created_at,
            updated_at: member.updated_at,
        }
    }
}
