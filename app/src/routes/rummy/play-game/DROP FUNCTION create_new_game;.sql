DROP FUNCTION create_new_game;

CREATE FUNCTION create_new_game(player_ids INT[]) RETURNS int AS $$
    DECLARE
        new_game_id int;
        player_id int;
    BEGIN
        INSERT INTO public.games DEFAULT VALUES RETURNING games.game_id INTO new_game_id;

        FOREACH player_id IN ARRAY player_ids
        LOOP
            INSERT INTO player_scores_to_player_and_game_id(player_id, game_id) VALUES (player_id, new_game_id);
        END LOOP;

        RETURN new_game_id;
    END;
$$ LANGUAGE plpgsql;