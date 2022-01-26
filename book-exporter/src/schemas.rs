table! {
    #[sql_name = "Novel"]
    novels (id) {
        id -> Integer,
        #[sql_name = "novelSourceName"]
        novel_source_name -> Text,
        #[sql_name = "narouNovelSourceId"]
        narou_novel_source_id -> Nullable<Integer>,
        #[sql_name = "novel18NovelSourceId"]
        novel18_novel_source_id -> Nullable<Integer>,
        #[sql_name = "hamelnNovelSourceId"]
        hameln_novel_source_id -> Nullable<Integer>,
        #[sql_name = "kakuyomuNovelSourceId"]
        kakuyomu_novel_source_id -> Nullable<Integer>,
    }
}

table! {
    #[sql_name = "Episode"]
    episodes (id) {
        id -> Integer,
        #[sql_name = "novelId"]
        novel_id -> Integer,
        #[sql_name = "episodeSourceName"]
        episode_source_name -> Text,
        #[sql_name = "narouEpisodeSourceId"]
        narou_episode_source_id -> Nullable<Integer>,
        #[sql_name = "novel18EpisodeSourceId"]
        novel18_episode_source_id -> Nullable<Integer>,
        #[sql_name = "hamelnEpisodeSourceId"]
        hameln_episode_source_id -> Nullable<Integer>,
        #[sql_name = "kakuyomuEpisodeSourceId"]
        kakuyomu_episode_source_id -> Nullable<Integer>,
    }
}

table! {
    #[sql_name = "EpisodeSourceNarou"]
    episode_source_narou(id) {
        id -> Integer,
        #[sql_name = "episodeId"]
        episode_id -> Integer,
        #[sql_name = "novelId"]
        novel_id-> Integer,
        #[sql_name = "title"]
        title -> Text,
        #[sql_name = "body"]
        body -> Text,
        #[sql_name = "uid"]
        part -> Integer,
    }
}

table! {
    #[sql_name = "EpisodeSourceNovel18"]
    episode_source_novel18(id) {
        id -> Integer,
        #[sql_name = "episodeId"]
        episode_id -> Integer,
        #[sql_name = "novelId"]
        novel_id-> Integer,
        #[sql_name = "title"]
        title -> Text,
        #[sql_name = "body"]
        body -> Text,
        #[sql_name = "uid"]
        part -> Integer,
    }
}

table! {
    #[sql_name = "EpisodeSourceHameln"]
    episode_source_hameln(id) {
        id -> Integer,
        #[sql_name = "episodeId"]
        episode_id -> Integer,
        #[sql_name = "novelId"]
        novel_id-> Integer,
        #[sql_name = "title"]
        title -> Text,
        #[sql_name = "body"]
        body -> Text,
        #[sql_name = "uid"]
        part -> Integer,
    }
}

table! {
    #[sql_name = "EpisodeSourceKakuyomu"]
    episode_source_kakuyomu(id) {
        id -> Integer,
        #[sql_name = "episodeId"]
        episode_id -> Integer,
        #[sql_name = "novelId"]
        novel_id-> Integer,
        #[sql_name = "title"]
        title -> Text,
        #[sql_name = "body"]
        body -> Text,
        #[sql_name = "uid"]
        part -> Integer,
    }
}
