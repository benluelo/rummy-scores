<script lang="ts">
	import Chip, { Set as ChipSet, Text as ChipText } from '@smui/chips';
	import Button from '@smui/button';
	import { goto } from '$app/navigation';
	import CredentialsWrapper from '../../Components/EnsureLoggedIn.svelte';
	import { client } from '../../Components/client';
	import { onMount } from 'svelte';

	// export let client: Client;

	let id: number;
	let new_player_name = '';
	let all_players: Player[] = [];

	onMount(async () => {
		all_players = await $client.getPlayers();
	});

	$: index_of_existing_player = all_players.findIndex((element) => {
		return element.name.toLowerCase() === new_player_name.toLowerCase();
	});
	let new_player_promise: Promise<Player> | null;
</script>

{#if all_players}
	{#if new_player_promise}
		{#await new_player_promise then new_player}
			Successfully created new player {new_player.name}!
			<br />
		{/await}
	{/if}
	<input type="text" bind:value={new_player_name} />
	<Button
		disabled={index_of_existing_player !== -1}
		on:click={() => {
			new_player_promise = $client.newPlayer(new_player_name);
			new_player_name = '';
		}}
	>
		Create new player
	</Button>
	<br />
	{#if index_of_existing_player !== -1}
		Player {all_players[index_of_existing_player].name} already exists!
		<br />
		Names are case-insensitive.
	{/if}
{/if}

<!-- Selected Players:
	<ChipSet chips={[...players]} let:chip nonInteractive>
		<Chip {chip}>
			<ChipText>{chip}</ChipText>
		</Chip>
	</ChipSet> -->
