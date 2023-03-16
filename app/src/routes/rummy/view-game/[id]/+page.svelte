<script lang="ts">
	import Button from '@smui/button';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';

	import { rounds } from '../../../../Components/models';
	import { client } from '../../../../Components/client';

	// import type { Chart } from '@types/chart.js';
	// import { Chart } from '@types/chart.js';
	import Chart from 'chart.js/auto';
	import { get_score } from '../../../../Components/utils';

	let id: number = parseInt($page.params.id);

	let game_data: Game | undefined;

	let chart: Chart | undefined;

	onMount(async () => {
		game_data = await $client.gameData(id);

		console.log(game_data);

		let canvasElement = <HTMLCanvasElement | null>document.getElementById('myChart');

		console.log(canvasElement);

		function capitalize(string: string): string {
			return string.charAt(0).toUpperCase() + string.slice(1);
		}

		// @ts-ignore
		chart = new Chart(canvasElement!.getContext('2d')!, {
			type: 'line',
			options: {
				// interaction: {
				// 	mode: 'nearest',
				// 	axis: 'x',
				// 	intersect: false
				// },
				plugins: {
					tooltip: {
						callbacks: {
							label: (item) => {
								return capitalize(item.dataset.label!) + ': ' + item.formattedValue + ' points';
							},
							afterLabel: function (item) {
								return 'Score this round: ' + (<any>item.raw).round_score.toString() + ' points';
							},
							title: function (items) {
								return (
									capitalize(rounds[items[0].dataIndex]) +
									(rounds[items[0].dataIndex] === 'six' ? 'es' : 's')
								);
							}
						}
					}
				}
			},
			data: {
				labels: [...rounds],
				datasets: game_data.scores.map((score) => {
					let just_score = get_score(score);
					const color = `rgb(${rand255()}, ${rand255()}, ${rand255()})`;
					return {
						label: score.player_name,
						data: just_score.map(([_, round_score], i, arr) =>
							// could also add round_score to the reduction, both work
							{
								return {
									index: i,
									total_score: arr.slice(0, i + 1).reduce((acc, [_round, inner_round_score]) => {
										return acc + (inner_round_score || 0);
									}, 0),
									round_score: round_score
								};
							}
						),
						backgroundColor: color,
						borderColor: color,
						parsing: {
							xAxisKey: 'index',
							yAxisKey: 'total_score'
						}
					};
				})
			}
		});
	});

	function rand255() {
		const num = Math.random();
		return (num < 0.75 ? num : 0.75) * 255;
	}
</script>

<div class="chart-container">
	<canvas id="myChart" />
</div>

<!-- <style>
	/*
      The wrapper div needs to have an explicit width and height in CSS.
      It can also be a flexbox child or CSS grid element.
      The point being it needs dimensions since the <LayerCake> element will
      expand to fill it.
    */
	.chart-container {
		width: 200px;
		height: 200px;
	}
</style> -->
