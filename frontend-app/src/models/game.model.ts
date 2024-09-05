import { ObservableArray } from "@nativescript/core";

export interface GameInfo {
  game_id: string,
  game_no: number,
  score_str: string,
}

export interface Game {
  date_id: string,
  date: Date,
  game_id: string,
  game_no: number,
  score_str: string,
}

export interface DateRecord {
  date_id: string,
  date: Date,
  games: ObservableArray<GameInfo>,
}
