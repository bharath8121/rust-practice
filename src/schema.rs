// @generated automatically by Diesel CLI.

diesel::table! {
    developers (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        name -> Text,
        phone_number -> Nullable<Text>,
        description -> Text,
    }
}

diesel::table! {
    email_confirmations (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        email -> Nullable<Text>,
        token -> Nullable<Text>,
        verification_type -> Nullable<Text>,
        expires_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    flyway_schema_history (installed_rank) {
        installed_rank -> Int4,
        #[max_length = 50]
        version -> Nullable<Varchar>,
        #[max_length = 200]
        description -> Varchar,
        #[sql_name = "type"]
        #[max_length = 20]
        type_ -> Varchar,
        #[max_length = 1000]
        script -> Varchar,
        checksum -> Nullable<Int4>,
        #[max_length = 100]
        installed_by -> Varchar,
        installed_on -> Timestamp,
        execution_time -> Int4,
        success -> Bool,
    }
}

diesel::table! {
    payment_plans (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        plan_name -> Nullable<Text>,
        description -> Text,
        plan_format -> Text,
    }
}

diesel::table! {
    properties (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        street_address -> Text,
        country -> Text,
        zipcode -> Text,
        price -> Numeric,
        status -> Text,
    }
}

diesel::table! {
    property_developers (developer_id, property_id) {
        developer_id -> Int8,
        property_id -> Int8,
    }
}

diesel::table! {
    property_images (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        image_url -> Text,
        property_id -> Int8,
    }
}

diesel::table! {
    property_payment_plans (property_id, payment_plan_id) {
        property_id -> Int8,
        payment_plan_id -> Int8,
    }
}

diesel::table! {
    spring_session (primary_id) {
        #[max_length = 36]
        primary_id -> Bpchar,
        #[max_length = 36]
        session_id -> Bpchar,
        creation_time -> Int8,
        last_access_time -> Int8,
        max_inactive_interval -> Int4,
        expiry_time -> Int8,
        #[max_length = 100]
        principal_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    spring_session_attributes (session_primary_id, attribute_name) {
        #[max_length = 36]
        session_primary_id -> Bpchar,
        #[max_length = 200]
        attribute_name -> Varchar,
        attribute_bytes -> Bytea,
    }
}

diesel::table! {
    t_app_instance_sizes (id) {
        id -> Int2,
        cloud_provider -> Int2,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        slug -> Nullable<Varchar>,
        #[max_length = 50]
        cpu_type -> Nullable<Varchar>,
        cpus -> Nullable<Int2>,
        #[max_length = 50]
        memory_bytes -> Nullable<Varchar>,
        usd_per_month -> Nullable<Float8>,
        usd_per_second -> Nullable<Float8>,
        #[max_length = 100]
        tier_slug -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    t_cloud_provider (id) {
        id -> Int2,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        logo -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    t_cloud_provider_region (id) {
        id -> Int2,
        cloud_provider -> Int2,
        #[max_length = 100]
        label -> Nullable<Varchar>,
        #[max_length = 100]
        slug -> Nullable<Varchar>,
        #[max_length = 100]
        flag -> Nullable<Varchar>,
        #[max_length = 100]
        continent -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    t_db_metadata (id) {
        id -> Int4,
        table_export_data -> Jsonb,
        #[max_length = 100]
        tenant_id -> Varchar,
        #[max_length = 150]
        note -> Nullable<Varchar>,
        created_date -> Timestamptz,
        last_updated_date -> Timestamptz,
        created_by_id -> Int4,
        last_updated_by_id -> Int4,
        version -> Int4,
        #[max_length = 500]
        table_name_prefixes -> Nullable<Varchar>,
        #[max_length = 100]
        table_name_delimiters -> Nullable<Varchar>,
        #[max_length = 100]
        column_name_delimiters -> Nullable<Varchar>,
        #[max_length = 20]
        db_type -> Varchar,
    }
}

diesel::table! {
    t_deployment (id) {
        id -> Int4,
        cloud_provider -> Int2,
        region_code -> Nullable<Int2>,
        deleted -> Nullable<Bool>,
        version -> Int4,
        project_id -> Int4,
        no_of_containers -> Nullable<Int2>,
        #[max_length = 100]
        tenant_id -> Varchar,
        created_date -> Timestamptz,
        last_updated_date -> Timestamptz,
        created_by_id -> Int4,
        last_updated_by_id -> Int4,
        #[max_length = 100]
        digital_ocean_api_key -> Nullable<Varchar>,
        #[max_length = 100]
        environment -> Nullable<Varchar>,
        app_instance_id -> Nullable<Int2>,
    }
}

diesel::table! {
    t_non_valid_email_domain (domain_name) {
        #[max_length = 500]
        domain_name -> Varchar,
    }
}

diesel::table! {
    t_oas_file (id) {
        id -> Int4,
        #[max_length = 100]
        file_name -> Varchar,
        #[max_length = 250]
        file_s3_link -> Varchar,
        created_date -> Timestamptz,
        user_id -> Int4,
        tenant_id -> Int4,
        project_id -> Int4,
    }
}

diesel::table! {
    t_object_type (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
        version -> Int4,
        project_id -> Int4,
        fields -> Nullable<Jsonb>,
        #[max_length = 100]
        tenant_id -> Varchar,
        created_date -> Timestamptz,
        last_updated_date -> Timestamptz,
        created_by_id -> Int4,
        last_updated_by_id -> Int4,
    }
}

diesel::table! {
    t_project (id) {
        id -> Int4,
        #[max_length = 100]
        project_name -> Varchar,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
        #[max_length = 100]
        project_key -> Varchar,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        #[max_length = 100]
        tenant_id -> Varchar,
        created_date -> Timestamptz,
        last_updated_date -> Timestamptz,
        #[max_length = 20]
        api_type -> Nullable<Varchar>,
        #[max_length = 50]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    t_query_type (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
        version -> Int4,
        project_id -> Int4,
        query_attributes -> Nullable<Jsonb>,
        #[max_length = 100]
        tenant_id -> Varchar,
        created_date -> Timestamptz,
        last_updated_date -> Timestamptz,
        created_by_id -> Int4,
        last_updated_by_id -> Int4,
    }
}

diesel::table! {
    t_rest_data_source (id) {
        id -> Int4,
        #[max_length = 100]
        data_source_name -> Varchar,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
        #[max_length = 300]
        file_url -> Nullable<Varchar>,
        version -> Int4,
        #[max_length = 100]
        tenant_id -> Varchar,
        created_date -> Timestamptz,
        last_updated_date -> Timestamptz,
        created_by_id -> Int4,
        last_updated_by_id -> Int4,
    }
}

diesel::table! {
    t_rest_schema_object (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        deleted -> Nullable<Bool>,
        version -> Int4,
        project_id -> Int4,
        fields -> Nullable<Jsonb>,
        #[max_length = 100]
        tenant_id -> Varchar,
        created_date -> Timestamptz,
        last_updated_date -> Timestamptz,
        created_by_id -> Int4,
        last_updated_by_id -> Int4,
        #[max_length = 100]
        table_name -> Nullable<Varchar>,
        auto_incr_field_present -> Nullable<Bool>,
        composite_pk -> Nullable<Bool>,
        #[max_length = 50]
        pk_type -> Nullable<Varchar>,
        additional_imports -> Nullable<Jsonb>,
        #[max_length = 10]
        db_object_type -> Nullable<Varchar>,
        #[max_length = 20]
        object_source -> Nullable<Varchar>,
        #[max_length = 50]
        db_object_source_config -> Nullable<Varchar>,
    }
}

diesel::table! {
    t_role (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 100]
        display -> Varchar,
    }
}

diesel::table! {
    t_tenant (id) {
        id -> Int4,
        #[max_length = 250]
        tenant_id -> Varchar,
        #[max_length = 200]
        name -> Varchar,
        created_date -> Timestamptz,
        deleted -> Bool,
    }
}

diesel::table! {
    t_user (id) {
        id -> Int4,
        #[max_length = 20]
        mobile -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Varchar,
        last_updated_date -> Timestamptz,
        deleted -> Bool,
        created_date -> Timestamptz,
        last_login_date -> Nullable<Timestamptz>,
        role_id -> Int4,
        #[max_length = 100]
        password -> Nullable<Varchar>,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        email_verified -> Nullable<Bool>,
        tenant_id -> Int4,
        #[max_length = 64]
        verification_token -> Nullable<Varchar>,
        verification_expiry_date -> Nullable<Timestamptz>,
        enabled -> Nullable<Bool>,
    }
}

diesel::table! {
    task (id) {
        id -> Text,
        description -> Text,
        completed -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        user_email -> Nullable<Text>,
        password_enc -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        verified -> Nullable<Bool>,
        real_estate_agent -> Nullable<Bool>,
        phone_number -> Nullable<Text>,
    }
}

diesel::joinable!(property_developers -> developers (developer_id));
diesel::joinable!(property_developers -> properties (property_id));
diesel::joinable!(property_images -> properties (property_id));
diesel::joinable!(property_payment_plans -> payment_plans (payment_plan_id));
diesel::joinable!(property_payment_plans -> properties (property_id));
diesel::joinable!(spring_session_attributes -> spring_session (session_primary_id));
diesel::joinable!(t_oas_file -> t_project (project_id));
diesel::joinable!(t_oas_file -> t_tenant (tenant_id));
diesel::joinable!(t_oas_file -> t_user (user_id));
diesel::joinable!(t_user -> t_tenant (tenant_id));

diesel::allow_tables_to_appear_in_same_query!(
    developers,
    email_confirmations,
    flyway_schema_history,
    payment_plans,
    properties,
    property_developers,
    property_images,
    property_payment_plans,
    spring_session,
    spring_session_attributes,
    t_app_instance_sizes,
    t_cloud_provider,
    t_cloud_provider_region,
    t_db_metadata,
    t_deployment,
    t_non_valid_email_domain,
    t_oas_file,
    t_object_type,
    t_project,
    t_query_type,
    t_rest_data_source,
    t_rest_schema_object,
    t_role,
    t_tenant,
    t_user,
    task,
    users,
);
