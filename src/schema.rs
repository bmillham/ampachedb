// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct ObjectCountObjectTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct PlaylistTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct RatingObjectTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct RequestlistStatusEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct SearchTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct SongModeEnum;
}

diesel::table! {
    access_list (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        start -> Varbinary,
        #[max_length = 255]
        end -> Varbinary,
        level -> Unsigned<Smallint>,
        #[sql_name = "type"]
        #[max_length = 64]
        type_ -> Nullable<Varchar>,
        user -> Integer,
        enabled -> Unsigned<Tinyint>,
    }
}

diesel::table! {
    accounts (id) {
        #[max_length = 32]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 128]
        password -> Nullable<Varchar>,
    }
}

diesel::table! {
    album (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Text>,
        #[max_length = 32]
        prefix -> Nullable<Varchar>,
        #[max_length = 36]
        mbid -> Nullable<Varchar>,
        year -> Unsigned<Integer>,
        disk -> Nullable<Unsigned<Smallint>>,
    }
}

diesel::table! {
    artist (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Text>,
        #[max_length = 32]
        prefix -> Nullable<Varchar>,
        #[max_length = 1369]
        mbid -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        #[max_length = 64]
        placeformed -> Nullable<Varchar>,
        yearformed -> Nullable<Integer>,
        last_update -> Unsigned<Integer>,
    }
}

diesel::table! {
    broadcast (id) {
        id -> Unsigned<Integer>,
        user -> Unsigned<Integer>,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        is_private -> Unsigned<Tinyint>,
        song -> Unsigned<Integer>,
        started -> Unsigned<Tinyint>,
        listeners -> Unsigned<Integer>,
        #[max_length = 32]
        key -> Nullable<Varchar>,
    }
}

diesel::table! {
    catalog (id) {
        id -> Unsigned<Integer>,
        #[max_length = 128]
        name -> Nullable<Varchar>,
        #[max_length = 128]
        catalog_type -> Nullable<Varchar>,
        last_update -> Unsigned<Integer>,
        last_clean -> Nullable<Unsigned<Integer>>,
        last_add -> Unsigned<Integer>,
        enabled -> Unsigned<Tinyint>,
        #[max_length = 255]
        rename_pattern -> Nullable<Varchar>,
        #[max_length = 255]
        sort_pattern -> Nullable<Varchar>,
        #[max_length = 255]
        gather_types -> Nullable<Varchar>,
    }
}

diesel::table! {
    catalog_local (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        path -> Nullable<Varchar>,
        catalog_id -> Integer,
    }
}

diesel::table! {
    catalog_remote (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        uri -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        catalog_id -> Integer,
    }
}

diesel::table! {
    channel (id) {
        id -> Unsigned<Integer>,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        #[max_length = 256]
        url -> Nullable<Varchar>,
        #[max_length = 64]
        interface -> Nullable<Varchar>,
        port -> Unsigned<Integer>,
        fixed_endpoint -> Unsigned<Tinyint>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
        object_id -> Unsigned<Integer>,
        is_private -> Unsigned<Tinyint>,
        random -> Unsigned<Tinyint>,
        #[sql_name = "loop"]
        loop_ -> Unsigned<Tinyint>,
        #[max_length = 20]
        admin_password -> Nullable<Varchar>,
        start_date -> Unsigned<Integer>,
        max_listeners -> Unsigned<Integer>,
        peak_listeners -> Unsigned<Integer>,
        listeners -> Unsigned<Integer>,
        connections -> Unsigned<Integer>,
        #[max_length = 8]
        stream_type -> Nullable<Varchar>,
        bitrate -> Unsigned<Integer>,
        pid -> Unsigned<Integer>,
    }
}

diesel::table! {
    democratic (id) {
        id -> Unsigned<Integer>,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        cooldown -> Nullable<Unsigned<Tinyint>>,
        level -> Unsigned<Tinyint>,
        user -> Integer,
        primary -> Unsigned<Tinyint>,
        base_playlist -> Unsigned<Integer>,
    }
}

diesel::table! {
    dynamic_playlist (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        user -> Integer,
        date -> Unsigned<Integer>,
        #[sql_name = "type"]
        #[max_length = 128]
        type_ -> Nullable<Varchar>,
    }
}

diesel::table! {
    dynamic_playlist_data (id) {
        id -> Unsigned<Integer>,
        dynamic_id -> Unsigned<Integer>,
        #[max_length = 255]
        field -> Nullable<Varchar>,
        #[max_length = 64]
        internal_operator -> Nullable<Varchar>,
        #[max_length = 64]
        external_operator -> Nullable<Varchar>,
        #[max_length = 255]
        value -> Nullable<Varchar>,
    }
}

diesel::table! {
    groups (id) {
        #[max_length = 32]
        id -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    ip_history (id) {
        id -> Unsigned<Integer>,
        user -> Integer,
        #[max_length = 255]
        ip -> Nullable<Varbinary>,
        date -> Unsigned<Integer>,
        #[max_length = 255]
        agent -> Nullable<Varchar>,
    }
}

diesel::table! {
    listeners (id) {
        id -> Integer,
        current -> Nullable<Integer>,
        max -> Nullable<Integer>,
    }
}

diesel::table! {
    live_stream (id) {
        id -> Unsigned<Integer>,
        #[max_length = 128]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        site_url -> Nullable<Varchar>,
        #[max_length = 4096]
        url -> Nullable<Varchar>,
        genre -> Unsigned<Integer>,
        catalog -> Unsigned<Integer>,
        #[max_length = 32]
        codec -> Nullable<Varchar>,
    }
}

diesel::table! {
    localplay_httpq (id) {
        id -> Unsigned<Integer>,
        #[max_length = 128]
        name -> Nullable<Varchar>,
        owner -> Integer,
        #[max_length = 255]
        host -> Nullable<Varchar>,
        port -> Unsigned<Integer>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        access -> Unsigned<Smallint>,
    }
}

diesel::table! {
    localplay_mpd (id) {
        id -> Unsigned<Integer>,
        #[max_length = 128]
        name -> Nullable<Varchar>,
        owner -> Integer,
        #[max_length = 255]
        host -> Nullable<Varchar>,
        port -> Unsigned<Integer>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        access -> Unsigned<Smallint>,
    }
}

diesel::table! {
    localplay_shoutcast (id) {
        id -> Unsigned<Integer>,
        #[max_length = 128]
        name -> Nullable<Varchar>,
        owner -> Integer,
        #[max_length = 255]
        pid -> Nullable<Varchar>,
        #[max_length = 255]
        playlist -> Nullable<Varchar>,
        #[max_length = 255]
        local_root -> Nullable<Varchar>,
        access -> Unsigned<Smallint>,
    }
}

diesel::table! {
    mistags (id) {
        id -> Integer,
        track_id -> Integer,
        reported_by -> Text,
        reported -> Timestamp,
        artist -> Text,
        album -> Text,
        title -> Text,
        #[max_length = 2000]
        comments -> Varchar,
    }
}

diesel::table! {
    now_playing (id) {
        #[max_length = 64]
        id -> Varchar,
        object_id -> Unsigned<Integer>,
        #[max_length = 255]
        object_type -> Nullable<Varchar>,
        user -> Integer,
        expire -> Unsigned<Integer>,
        insertion -> Nullable<Integer>,
    }
}

diesel::table! {
    permissions (id) {
        #[max_length = 32]
        id -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    played (played_id) {
        played_id -> Integer,
        track_id -> Integer,
        date_played -> Timestamp,
        #[max_length = 255]
        played_by -> Nullable<Varchar>,
        played_by_me -> Bool,
    }
}

diesel::table! {
    player_control (id) {
        id -> Unsigned<Integer>,
        user -> Unsigned<Integer>,
        #[max_length = 32]
        cmd -> Nullable<Varchar>,
        #[max_length = 256]
        value -> Nullable<Varchar>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
        object_id -> Unsigned<Integer>,
        send_date -> Unsigned<Integer>,
    }
}

diesel::table! {
    playlist_data (id) {
        id -> Unsigned<Integer>,
        playlist -> Unsigned<Integer>,
        object_id -> Nullable<Unsigned<Integer>>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
        track -> Unsigned<Integer>,
    }
}

diesel::table! {
    preference (id) {
        id -> Unsigned<Integer>,
        #[max_length = 128]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        value -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        level -> Unsigned<Integer>,
        #[sql_name = "type"]
        #[max_length = 128]
        type_ -> Nullable<Varchar>,
        #[max_length = 128]
        catagory -> Nullable<Varchar>,
    }
}

diesel::table! {
    recommendation (id) {
        id -> Unsigned<Integer>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
        object_id -> Unsigned<Integer>,
        last_update -> Unsigned<Integer>,
    }
}

diesel::table! {
    recommendation_item (id) {
        id -> Unsigned<Integer>,
        recommendation -> Unsigned<Integer>,
        recommendation_id -> Nullable<Unsigned<Integer>>,
        #[max_length = 256]
        name -> Nullable<Varchar>,
        #[max_length = 256]
        rel -> Nullable<Varchar>,
        #[max_length = 1369]
        mbid -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    requestlist (ID) {
        ID -> Unsigned<Integer>,
        songID -> Unsigned<Integer>,
        t_stamp -> Datetime,
        #[max_length = 255]
        host -> Nullable<Varchar>,
        msg -> Nullable<Text>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        code -> Integer,
        ETA -> Datetime,
        #[max_length = 7]
        status -> VarChar,
    }
}

diesel::table! {
    session (id) {
        #[max_length = 256]
        id -> Varchar,
        #[max_length = 16]
        username -> Nullable<Varchar>,
        expire -> Unsigned<Integer>,
        value -> Longtext,
        #[max_length = 255]
        ip -> Nullable<Varbinary>,
        #[sql_name = "type"]
        #[max_length = 16]
        type_ -> Nullable<Varchar>,
        #[max_length = 255]
        agent -> Nullable<Varchar>,
    }
}

diesel::table! {
    session_stream (id) {
        #[max_length = 64]
        id -> Varchar,
        user -> Unsigned<Integer>,
        #[max_length = 255]
        agent -> Nullable<Varchar>,
        expire -> Unsigned<Integer>,
        #[max_length = 255]
        ip -> Nullable<Varbinary>,
    }
}

diesel::table! {
    share (id) {
        id -> Unsigned<Integer>,
        user -> Unsigned<Integer>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
        object_id -> Unsigned<Integer>,
        allow_stream -> Unsigned<Tinyint>,
        allow_download -> Unsigned<Tinyint>,
        expire_days -> Unsigned<Integer>,
        max_counter -> Unsigned<Integer>,
        #[max_length = 20]
        secret -> Nullable<Varchar>,
        counter -> Unsigned<Integer>,
        creation_date -> Unsigned<Integer>,
        lastvisit_date -> Unsigned<Integer>,
        #[max_length = 255]
        public_url -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    site_options (id) {
        id -> Integer,
        show_title -> Text,
        menu_background_color -> Text,
        menu_foreground_color -> Text,
        menu_highlight_background -> Text,
        menu_highlight_foreground -> Text,
        table_header_background -> Text,
        table_header_foreground -> Text,
        sort_header_background -> Text,
        sort_header_foreground -> Text,
        sort_odd_background -> Text,
        sort_odd_foreground -> Text,
        sort_even_background -> Text,
        sort_even_foreground -> Text,
        show_time -> Text,
        show_end -> Text,
        limit_requests -> Integer,
        offset -> Integer,
        catalog -> Text,
        show_time_dt -> Datetime,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    song (id) {
        id -> Unsigned<Integer>,
        file -> Nullable<Text>,
        catalog -> Unsigned<Integer>,
        album -> Unsigned<Integer>,
        year -> Integer,
        artist -> Unsigned<Integer>,
        title -> Nullable<Text>,
        bitrate -> Integer,
        rate -> Integer,
        #[max_length = 3]
        mode -> Nullable<Varchar>,
        size -> Unsigned<Integer>,
        time -> Unsigned<Smallint>,
        track -> Nullable<Unsigned<Smallint>>,
        #[max_length = 36]
        mbid -> Nullable<Varchar>,
        played -> Unsigned<Tinyint>,
        enabled -> Unsigned<Tinyint>,
        update_time -> Nullable<Unsigned<Integer>>,
        addition_time -> Nullable<Unsigned<Integer>>,
        modification_time -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    song_preview (id) {
        id -> Unsigned<Integer>,
        #[max_length = 256]
        session -> Nullable<Varchar>,
        artist -> Nullable<Integer>,
        #[max_length = 1369]
        artist_mbid -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 36]
        album_mbid -> Nullable<Varchar>,
        #[max_length = 36]
        mbid -> Nullable<Varchar>,
        disk -> Nullable<Integer>,
        track -> Nullable<Integer>,
        #[max_length = 255]
        file -> Nullable<Varchar>,
    }
}

diesel::table! {
    stream_playlist (id) {
        id -> Unsigned<Integer>,
        #[max_length = 256]
        sid -> Nullable<Varchar>,
        url -> Text,
        info_url -> Nullable<Text>,
        image_url -> Nullable<Text>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        author -> Nullable<Varchar>,
        #[max_length = 255]
        album -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Nullable<Varchar>,
        time -> Nullable<Smallint>,
        #[max_length = 32]
        codec -> Nullable<Varchar>,
    }
}

diesel::table! {
    suggestions (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        album -> Nullable<Varchar>,
        #[max_length = 255]
        artist -> Nullable<Varchar>,
        #[max_length = 255]
        suggestor -> Nullable<Varchar>,
        #[max_length = 2048]
        comments -> Nullable<Varchar>,
    }
}

diesel::table! {
    tag (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    tag_map (id) {
        id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
        object_id -> Unsigned<Integer>,
        #[max_length = 16]
        object_type -> Nullable<Varchar>,
        user -> Integer,
    }
}

diesel::table! {
    tmp_browse (sid, id) {
        id -> Integer,
        #[max_length = 128]
        sid -> Varchar,
        data -> Longtext,
        object_data -> Nullable<Longtext>,
    }
}

diesel::table! {
    tmp_playlist (id) {
        id -> Unsigned<Integer>,
        #[max_length = 256]
        session -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 32]
        type_ -> Nullable<Varchar>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    tmp_playlist_data (id) {
        id -> Unsigned<Integer>,
        tmp_playlist -> Unsigned<Integer>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
        object_id -> Unsigned<Integer>,
        track -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    update_info (key) {
        #[max_length = 128]
        key -> Varchar,
        #[max_length = 255]
        value -> Nullable<Varchar>,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 128]
        username -> Nullable<Varchar>,
        #[max_length = 128]
        fullname -> Nullable<Varchar>,
        #[max_length = 128]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        apikey -> Nullable<Varchar>,
        #[max_length = 128]
        password -> Nullable<Varchar>,
        access -> Unsigned<Tinyint>,
        disabled -> Unsigned<Tinyint>,
        last_seen -> Unsigned<Integer>,
        create_date -> Nullable<Unsigned<Integer>>,
        #[max_length = 128]
        validation -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_catalog (user) {
        user -> Unsigned<Integer>,
        catalog -> Unsigned<Integer>,
        level -> Unsigned<Smallint>,
    }
}

diesel::table! {
    user_flag (id) {
        id -> Unsigned<Integer>,
        user -> Integer,
        object_id -> Unsigned<Integer>,
        #[max_length = 32]
        object_type -> Nullable<Varchar>,
        date -> Unsigned<Integer>,
    }
}

diesel::table! {
    user_preference (user) {
        user -> Integer,
        preference -> Unsigned<Integer>,
        #[max_length = 255]
        value -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (uname) {
        #[max_length = 50]
        uname -> Varchar,
        #[max_length = 50]
        pword -> Varchar,
        administrator -> Nullable<Bool>,
        spword -> Nullable<Blob>,
    }
}

diesel::table! {
    wanted (id) {
        id -> Unsigned<Integer>,
        user -> Integer,
        artist -> Nullable<Integer>,
        #[max_length = 1369]
        artist_mbid -> Nullable<Varchar>,
        #[max_length = 36]
        mbid -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        year -> Nullable<Integer>,
        date -> Unsigned<Integer>,
        accepted -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    access_list,
    accounts,
    album,
    artist,
    broadcast,
    catalog,
    catalog_local,
    catalog_remote,
    channel,
    democratic,
    dynamic_playlist,
    dynamic_playlist_data,
    groups,
    ip_history,
    listeners,
    live_stream,
    localplay_httpq,
    localplay_mpd,
    localplay_shoutcast,
    mistags,
    now_playing,
    permissions,
    played,
    player_control,
    playlist_data,
    preference,
    recommendation,
    recommendation_item,
    requestlist,
    session,
    session_stream,
    share,
    site_options,
    song,
    song_preview,
    stream_playlist,
    suggestions,
    tag,
    tag_map,
    tmp_browse,
    tmp_playlist,
    tmp_playlist_data,
    update_info,
    user,
    user_catalog,
    user_flag,
    user_preference,
    users,
    wanted,
);

//diesel::joinable!(requestlist -> song(songID));
diesel::joinable!(song -> artist(artist));
