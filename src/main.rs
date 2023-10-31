pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::{insert_into, prelude::*};
use dotenvy::dotenv;
use fake::{faker, Fake};
use std::env;

fn main() {
    let conn = &mut establish_connection();

    println!("seeding started");

    seed_db(conn);

    println!("seeding finished");
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("err connecting to {}", database_url))
}

fn seed_db(conn: &mut PgConnection) {
    // maintain this order
    create_default_test_master_user(conn);
    create_default_test_user(conn);

    for _ in 1..20 {
        fake_root_user_with_user_org(conn);
    }
}

fn hash_password(plain: String) -> String {
    // use the lowest cost (4) since we do not care about security of seeded data
    bcrypt::hash(plain, 4).unwrap().to_string()
}

fn create_default_test_master_user(conn: &mut PgConnection) {
    use schema::user::dsl::*;

    let test_master_user_access_level = fake_access_level(conn, true, None);

    insert_into(user)
        .values((
            username.eq("test_master_user"),
            password.eq(hash_password(String::from("testmasteruser"))),
            email.eq("rastercar.tests.001@gmail.com"),
            email_verified.eq(true),
            access_level_id.eq(test_master_user_access_level.id),
        ))
        .get_result::<models::User>(conn)
        .unwrap();
}

fn create_default_test_user(conn: &mut PgConnection) {
    let test_user_organization = {
        use schema::organization::dsl::*;

        insert_into(organization)
            .values((
                name.eq("test user org"),
                blocked.eq(false),
                billing_email.eq("testuser@gmail.com"),
                billing_email_verified.eq(true),
            ))
            .get_result::<models::Organization>(conn)
            .unwrap()
    };

    let test_user_access_level = fake_access_level(conn, true, Some(test_user_organization.id));

    {
        use schema::user::dsl::*;

        insert_into(user)
            .values((
                organization_id.eq(test_user_organization.id),
                access_level_id.eq(test_user_access_level.id),
                email.eq("rastercar.tests.002@gmail.com"),
                username.eq("test_user"),
                email_verified.eq(true),
                password.eq(hash_password(String::from("testuser"))),
                description.eq(faker::lorem::en::Words(1..3)
                    .fake::<Vec<String>>()
                    .join(" ")),
            ))
            .get_result::<models::User>(conn)
            .unwrap();
    };
}

fn fake_organization(conn: &mut PgConnection) -> models::Organization {
    use schema::organization::dsl::*;

    insert_into(organization)
        .values((
            name.eq(faker::company::en::CompanyName().fake::<String>()),
            blocked.eq(false),
            billing_email.eq(faker::internet::en::SafeEmail().fake::<String>()),
            billing_email_verified.eq(true),
        ))
        .get_result::<models::Organization>(conn)
        .unwrap()
}

fn fake_access_level(
    conn: &mut PgConnection,
    is_fixed_value: bool,
    organization_id_value: Option<i32>,
) -> models::AccessLevel {
    use schema::access_level::dsl::*;

    insert_into(access_level)
        .values((
            name.eq(faker::lorem::en::Word().fake::<String>()),
            is_fixed.eq(is_fixed_value),
            description.eq(faker::lorem::en::Words(2..7)
                .fake::<Vec<String>>()
                .join(" ")),
            permissions.eq(vec!["UPDATE_ORGANIZATION"]),
            organization_id.eq(organization_id_value),
        ))
        .get_result::<models::AccessLevel>(conn)
        .unwrap()
}

fn fake_root_user_with_user_org(conn: &mut PgConnection) -> models::User {
    use schema::user::dsl::*;

    let user_org = fake_organization(conn);
    let access_level = fake_access_level(conn, true, Some(user_org.id));

    let created_user = insert_into(user)
        .values((
            organization_id.eq(user_org.id),
            access_level_id.eq(access_level.id),
            email.eq(faker::internet::en::SafeEmail().fake::<String>()),
            username.eq(faker::internet::en::Username().fake::<String>()),
            email_verified.eq(faker::boolean::en::Boolean(50).fake::<bool>()),
            password.eq(hash_password(
                faker::internet::en::Password(10..50).fake::<String>(),
            )),
            description.eq(faker::lorem::en::Words(1..3)
                .fake::<Vec<String>>()
                .join(" ")),
        ))
        .get_result::<models::User>(conn)
        .unwrap();

    {
        use schema::organization::dsl::*;

        diesel::update(organization)
            .filter(id.eq(user_org.id))
            .set(owner_id.eq(created_user.id))
            .execute(conn)
            .unwrap();
    }

    created_user
}
