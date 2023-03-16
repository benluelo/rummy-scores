export function get_score(player_and_score: PlayerAndScore): [Round, number | null][] {
	return [
		['ace', player_and_score.ace],
		['two', player_and_score.two],
		['three', player_and_score.three],
		['four', player_and_score.four],
		['five', player_and_score.five],
		['six', player_and_score.six],
		['seven', player_and_score.seven],
		['eight', player_and_score.eight],
		['nine', player_and_score.nine],
		['ten', player_and_score.ten],
		['jack', player_and_score.jack],
		['queen', player_and_score.queen],
		['king', player_and_score.king]
	];
}
