CREATE TABLE tribesim.simulation_yearly_global_stats
(
    `simulation_id` String,
    `year` UInt32,
    `total_memes_known` UInt64,
    `avg_memes_known` Float64,
    `avg_trick_efficiency` Float64,
    `avg_brain_volume` Float64,
    `avg_meme_size` Float64,
    `event_time` DateTime DEFAULT now()
)
ENGINE = MergeTree
PARTITION BY simulation_id
ORDER BY (simulation_id, year)
SETTINGS index_granularity = 8192;

CREATE TABLE tribesim.simulation_yearly_meme_stats
(
    `simulation_id` String,
    `year` UInt32,
    `meme_kind` LowCardinality(String),
    `avg_meme_efficiency` Float64,
    `avg_meme_size` Float64,
    `event_time` DateTime DEFAULT now()
)
ENGINE = MergeTree
PARTITION BY simulation_id
ORDER BY (simulation_id, year, meme_kind)
SETTINGS index_granularity = 8192;
