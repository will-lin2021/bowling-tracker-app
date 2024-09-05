// [Module] Nativescript
import { Http, HttpContent, HttpResponse, Observable, ObservableArray, fromObjectRecursive, fromObject} from '@nativescript/core';

// [Module] Project
import { GameInfo, Game, DateRecord, GameResponse, RecordResponse, ErrorResponse } from '../models';

const SERVER_URL: string = 'http://localhost:8000/api';

function processErrorContent(content?: HttpContent) {
  if (content === undefined) {
    return 'No content';
  }

  console.log(content.toString());
  console.log(content.toJSON() as ErrorResponse);

  return (content.toJSON() as ErrorResponse).message;
}

function toYYYYMMDD(date: Date) {
  return `${date.getUTCFullYear()}-${date.getUTCMonth() + 1}-${date.getUTCDate()}`;
}

class _GameService {
  private page: number = 1;
  private dateStore: ObservableArray<DateRecord> = new ObservableArray([]);

  lastUpdated: Date = new Date();

  async init() {
    try {
      await this.fetch();

      console.log('Data Initialization Complete!');
    } catch (e) {
      throw e;
    }
  }

  async fetch(): Promise<boolean> {
    try {
      const res = await Http.request({
        url: `${SERVER_URL}/games?page=${this.page}&limit=12`,
        method: 'GET',
      });

      if (res.statusCode != 200) {
        console.log(processErrorContent(res.content));

        return false;
      }

      let changed = false;

      (res.content?.toJSON() as RecordResponse).dates.forEach((date) => {
        date.date = new Date(date.date);

        this.dateStore.push(date);

        changed = true;
      })

      if (changed) {
        this.lastUpdated = new Date();
        return true;
      }

      return false;
    } catch (e) {
      throw e;
    }
  }

  async deleteGameById(gameId: string): Promise<boolean> {
    try {
      // Request server to delete game with given gameId
      const res = await Http.request({
        url: `${SERVER_URL}/games/${gameId}`,
        method: 'DELETE',
      });

      if (res.statusCode !== 200) {
        console.log(processErrorContent(res.content));

        return false;
      }

      // Find record with game with given Id
      const recordIdx = this.dateStore.findIndex((record) => record.games.find((game) => game.game_id === gameId) !== undefined);

      if (recordIdx === -1) {
        console.log('gameId not found');

        return false;
      }

      const record = this.dateStore.getItem(recordIdx);

      // Find game within the record with the given Id
      const gameIdx = record.games.findIndex((game) => game.game_id === gameId);

      // Remove game with given Id
      record.games.splice(gameIdx, 1);

      // Check if this is the last game and remove if it is
      if (record.games.length === 0) {
        this.dateStore.splice(recordIdx, 1);
      }

      this.lastUpdated = new Date();

      return true;
    } catch (e) {
      throw e;
    }
  }

  getDateStore(): ObservableArray<DateRecord> {
    return this.dateStore;
  }

  getRecordById(dateId: string): DateRecord {
    return this.dateStore.find((date) => (date).date_id === dateId);
  }

  async testAddGame() {
    const dates = [
      new Date("2022-01-01"),
      new Date("2023-01-01"),
      new Date("2024-01-01"),
      new Date("2024-06-06"),
      new Date("2024-12-31"),
    ];

    try {
      for (let i = 0; i < dates.length; i++) {
        const res = await Http.request({
          url: `${SERVER_URL}/games/`,
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          content: JSON.stringify({
            date: toYYYYMMDD(dates[i]),
            score_str: 'X X X X X X X X X XXX',
          })
        });

        if (res.statusCode !== 200) {
          console.log(processErrorContent(res.content));

          return;
        }
      }

    } catch (e) {
      throw e;
    }
  }

  test() {
    this.dateStore.getItem(0).date = new Date();

    console.log(this.dateStore);
  }
}

export const GameService = new _GameService();
