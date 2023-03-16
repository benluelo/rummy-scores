<!--
  @component
  Generates an SVG multi-series line chart. It expects your data to be an array of objects, each with a `values` key that is an array of data objects.
 -->
<script lang="ts">
	import { getContext } from 'svelte';
	import { rounds } from '../models';

	const { data, xGet, yGet, zGet } = getContext('LayerCake');

	console.log('data', $data);

	$: path = (
		values: {
			round: Round;
			score: number | null;
		}[]
	) => {
		return (
			'M' +
			values
				.map((value) => {
					console.log('value', value);
					console.log('$xGet', $xGet, $xGet(rounds.indexOf(value.round)));
					console.log('$yGet', $yGet, $yGet(value));

					return $xGet(rounds.indexOf(value.round)) + ',' + $yGet(value.score);
				})
				.join('L')
		);
	};
</script>

<g class="line-group">
	{#each $data as group}
		<path class="path-line" d={path(group.values)} stroke={$zGet(group)} />
	{/each}
</g>

<style>
	.path-line {
		fill: none;
		stroke-linejoin: round;
		stroke-linecap: round;
		stroke-width: 3px;
	}
</style>
