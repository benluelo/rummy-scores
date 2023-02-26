import { browser } from "$app/env";
import { goto } from "$app/navigation";
import { derived, get, readable, writable } from "svelte/store";

class Client {
    constructor() { }

    // TODO: https://www.npmjs.com/package/esbuild-svelte

    public async login() {
        let auth_header = this.authHeader();
        let resp = await fetch(`${get(baseUrl)}/login`, {
            headers: {
                Authorization: auth_header,
            },
        });

        if (resp.status === 200) {
            return LoginStatus.Success;
        } else {
            return LoginStatus.Error;
        }
    }

    private authHeader() {
        return window.btoa(`${get(username)}:${get(password)}`);
    }

    public async getPlayers(): Promise<Player[]> {
        return await fetch(`${get(baseUrl)}/all_players`, {
            headers: {
                Authorization: this.authHeader(),
            },
        }).then(res => res.json())
    }

    public async newGame(players: Player[]): Promise<void> {
        return await fetch(`${get(baseUrl)}/new_game?players=${[...players].join(',')}`, {
            headers: {
                Authorization: this.authHeader(),
            },
        }).then(res => res.json())
    }

    public async newPlayer(name: String): Promise<Player> {
        return await fetch(`${get(baseUrl)}/new_player/${name}`, {
            method: 'POST',
            headers: {
                Authorization: this.authHeader(),
            },
        }).then(res => res.json())
    }

    public async gameData(id: number): Promise<Game> {
        return await fetch(`${get(baseUrl)}/game_data/${id}`, {
            headers: {
                Authorization: this.authHeader(),
            },
        }).then(res => res.json())
    }

    public async completeRound(game_id: number, round: Round, scores: { [x: number]: number }): Promise<void> {
        await fetch(`${get(baseUrl)}/round_completed`, {
            method: 'POST',
            headers: {
                "Content-Type": "application/json",
                Authorization: this.authHeader(),
            },
            body: JSON.stringify({
                game_id: game_id, round: round, scores: scores
            })
        });
    }
}

const USERNAME_KEY = 'USERNAME';
const PASSWORD_KEY = 'PASSWORD';

const storedUsername = (browser && localStorage.getItem(USERNAME_KEY)) || '';
const storedPassword = (browser && localStorage.getItem(PASSWORD_KEY)) || '';

export const username = writable(storedUsername);
export const password = writable(storedPassword);
export const baseUrl = readable("http://127.0.0.1:3001");

export const client = readable(new Client());

// export const clientStore = derived(
//     [username, password],
//     ([user, pass]) => new Client(baseUrl, user, pass)
// );

username.subscribe((newUsername) => {
    browser && localStorage.setItem(USERNAME_KEY, newUsername);
});
password.subscribe((newPassword) => {
    browser && localStorage.setItem(PASSWORD_KEY, newPassword);
});

export enum LoginStatus {
    Success,
    Error,
}