/// <reference types="svelte" />

type Round =
    'ace' |
    'two' |
    'three' |
    'four' |
    'five' |
    'six' |
    'seven' |
    'eight' |
    'nine' |
    'ten' |
    'jack' |
    'queen' |
    'king';

type Player = {
    name: string,
    id: number
}
type Score = {
    [round in Round]: number | null
}

type PlayerAndScore = Score & {
    [key in keyof Player as `player_${key}`]: Player[key]
}

type Game = {
    scores: (PlayerAndScore)[],
    id: number,
    // date: Date,
}