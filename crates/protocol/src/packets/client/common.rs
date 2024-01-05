use super::*;

packets!(
    ClientInformation {
        locale String;
        view_distance u8;
        chat_mode VarInt;
        chat_colors bool;
        displayed_skin_parts u8;
        main_hand VarInt;
        enable_text_filtering bool;
        allow_server_listings bool;
    }

    KeepAliveResponse {
        id u64;
    }
);
