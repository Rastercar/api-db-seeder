use chrono::offset::Utc;
use chrono::DateTime;
use diesel::{Identifiable, Queryable, Selectable};
use diesel_geometry::sql_types::*;
use ipnetwork::IpNetwork;

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::access_level)]
pub struct AccessLevel {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub is_fixed: bool,
    pub permissions: Vec<Option<String>>,
    pub organization_id: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::organization)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub blocked: bool,
    pub billing_email: String,
    pub billing_email_verified: bool,
    pub confirm_billing_email_token: Option<String>,
    pub owner_id: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::sim_card)]
pub struct SimCard {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub phone_number: String,
    pub ssn: String,
    pub apn_address: String,
    pub apn_user: String,
    pub apn_password: String,
    pub pin: Option<String>,
    pub pin2: Option<String>,
    pub puk: Option<String>,
    pub puk2: Option<String>,
    pub organization_id: i32,
    pub tracker_id: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::user)]
pub struct User {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub reset_password_token: Option<String>,
    pub confirm_email_token: Option<String>,
    pub profile_picture: Option<String>,
    pub description: Option<String>,
    pub organization_id: Option<i32>,
    pub access_level_id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::vehicle)]
pub struct Vehicle {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub plate: String,
    pub photo: Option<String>,
    pub model_year: Option<i16>,
    pub fabrication_year: Option<i16>,
    pub chassis_number: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub additional_info: Option<String>,
    pub organization_id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::vehicle_tracker)]
pub struct VehicleTracker {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub model: String,
    pub imei: String,
    pub organization_id: i32,
    pub vehicle_id: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(tracker_id))]
#[diesel(table_name = crate::schema::vehicle_tracker_last_location)]
pub struct VehicleTrackerLastLocation {
    pub tracker_id: i32,
    pub time: DateTime<Utc>,
    pub point: Point,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(time, tracker_id))]
#[diesel(table_name = crate::schema::vehicle_tracker_location)]
pub struct VehicleTrackerLocation {
    pub time: DateTime<Utc>,
    pub tracker_id: i32,
    pub point: Point,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(session_token))]
#[diesel(table_name = crate::schema::session)]
pub struct Session {
    pub public_id: i32,
    pub session_token: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub user_agent: String,
    pub ip: IpNetwork,
    pub user_id: i32,
}
