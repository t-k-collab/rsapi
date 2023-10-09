use crate::{
    interfaces::{repositories::members::MemberWithFamilyInfoModel, responses::CustomError},
    use_cases::members::find_member::FindMemberInputData,
};
use sqlx::{Error as sqlxErr, Pool, Postgres};

use super::MemberModel;

pub struct FindMemberRepository {
    pub pool: Pool<Postgres>,
}

impl FindMemberRepository {
    // this is not called from anywhere yet.
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

    pub async fn find_with_family_info(
        &self,
        input: FindMemberInputData,
    ) -> Result<Option<Vec<MemberWithFamilyInfoModel>>, CustomError> {
        println!("calling find member repository: {}", &input.member_id);
        let rows =
            sqlx::query_as::<Postgres, MemberWithFamilyInfoModel>("
            select m.member_id, m.first_name, m.middle_name, m.family_name, m.email, m.date_of_birth, fm.family_id, f.name
            from members m 
            join family_members fm on m.member_id = fm.member_id
            join families f on fm.family_id = f.family_id
            where m.member_id = $1
            ")
                .bind(input.member_id)
                .fetch_all(&self.pool)
                .await;
        println!("inner join result: {:#?}", &rows);

        match rows {
            Ok(res) => Ok(Some(res)),
            Err(sqlxErr::RowNotFound) => Ok(None),
            Err(err) => Err(CustomError::SqlxError {
                msg: err.to_string(), // REFACTOR Adjust error object.
            }),
            // Other errors related to SqlxError?
        }
    }
}
