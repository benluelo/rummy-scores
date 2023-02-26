<script lang="ts">
	import Chip, { Set as ChipSet, Text as ChipText } from '@smui/chips';
	import Button from '@smui/button';
	import { goto } from '$app/navigation';
	import CredentialsWrapper from '../../Components/EnsureLoggedIn.svelte';
	import { client } from '../../Components/client';
	import { onMount } from 'svelte';

	// export let client: Client;

	let id: number;
	let players: Player[] = [];
	let all_players: Player[] = [];

	$: {
		console.log(players);
	}

	onMount(async () => {
		all_players = await $client.getPlayers();
	});

	let new_game_clicked = false;
</script>

{#if new_game_clicked}
	{#await $client.newGame(players).then((newGameId) => goto(`/rummy/play-game/${newGameId}`))}
		Loading game...
	{/await}
{:else}
	{#if all_players}
		<select multiple bind:value={players}>
			{#each all_players as player}
				<option value={player.id}>{player.name}</option>
			{/each}
		</select>
	{/if}
	<Button
		disabled={players.length < 2}
		on:click={() => {
			new_game_clicked = true;
		}}
	>
		Play
	</Button>
{/if}

<!-- Selected Players:
	<ChipSet chips={[...players]} let:chip nonInteractive>
		<Chip {chip}>
			<ChipText>{chip}</ChipText>
		</Chip>
	</ChipSet> -->
