<script lang="ts">
	import { page } from '$app/stores';
	import { client } from '../../../Components/client';
	import { onMount } from 'svelte';
	import { rounds } from '../../../Components/models';
	import { get_score } from '../../../Components/utils';

	let id: number = parseInt($page.params.id);

	let game_completed = false;

	let current_round_index: number = 0;

	$: allCurrentRoundFieldsFilledIn =
		gameData?.scores.every((score) => {
			const s = score[rounds[current_round_index]];
			return s !== null;
		}) || false;
	$: currentRoundScoresValid =
		gameData?.scores.every((score) => {
			const s = score[rounds[current_round_index]];
			return s === null || s % 5 === 0;
		}) || false;

	$: {
		console.log('completeButtonEnabled', allCurrentRoundFieldsFilledIn);
		console.log('currentRoundScoresValid', currentRoundScoresValid);
	}

	$: {
		console.log('current_round', current_round_index);
	}

	$: {
		console.log('gameData', gameData);
	}

	let gameData: GameWithPrevious | undefined;

	type GameWithPrevious = {
		id: number;
		scores: (PlayerAndScore & { previous: number | null | '-' })[];
	};

	onMount(async () => {
		const game = await $client.gameData(id);

		gameData = {
			id: game.id,
			scores: game.scores.map((score) => ({ previous: null, ...score }))
		};

		let current_round_set = new Set(
			gameData.scores.map(get_score).map((score) => {
				let index = score.findIndex((value) => {
					console.debug(value);
					return value[1] === null;
				});
				if (index === -1) {
					game_completed = true;
					return 0;
				} else {
					return index;
				}
			})
		);

		if (current_round_set.size !== 1) {
			throw new Error('Data is invalid');
		} else {
			let maybe_cri = [...current_round_set][0];
			if (maybe_cri === null) {
			}
			current_round_index = [...current_round_set][0];
		}
	});

	const sum_score = (acc: number, [_, curr]: [Round, number | null]) => {
		console.debug('curr', curr);
		return acc + (curr || 0);
	};

	function completeRound() {
		let scores = gameData!.scores.reduce((acc, score) => {
			return { [score.player_id]: score[rounds[current_round_index]]!, ...acc };
		}, {});

		completeRoundPromise = $client.completeRound(id, rounds[current_round_index], scores);
		current_round_index += 1;
	}

	let completeRoundPromise;

	function validator(
		node: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		},
		{ round, player_idx }: { round: Round; player_idx: number }
	) {
		// if (value) {
		// console.debug(value);
		if (/* value &&  */ gameData) {
			// 	const { player_idx, round } = value;
			const previous = gameData.scores[player_idx].previous;
			const newValueIsValid =
				node.currentTarget.value === '' || node.currentTarget.value === '-'
					? true
					: node.currentTarget.value.match(/^-?\d+$/)?.length === 1 || false;
			const newValue = newValueIsValid
				? node.currentTarget.value === '' || node.currentTarget.value === '-'
					? node.currentTarget.value
					: parseInt(node.currentTarget.value, 10)
				: previous;

			console.log('newValueIsValid', {
				currentValue: node.currentTarget.value,
				newValueIsValid,
				round,
				name: gameData.scores[player_idx].player_name,
				previous,
				newValue
			});
			if (!newValueIsValid) {
				node.currentTarget.value = previous?.toString() || '';
			}
			if (newValue === '-') {
				gameData.scores[player_idx][round] = 0;
			} else if (newValue !== '') {
				console.log('newValue', newValue);
				gameData.scores[player_idx][round] = newValue;
			}
			gameData.scores[player_idx].previous = newValue === '' ? null : newValue;
		}
		// }
	}

	const get_winner = (acc: typeof reduce_init, player_and_score: PlayerAndScore) => {
		const current: [string, number] = [
			player_and_score.player_name,
			get_score(player_and_score).reduce(sum_score, 0)
		];

		return current[1] > acc[1] ? current : acc;
	};

	const reduce_init: readonly [string, number] = ['', 0] as const;
</script>

{#if gameData}
	<table>
		<tr>
			<th scope="col">Round</th>
			{#each gameData.scores as player_and_score, index}
				<th>{player_and_score.player_name}</th>
			{/each}
		</tr>
		{#each rounds as round, index}
			<tr>
				<th scope="row">{round}</th>
				{#each gameData.scores as player_and_score, player_idx}
					<td>
						<!-- {#if player_and_score[round] === null} -->
						<input
							class="score-input"
							on:input={(node) => validator(node, { player_idx, round })}
							disabled={index !== current_round_index || game_completed}
							class:invalid-score={(player_and_score[round] || 0) % 5 !== 0}
							step="5"
						/>
						<!-- bind:value={player_and_score[round]} -->
						<!-- type="number" -->
						<!-- use:validator={{ player_idx, round }} -->
						<!-- {:else} -->
						<!-- <input value={player_and_score[round]} type="number" disabled /> -->
						<!-- {/if} -->
					</td>
				{/each}
				<td>
					{#if index === current_round_index && !game_completed}
						<button
							class="complete-button-base"
							on:click={completeRound}
							disabled={!(allCurrentRoundFieldsFilledIn && currentRoundScoresValid)}
							class:invalid-score={!currentRoundScoresValid}
							class:complete-button-disabled={!(
								allCurrentRoundFieldsFilledIn && currentRoundScoresValid
							)}
							class:complete-button-enabled={allCurrentRoundFieldsFilledIn &&
								currentRoundScoresValid}
						>
							<!-- use:clickOutside -->
							Complete Round
						</button>
					{/if}
				</td>
			</tr>
		{/each}

		<tr>
			<th scope="row">Total</th>
			{#each gameData.scores as player_and_score}
				<td>
					{get_score(player_and_score).reduce(sum_score, 0)}
				</td>
			{/each}
		</tr>
		{#if game_completed}
			<tr>
				<td colspan={gameData.scores.length + 1}>
					Completed! Winner is {gameData.scores.reduce(get_winner, reduce_init)}
				</td>
			</tr>
		{/if}
	</table>
{/if}

<style>
	.invalid-score {
		border-color: red;
		color: red;
	}

	.score-input {
		text-align: end;
		font-family: monospace;
		width: 3rem;
		border-top: none;
		border-left: none;
		border-right: none;
		border-bottom: 1px solid;
	}

	.complete-button-base {
		border-radius: 2px;
		border: 1px solid;
	}

	.complete-button-enabled {
		box-shadow: 0px 3px black;
	}

	.complete-button-enabled:active {
		transform: translateY(2px);
		box-shadow: 0px 1px black;
	}

	.complete-button-disabled {
		transform: translateY(3px);
		box-shadow: 0px 0px;
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
